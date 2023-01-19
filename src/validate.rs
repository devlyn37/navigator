use anyhow::Context;
use clap::ArgMatches;
use ethers::{abi::Address, types::Chain};
use std::{error::Error, str::FromStr};

// TODO this seems like a lot XD

#[derive(Debug)]
pub struct InputError {
    pub message: String,
}

impl InputError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }

    fn description(&self) -> &str {
        return &self.message;
    }

    fn cause(&self) -> Option<&dyn Error> {
        return None;
    }
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn decode_command(
    matches: &ArgMatches,
) -> Result<(Chain, Address, String, Vec<u8>), Box<dyn std::error::Error>> {
    let chain_input = matches
        .get_one::<String>("CHAIN")
        .ok_or(InputError::new("need chain input"))?;
    let chain = Chain::from_str(chain_input)
        .with_context(|| format!("error parsing chain input: {}", chain_input))?;

    let contract_input = matches
        .get_one::<String>("CONTRACT")
        .ok_or(InputError::new("need contract input"))?;
    let contract_address = contract_input
        .parse()
        .with_context(|| format!("error parsing contract input: {:?}", contract_input))?;

    let key = matches
        .get_one::<String>("ETHERSCAN_KEY")
        .with_context(|| "etherscan key required")?;

    let data_input = matches
        .get_one::<String>("DATA")
        .ok_or(InputError::new("need data input"))?;
    let decoded_data = hex::decode(data_input.trim_start_matches("0x"))
        .with_context(|| format!("error encoding hex string: {}", data_input))?;

    Ok((chain, contract_address, key.to_owned(), decoded_data))
}
