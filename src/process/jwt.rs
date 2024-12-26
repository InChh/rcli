use jsonwebtoken::{DecodingKey, EncodingKey, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub(crate) aud: String,
    pub(crate) sub: String,
    pub(crate) iat: u64,
    pub(crate) exp: u64,
}

pub fn process_jwt_sign(aud: &str, sub: &str, exp: &str, key: &str) -> anyhow::Result<String> {
    let current_timestamp = jsonwebtoken::get_current_timestamp();
    let claims = JwtClaims {
        aud: aud.to_string(),
        sub: sub.to_string(),
        iat: current_timestamp,
        exp: current_timestamp + parse_exp_duration(exp)?,
    };

    let jwt = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )?;

    Ok(jwt)
}

/// Parse the expiration seconds from the input string
/// for example, "14d" means 14 days, "1h" means 1 hour, "30m" means 30 minutes, "3600" means 3600 seconds
///
/// The expiration duration is returned in seconds
fn parse_exp_duration(exp: &str) -> anyhow::Result<u64> {
    let exp = exp.to_string();
    let exp = exp.trim();
    let exp = exp.to_lowercase();
    let exp = exp.chars().collect::<Vec<char>>();
    let mut num = String::new();
    let mut unit = String::new();
    for c in exp {
        if c.is_ascii_digit() {
            num.push(c);
        } else {
            unit.push(c);
        }
    }
    let num = num.parse::<u64>()?;
    let exp = match unit.as_str() {
        "d" => num * 24 * 60 * 60,
        "h" => num * 60 * 60,
        "m" => num * 60,
        "" => num,
        _ => return Err(anyhow::anyhow!("Invalid expiration time format")),
    };
    Ok(exp)
}

pub fn process_jwt_verify(token: &str, key: &str) -> anyhow::Result<JwtClaims> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.validate_aud = false;
    let TokenData { claims, .. } = jsonwebtoken::decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &validation,
    )?;
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_jwt_sign_verify_should_work() {
        let aud = "device1";
        let sub = "acme";
        let iss = jsonwebtoken::get_current_timestamp();
        let exp = "14d";
        let key = "key";
        let jwt = process_jwt_sign(aud, sub, exp, key).unwrap();
        let claims = process_jwt_verify(&jwt, key).unwrap();
        assert_eq!(claims.aud, aud);
        assert_eq!(claims.sub, sub);
        assert_eq!(claims.exp, iss + parse_exp_duration(exp).unwrap());
    }

    #[test]
    fn test_process_jwt_verify_should_fail() {
        let aud = "device1";
        let sub = "acme";
        let exp = "14d";
        let key = "key";
        let jwt = process_jwt_sign(aud, sub, exp, key).unwrap();
        assert!(process_jwt_verify(&jwt, "wrong_key").is_err());
        assert!(process_jwt_verify("wrong_token", key).is_err());
    }

    #[test]
    fn test_parse_exp_duration_should_work() {
        assert_eq!(parse_exp_duration("14d").unwrap(), 14 * 24 * 60 * 60);
        assert_eq!(parse_exp_duration("1h").unwrap(), 60 * 60);
        assert_eq!(parse_exp_duration("30m").unwrap(), 30 * 60);
        assert_eq!(parse_exp_duration("3600").unwrap(), 3600);
    }

    #[test]
    fn test_parse_exp_duration_should_fail() {
        assert!(parse_exp_duration("14x").is_err());
        assert!(parse_exp_duration("x14").is_err());
        assert!(parse_exp_duration("abc").is_err());
        assert!(parse_exp_duration("12x12d").is_err());
    }
}
