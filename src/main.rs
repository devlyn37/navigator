use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use ethers::{abi::Token, prelude::*, utils::{hex, keccak256}};
/// Search for a pattern in a file and display th elines that contain it.
#[derive(Parser)]
struct Cli {
    contract_address: String,
    chain: String,
    error: String,
    etherscan_key: String,
}


// contract address: 
// 0x1d9317911cf1003b42a965574c29f18a87a2858c
// data:
// 0x0209c6b7000000000000000000000000e3ce7d40801067f3057eb5b7680ddffb9c2019c80000000000000000000000000000000000000000000000000000000000000001

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!(
        "contract address: {}, chain: {}, error: {}, key: {}",
        args.contract_address, args.chain, args.error, args.etherscan_key
    );
    let address: Address = args
        .contract_address
        .parse()
        .expect("Provided contract address invalid");
    let chain = Chain::from_str(&args.chain).expect("Provided chain name invalid");
    let client = Client::new(chain, args.etherscan_key).unwrap();

		println!("Here's the hash of the error message {}", hex::encode(keccak256("SeasonPassNFT: Not enough ETH sent")));

    println!(
        "Fetching on abi for contract on {} on chain {}",
        args.contract_address, args.chain
    );
    let abi = client
        .contract_abi(address)
        .await
        .expect("Could not fetch the abi for the provided contract address");
    // let (error_name, args) =
    //     parse_error(abi, &args.error).expect("Could not decode error with provided information");
		let (function_name, args) =
        parse_function(abi, &args.error).expect("Could not decode error with provided information");

    println!("Error name: {}", function_name);
    println!("Args: {:?}", args);

    Ok(())
}



fn parse_error(contract: ethers::abi::Contract, hex: &str) -> Option<(String, Vec<Token>)> {
    let found = contract
        .errors
        .into_values()
        .filter_map(|x| x.into_iter().nth(0))
        .find(|error| {
            let encoded_signature = hex::encode(error.signature());
            return encoded_signature.contains(hex.trim_start_matches("0x"));
        })?;

    let decoded = found.decode(hex.as_bytes()).ok()?;
    Some((found.name.to_owned(), decoded))
}

fn parse_function(contract: ethers::abi::Contract, hex: &str) -> Option<(String, Vec<Token>)> {
	    let found = contract
        .functions
        .into_values()
        .filter_map(|x| x.into_iter().nth(0))
        .find(|function| {
						println!("{:?}", function.signature());
						let function_selector = format!("{}{}", "0x", hex::encode(function.short_signature()));
						println!("{:?}", function_selector);
            return hex.contains(function_selector.as_str());
        })?;


				let decoded = found.decode_input(hex.as_bytes()).ok()?;
    		Some((found.name.to_owned(), decoded))
}
