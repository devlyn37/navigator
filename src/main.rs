use anyhow::Result;
use clap::{arg, Command, Parser};
use ethers::{abi::Token, prelude::*};
mod decode;
mod validate;
/// Search for a pattern in a file and display th elines that contain it.
#[derive(Parser)]
struct Cli {
    contract_address: String,
    chain: String,
    data: String,
    etherscan_key: String,
}

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
            let (chain, contract_address, key, data) =
                validate::validate_and_format_input(sub_matches).expect("ah");
            let client = Client::new(chain, key).unwrap();
            let abi = client
                .contract_abi(contract_address)
                .await
                .expect("Could not fetch the abi for the provided contract address");

            let name: String;
            let args: Vec<Token>;

            match sub_matches
                .get_one::<String>("kind")
                .map(|s| s.as_str())
                .expect("error parsing kind")
            {
                "error" => {
                    (name, args) = decode::error(abi, data).expect("error parsing error data");
                }
                "function" => {
                    (name, args) =
                        decode::function(abi, data).expect("error parsing function data");
                }
                _ => unreachable!(),
            }

            println!("Error name: {}", name);
            println!("Args: {:?}", args);
            Ok(())
        }
        _ => unreachable!(),
    }
}
