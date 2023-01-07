use std::{str::FromStr};

use anyhow::Result;
use clap::Parser;
use ethers::{prelude::*, utils::hex, abi::Token};
/// Search for a pattern in a file and display th elines that contain it.
#[derive(Parser)]
struct Cli {
    contract_address: String,
		chain: String,
		error: String,
		etherscan_key: String,
}

// 0xd2ade556
// 0x98AA442ceFCAF0A7277D10889d07d04E90B37eA5

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
		let args = Cli::parse();
		println!("contract address: {}, chain: {}, error: {}, key: {}", args.contract_address, args.chain, args.error, args.etherscan_key);
		let address: Address = args.contract_address.parse().expect("Provided contract address invalid");
		let chain = Chain::from_str(&args.chain).expect("Provided chain name invalid");
		let client = Client::new(chain, args.etherscan_key).unwrap();

		println!("Fetching on abi for contract on {} on chain {}", args.contract_address, args.chain);
    let abi = client
        .contract_abi(address).await.expect("Could not fetch the abi for the provided contract address");
		let (error_name, args) = parse_error(abi, &args.error).expect("Could not decode error with provided information");

		println!("Error name: {}", error_name);
		println!("Args: {:?}", args);

    Ok(())
}

fn parse_error(contract: ethers::abi::Contract, hex: &str) -> Result<(String, Vec<Token>), String> {
	for errors in contract.errors.into_values() {
			let error = &errors[0];
			let encoded_signature = hex::encode(error.signature().as_bytes());
			if encoded_signature.contains(hex.trim_start_matches("0x")) {
				let decoded = error.decode(hex.as_bytes());
				match decoded {
						Ok(result) => return Ok((error.name.to_owned(), result)),
						Err(_) => return Err("Could not decode data".to_string()),
				}
			}
	}

	return Err("No errors within the contract mapped to the data provided".to_string());
}
