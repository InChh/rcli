use crate::cli::text::TextSignFormat;
use crate::process::gen_pass::generate_password;
use crate::utils::get_reader;
use anyhow::Result;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::io::Read;

pub trait TextSigner {
    fn sign(&self, reader: impl Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyGenerator {
    fn generate_key(&self) -> Result<Key>;
}

pub enum Key {
    Symmetric { key: Vec<u8> },
    Asymmetric { public: Vec<u8>, secret: Vec<u8> },
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into()?;
        Ok(Self { key })
    }

    pub fn from_file(key_file: &str) -> Result<Self> {
        let key = std::fs::read(key_file)?;
        Self::try_new(&key)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, mut reader: impl Read) -> Result<Vec<u8>> {
        let mut hasher = blake3::Hasher::new_keyed(&self.key);
        let mut buffer = [0; 1024];
        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        let hash = hasher.finalize();
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut hasher = blake3::Hasher::new_keyed(&self.key);
        let mut buffer = [0; 1024];
        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        let hash = hasher.finalize();
        Ok(hash.as_bytes() == sig)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        Ok(Self {
            key: SigningKey::from_bytes(key.try_into()?),
        })
    }

    pub fn from_file(key_file: &str) -> Result<Self> {
        let key = std::fs::read(key_file)?;
        Self::try_new(&key)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, mut reader: impl Read) -> Result<Vec<u8>> {
        let msg = {
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;
            buffer
        };
        let sig = self.key.sign(&msg);
        Ok(sig.to_bytes().to_vec())
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        Ok(Self {
            key: VerifyingKey::from_bytes(key.try_into()?)?,
        })
    }
    pub fn from_file(key_file: &str) -> Result<Self> {
        let key = std::fs::read(key_file)?;
        Self::try_new(&key)
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let msg = {
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;
            buffer
        };
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&msg, &sig).is_ok())
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::from_file(key)?;
            signer.sign(reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::from_file(key)?;
            signer.sign(reader)?
        }
    };

    Ok(BASE64_URL_SAFE_NO_PAD.encode(&signed))
}

pub fn process_verify(input: &str, key: &str, sig: &str, format: TextSignFormat) -> Result<bool> {
    let reader = get_reader(input)?;
    let sig = BASE64_URL_SAFE_NO_PAD.decode(sig.as_bytes())?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::from_file(key)?;
            verifier.verify(reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::from_file(key)?;
            verifier.verify(reader, &sig)?
        }
    };

    Ok(verified)
}

pub struct CommonKeyGenerator {
    key_length: usize,
    key_format: TextSignFormat,
}

impl CommonKeyGenerator {
    pub fn new(key_length: usize, key_format: TextSignFormat) -> Self {
        Self {
            key_length,
            key_format,
        }
    }
}

impl KeyGenerator for CommonKeyGenerator {
    fn generate_key(&self) -> Result<Key> {
        let key = generate_password(self.key_length as u8, true, true, true, true)?;
        match self.key_format {
            TextSignFormat::Blake3 => Ok(Key::Symmetric {
                key: key.into_bytes(),
            }),
            TextSignFormat::Ed25519 => {
                let mut rng = OsRng;
                let key = SigningKey::generate(&mut rng);
                let public = key.verifying_key().to_bytes().to_vec();
                Ok(Key::Asymmetric {
                    public,
                    secret: key.to_bytes().to_vec(),
                })
            }
        }
    }
}

pub fn generate_key(format: TextSignFormat) -> Result<Key> {
    let key_length = match format {
        TextSignFormat::Blake3 => blake3::KEY_LEN,
        TextSignFormat::Ed25519 => ed25519_dalek::SECRET_KEY_LENGTH,
    };
    let generator = CommonKeyGenerator::new(key_length, format);

    generator.generate_key()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_blake3_sign_verify() {
        let signer = Blake3::from_file("fixtures/blake3.key").unwrap();
        let verifier = Blake3::from_file("fixtures/blake3.key").unwrap();

        let msg = b"hello world";
        let sig = signer.sign(Cursor::new(msg)).unwrap();
        assert!(verifier.verify(Cursor::new(msg), &sig).unwrap());
    }

    #[test]
    fn test_ed25519_sign_verify() {
        let signer = Ed25519Signer::from_file("fixtures/secret.key").unwrap();
        let verifier = Ed25519Verifier::from_file("fixtures/public.key").unwrap();

        let msg = b"hello world";
        let sig = signer.sign(Cursor::new(msg)).unwrap();
        assert!(verifier.verify(Cursor::new(msg), &sig).unwrap());
    }

    #[test]
    fn test_key_generator() {
        let generator = CommonKeyGenerator::new(32, TextSignFormat::Blake3);
        let key = generator.generate_key().unwrap();
        match key {
            Key::Symmetric { key } => {
                assert_eq!(key.len(), 32);
            }
            _ => panic!("Invalid key type"),
        }

        let generator = CommonKeyGenerator::new(32, TextSignFormat::Ed25519);
        let key = generator.generate_key().unwrap();
        match key {
            Key::Asymmetric { public, secret } => {
                assert_eq!(public.len(), 32);
                assert_eq!(secret.len(), 32);
            }
            _ => panic!("Invalid key type"),
        }
    }
}
