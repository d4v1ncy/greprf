// use crate::errors::Error;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "generate")]
    G(GenerateParams),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct GenerateParams {
    #[command(subcommand)]
    pub commands: GenerateCommands,
}

#[derive(Debug, Subcommand)]
pub enum GenerateCommands {
    S(GenerateSecretParams),
}
#[derive(Args, Debug)]
pub struct GenerateSecretParams {
    #[command(subcommand)]
    pub commands: GenerateSecretCommands,
}

#[derive(Debug, Subcommand)]
pub enum GenerateSecretCommands {
    #[command(about = "ascii secret")]
    As(SecretAsciiParams),

    #[command(about = "utf8 secret")]
    U8(SecretUtf8Params),

    #[command(about = "alphabet secret")]
    Ab(SecretAlphabetParams),

    #[command(about = "alphanumeric secret")]
    Al(SecretAlphanumericParams),

    #[command(about = "numbers secret")]
    Nu(SecretNumbersParams),
}

#[derive(Args, Debug)]
pub struct SecretAsciiParams {
    #[arg(default_value_t = 89)]
    pub length: usize,
}
#[derive(Args, Debug)]
pub struct SecretUtf8Params {
    #[arg(default_value_t = 89)]
    pub length: usize,
}
#[derive(Args, Debug)]
pub struct SecretAlphabetParams {
    #[arg(default_value_t = 59)]
    pub length: usize,
}
#[derive(Args, Debug)]
pub struct SecretAlphanumericParams {
    #[arg(default_value_t = 59)]
    pub length: usize,
}
#[derive(Args, Debug)]
pub struct SecretNumbersParams {
    #[arg(default_value_t = 79)]
    pub length: usize,
}
