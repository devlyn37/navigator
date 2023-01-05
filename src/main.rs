use core::panic;
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
}

// 0xd2ade556
// 0x98AA442ceFCAF0A7277D10889d07d04E90B37eA5

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
		let args = Cli::parse();
		println!("contract address: {}, chain: {}, error: {}", args.contract_address, args.chain, args.error);
		let api_key = "K14P3TW12QCI2VDR3YIDY7XA9Y5XP2D232";
		let client = Client::new(Chain::from_str(&args.chain).unwrap(), api_key).unwrap();

		println!("Fetching on abi for contract on {} on chain {}", args.contract_address, args.chain);
    let abi = client
        .contract_abi(args.contract_address.parse().unwrap()).await.expect("ahh");

		let (error_name, args) = parse_error(abi, &args.error);
		println!("Error name: {}", error_name);
		println!("Args: {:?}", args);
    Ok(())
}

fn parse_error(contract: ethers::abi::Contract, hex: &str) -> (String, Vec<Token>) {
	for (_, list) in contract.errors.into_iter() {
			let error = &list[0];
			let error_signature = hex::encode(error.signature().as_bytes());
			if error_signature.contains(hex.trim_start_matches("0x")) {
				match error.decode(hex.as_bytes()) {
						Ok(result) => return (error.name.to_owned(), result),
						Err(_) => panic!("ahhh"),
				} 
			}
	}

	panic!("ahh");
}
