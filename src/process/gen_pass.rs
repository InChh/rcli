use rand::{seq::SliceRandom, thread_rng};

const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"!@#$%&";

pub fn generate_password(
    length: u8,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    special: bool,
) -> anyhow::Result<String> {
    let mut rng = thread_rng();
    let mut chars = Vec::new();
    let mut has_valid_option = false;
    let mut password = Vec::with_capacity(length as usize);

    if lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
        has_valid_option = true;
    }

    if uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
        has_valid_option = true;
    }

    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).unwrap());
        has_valid_option = true;
    }

    if special {
        chars.extend_from_slice(SPECIAL);
        password.push(*SPECIAL.choose(&mut rng).unwrap());
        has_valid_option = true;
    }

    // If no options selected, default to lowercase
    if !has_valid_option {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }

    for _ in 0..length - password.len() as u8 {
        password.push(*chars.choose(&mut rng).unwrap());
    }

    password.shuffle(&mut rng);

    Ok(String::from_utf8(password)?)
}
