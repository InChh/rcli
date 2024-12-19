use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value = "16", help = "Password length")]
    pub length: u8,
    #[arg(
        short,
        long,
        default_value = "true",
        help = "Include lowercase letters"
    )]
    pub lowercase: bool,
    #[arg(
        short,
        long,
        default_value = "true",
        help = "Include uppercase letters"
    )]
    pub uppercase: bool,
    #[arg(short, long, default_value = "true", help = "Include numbers")]
    pub numbers: bool,
    #[arg(
        short,
        long,
        default_value = "true",
        help = "Include special characters"
    )]
    pub special: bool,
}
