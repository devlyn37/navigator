use anyhow::{Context, Result};
use clap::{arg, Command};
use ethers::{abi::{Token, Contract}, prelude::{Client}};
mod decode;
mod validate;

// function test

// contract address:
// 0x1d9317911cf1003b42a965574c29f18a87a2858c
// data:
// 0x0209c6b7000000000000000000000000292f9d08efcf1a3a988959190d44f48a53577f100000000000000000000000000000000000000000000000000000000000000001

// error test

// goerli
// 0xd2ade556
// 0x98AA442ceFCAF0A7277D10889d07d04E90B37eA5

fn cli() -> Command {
    Command::new("navigator")
        .about("Some tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("decode")
                .about("decodes hex ethereum call data")
                .arg_required_else_help(true)
                .arg(arg!(<CHAIN> "The remote to target"))
                .arg(arg!(<CONTRACT> "contract address for the target"))
                .arg(arg!(<ETHERSCAN_KEY> "api key for etherscan"))
                .arg(arg!(<DATA> "the data to decode"))
                .arg(
                    arg!(--kind <KIND>)
                        .value_parser(["function", "error"])
                        .num_args(1)
                        .required(true)
                        .require_equals(true),
                ),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("decode", sub_matches)) => {
            let (chain, address, key, data) = validate::decode_command(sub_matches).with_context(|| "error with input")?;
            let client = Client::new(chain, key).with_context(|| "error connecting to etherscan")?;
            let abi = client.contract_abi(address).await.with_context(|| "error fetching abi from etherscan")?;

						let data_type = sub_matches
						.get_one::<String>("kind")
						.map(|s| s.as_str())
						.with_context(|| "error parsing data type")?;
						
						let decoding_function: fn(Contract, Vec<u8>) -> Option<(std::string::String, Vec<Token>)>;
            match data_type
            {
                "error" => decoding_function = decode::error,
                "function" => decoding_function = decode::function,
                _ => unreachable!(),
            }

						let (name, args) = decoding_function(abi, data).with_context(|| format!("error decoding data"))?;
            println!("{} name: {}, arguments: {:?}", data_type, name, args);
            Ok(())
        }
        _ => unreachable!(),
    }
}
