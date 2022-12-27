use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use ethers::{etherscan::Client, types::Chain};
/// Search for a pattern in a file and display th elines that contain it.
#[derive(Parser)]
struct Cli {
    contract_address: String,
		chain: String
}
// 0xd2ade556
// 0x98AA442ceFCAF0A7277D10889d07d04E90B37eA5

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
		let args = Cli::parse();
		println!("contract address: {:?}", args.contract_address);
		println!("chain: {:?}", args.chain);
		let api_key = "K14P3TW12QCI2VDR3YIDY7XA9Y5XP2D232";
		let client = Client::new(Chain::from_str(&args.chain).unwrap(), api_key).unwrap();

		println!("Fetching on abi for contract on {:?} on chain {:?}", args.contract_address, args.chain);
    let abi = client
        .contract_abi(args.contract_address.parse().unwrap()).await?;

		println!("hi: {:?}", abi.errors);

    Ok(())
}
