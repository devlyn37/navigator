use anyhow::{anyhow, Context};
use clap::ArgMatches;
use ethers::{abi::Address, types::Chain};
use std::str::FromStr;

pub fn decode_command(
    matches: &ArgMatches,
) -> Result<(Chain, Address, String, Vec<u8>), Box<dyn std::error::Error>> {
    let chain_input = matches
        .get_one::<String>("CHAIN")
        .ok_or(anyhow!("need chain input"))?;
    let chain = Chain::from_str(chain_input)
        .with_context(|| format!("error parsing chain input: {}", chain_input))?;

    let contract_input = matches
        .get_one::<String>("CONTRACT")
        .ok_or(anyhow!("need contract input"))?;
    let contract_address = contract_input
        .parse()
        .with_context(|| format!("error parsing contract input: {:?}", contract_input))?;

    let key = matches
        .get_one::<String>("ETHERSCAN_KEY")
        .ok_or(anyhow!("etherscan key required"))?;

    let data_input = matches
        .get_one::<String>("DATA")
        .ok_or(anyhow!("data input required"))?;
    let decoded_data = hex::decode(data_input.trim_start_matches("0x"))
        .with_context(|| format!("error encoding hex string: {}", data_input))?;

    Ok((chain, contract_address, key.to_owned(), decoded_data))
}
