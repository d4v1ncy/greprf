// use crate::errors::Error;
use clap::{Args, Parser, Subcommand};
use clap::builder::PossibleValue;
use clap::ValueEnum;
use serde::{self, Deserialize, Serialize};


#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OutputFormat {
    Raw,
    Hex,
    Base64,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OutputFormat::Raw =>     write!(f, "raw"),
            OutputFormat::Hex =>      write!(f, "hex"),
            OutputFormat::Base64 => write!(f, "base64"),
        }
    }
}

impl ValueEnum for OutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            OutputFormat::Raw,
            OutputFormat::Hex,
            OutputFormat::Base64,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match &self {
            OutputFormat::Raw => {
                Some(PossibleValue::new("raw"))
            }
            OutputFormat::Hex => {
                Some(PossibleValue::new("hex"))
            }
            OutputFormat::Base64 => {
                Some(PossibleValue::new("base64"))
            }
        }
    }
    fn from_str(input: &str, ignore_case: bool) -> Result<OutputFormat, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        let input = input.trim();

        match input {
            "raw" => Ok(OutputFormat::Raw),
            "hex" => Ok(OutputFormat::Hex),
            "base64" => Ok(OutputFormat::Base64),
            otherwise => Err(otherwise.to_string()),
        }
    }
}

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
    #[command(about = "ascii")]
    As(SecretAsciiParams),

    #[command(about = "utf8")]
    U8(SecretUtf8Params),

    #[command(about = "alphabet")]
    Ab(SecretAlphabetParams),

    #[command(about = "alphanumeric")]
    Al(SecretAlphanumericParams),

    #[command(about = "numbers")]
    Nu(SecretNumbersParams),

    #[command(about = "bytes")]
    B(SecretBytesParams),
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
#[derive(Args, Debug)]
pub struct SecretBytesParams {
    #[arg(default_value_t = 37)]
    pub length: usize,

    #[arg(short, long, default_value_t = OutputFormat::Base64)]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub linebreak: bool,
}
