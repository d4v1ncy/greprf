#![allow(unused)]
use rand::prelude::*;
use rand::distributions::{Alphanumeric, Uniform, Standard};

use clap::Parser;
use greprf::errors::Error;
use greprf::clap::{Cli, Commands};
use greprf::clap::{GenerateCommands, GenerateSecretCommands};
use greprf::sutf8::UString;


const ASCII_PRINTABLE: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const ASCII_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ASCII_DIGITS: &'static str = "0123456789";
const ASCII_ALPHANUM: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";



fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let mut rng = thread_rng();

    match cli.commands {
        Commands::G(sub) => match sub.commands {
            GenerateCommands::S(sub) => match sub.commands {
                GenerateSecretCommands::As(params) => {
                    let mut result = Vec::<u8>::new();
                    for c in 0..params.length {
                        let mut pool = ASCII_PRINTABLE.as_bytes().to_vec();
                        pool.shuffle(&mut rng);
                        result.push(pool[c % pool.len()]);
                    }
                    println!("{}", String::from_utf8_lossy(&result));
                },
                GenerateSecretCommands::U8(params) => {
                    let mut result = String::new();
                    let secret = (&mut rng).sample_iter(Standard).take(params.length)
                        .collect::<Vec<u8>>();
                    let ussr = UString::new(&secret);
                    println!("{}", ussr.soft_word());
                    println!("{}", ussr.ascii());
                    println!("{}", hex::encode(ussr.garbage()));
                },

                GenerateSecretCommands::Ab(params) => {
                    let mut result = Vec::<u8>::new();
                    for c in 0..params.length {
                        let mut pool = ASCII_LETTERS.as_bytes().to_vec();
                        pool.shuffle(&mut rng);
                        result.push(pool[c % pool.len()]);
                    }
                    println!("{}", String::from_utf8_lossy(&result));
                },
                GenerateSecretCommands::Al(params) => {
                    let mut result = Vec::<u8>::new();
                    for c in 0..params.length {
                        let mut pool = ASCII_ALPHANUM.as_bytes().to_vec();
                        pool.shuffle(&mut rng);
                        result.push(pool[c % pool.len()]);
                    }
                    println!("{}", String::from_utf8_lossy(&result));
                },
                GenerateSecretCommands::Nu(params) => {
                    let mut result = Vec::<u8>::new();
                    for c in 0..params.length {
                        let mut pool = ASCII_DIGITS.as_bytes().to_vec();
                        pool.shuffle(&mut rng);
                        result.push(pool[c % pool.len()]);
                    }
                    println!("{}", String::from_utf8_lossy(&result));
                },
            },
        }
    }
    Ok(())
}
