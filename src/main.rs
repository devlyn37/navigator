use anyhow::Result;
use clap::Parser;
use ethers::{etherscan::Client, types::Chain};
/// Search for a pattern in a file and display th elines that contain it.
#[derive(Parser)]
struct Cli {
    contract_address: String,
		network: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
		let args = Cli::parse();
		println!("contract address: {:?}", args.contract_address);
		println!("network: {:?}", args.network);
		let api_key = "K14P3TW12QCI2VDR3YIDY7XA9Y5XP2D232";
		let client = Client::new(Chain::Mainnet, api_key).unwrap();
    let abi = client
        .contract_abi(args.contract_address.parse().unwrap()).await?;

		let events = abi.events;
		println!("hi: {:?}", events);

    Ok(())
}
