use crate::process::jwt::{process_jwt_sign, process_jwt_verify};
use crate::CmdExecutor;
use chrono::Local;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    Sign(JwtSignOpts),
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    pub aud: String,
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub exp: String,
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
    #[arg(short, long)]
    pub key: String,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let jwt = process_jwt_sign(&self.aud, &self.sub, &self.exp, &self.key)?;
        println!("{}", jwt);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        match process_jwt_verify(&self.token, &self.key) {
            Ok(claim) => {
                println!(
                    "Token is valid, sub: {}, aud: {}, iss: {}, exp: {}",
                    claim.sub,
                    claim.aud,
                    chrono::DateTime::from_timestamp(claim.iat as i64, 0)
                        .unwrap()
                        .with_timezone(&Local),
                    chrono::DateTime::from_timestamp(claim.exp as i64, 0)
                        .unwrap()
                        .with_timezone(&Local)
                )
            }
            Err(_) => {
                println!("Token is invalid")
            }
        }
        Ok(())
    }
}
