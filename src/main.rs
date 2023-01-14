use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use ethers::{
    abi::Token,
    prelude::*,
    utils::{hex, keccak256},
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!(
        "contract address: {}, chain: {}, data: {}, key: {}",
        args.contract_address, args.chain, args.data, args.etherscan_key
    );

    let data = hex::decode(args.data.trim_start_matches("0x")).expect("data provided is not hex");
    let address: Address = args
        .contract_address
        .parse()
        .expect("Provided contract address invalid");
    let chain = Chain::from_str(&args.chain).expect("Provided chain name invalid");
    let client = Client::new(chain, args.etherscan_key).unwrap();

    println!(
        "Here's the hash of the error message {}",
        hex::encode(keccak256("SeasonPassNFT: Not enough ETH sent"))
    );

    println!(
        "Fetching on abi for contract on {} on chain {}",
        args.contract_address, args.chain
    );
    let abi = client
        .contract_abi(address)
        .await
        .expect("Could not fetch the abi for the provided contract address");
    let (name, args) =
        parse_error(abi, data).expect("Could not decode error with provided information");
    // let (name, args) =
    //     parse_function(abi, data).expect("Could not decode error with provided information");

    println!("Error name: {}", name);
    println!("Args: {:?}", args);

    Ok(())
}

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
