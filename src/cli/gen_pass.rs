use crate::process::gen_pass::generate_password;
use crate::CmdExecutor;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value = "16", help = "Password length")]
    pub length: u8,
    #[arg(long, default_value = "true", help = "Include lowercase letters")]
    pub lowercase: bool,
    #[arg(long, default_value = "true", help = "Include uppercase letters")]
    pub uppercase: bool,
    #[arg(long, default_value = "true", help = "Include numbers")]
    pub numbers: bool,
    #[arg(long, default_value = "true", help = "Include special characters")]
    pub special: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let pass = generate_password(
            self.length,
            self.lowercase,
            self.uppercase,
            self.numbers,
            self.special,
        )?;
        println!("{}", pass);
        let estimate = zxcvbn::zxcvbn(&pass, &[]);
        println!("Password strength: {}", estimate.score());
        Ok(())
    }
}
