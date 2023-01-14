use std::{ffi::OsString, str::FromStr};

use anyhow::Result;
use clap::{arg, ArgMatches, Command, Parser};
use ethers::{
    abi::Token,
    prelude::*,
    utils::{hex},
};
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

fn parse_error(contract: ethers::abi::Contract, data: Vec<u8>) -> Option<(String, Vec<Token>)> {
    let found = contract
        .errors
        .into_values()
        .filter_map(|x| x.into_iter().nth(0))
        .find(|error| {
            let error_signature = &error.signature().to_fixed_bytes()[0..4];
            let data_signature = &data[0..4];
            return error_signature == data_signature;
        })?;

    let params = &data[4..];
    let decoded = found.decode(params).ok()?;
    Some((found.name.to_owned(), decoded))
}

fn parse_function(contract: ethers::abi::Contract, data: Vec<u8>) -> Option<(String, Vec<Token>)> {
    let found = contract
        .functions
        .into_values()
        .filter_map(|x| x.into_iter().nth(0))
        .find(|function| {
            let signature = &data[0..4];
            return signature == function.short_signature();
        })?;

    let params = &data[4..];
    let decoded = found.decode_input(params).ok()?;
    Some((found.name.to_owned(), decoded))
}

fn cli() -> Command {
    Command::new("git")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("decode")
                .about("decodes hex ethereum call data")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("error")
                        .about("decodes solidity custom error data")
                        .arg_required_else_help(true)
                        .arg(arg!(<CHAIN> "The remote to target"))
                        .arg(arg!(<CONTRACT> "contract address for the target"))
                        .arg(arg!(<ETHERSCAN_KEY> "api key for etherscan"))
                        .arg(arg!(<DATA> "the data to decode")),
                )
                .subcommand(
                    Command::new("function")
                        .about("decodes solidity function call data")
                        .arg_required_else_help(true)
                        .arg(arg!(<CHAIN> "The remote to target"))
                        .arg(arg!(<CONTRACT> "contract address for the target"))
                        .arg(arg!(<ETHERSCAN_KEY> "api key for etherscan"))
                        .arg(arg!(<DATA> "the data to decode")),
                ),
        )
}

fn validate_and_format_input(matches: &ArgMatches) -> (Chain, Address, String, Vec<u8>) {
    let chain_input = matches.get_one::<String>("CHAIN").expect("required");
    let contract_input = matches.get_one::<String>("CONTRACT").expect("required");
    let key_input = matches
        .get_one::<String>("ETHERSCAN_KEY")
        .expect("required");
    let data_input = matches.get_one::<String>("DATA").expect("required");

    (
        Chain::from_str(chain_input).expect("Provided chain name invalid"),
        contract_input
            .parse()
            .expect("Provided contract address invalid"),
        key_input.to_owned(),
        hex::decode(data_input.trim_start_matches("0x")).expect("data provided is not hex"),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("decode", sub_matches)) => {
            let decode_command = sub_matches.subcommand().unwrap_or(("decode", sub_matches));
            match decode_command {
                ("error", sub_matches) => {
                    let (chain, contract_address, key, data) =
                        validate_and_format_input(sub_matches);
                    let client = Client::new(chain, key).unwrap();
                    let abi = client
                        .contract_abi(contract_address)
                        .await
                        .expect("Could not fetch the abi for the provided contract address");
                    let (name, args) = parse_error(abi, data).expect("ahhl");
                    println!("Error name: {}", name);
                    println!("Args: {:?}", args);
                    return Ok(());
                }
                (ext, sub_matches) => {
                    let args = sub_matches
                        .get_many::<OsString>("")
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>();
                    println!("Calling out to {:?} with {:?}", ext, args);
                    return Ok(());
                }
                _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
            };
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
            Ok(())
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
