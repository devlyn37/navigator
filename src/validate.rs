use clap::ArgMatches;
use ethers::{abi::Address, types::Chain};
use std::{error::Error, str::FromStr};

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
) -> Result<(Chain, Address, String, Vec<u8>), InputError> {
    let chain_input = matches
        .get_one::<String>("CHAIN")
        .ok_or(InputError::new("need chain input"))?;
    let contract_input = matches
        .get_one::<String>("CONTRACT")
        .ok_or(InputError::new("need contract input"))?;
    let key_input = matches
        .get_one::<String>("ETHERSCAN_KEY")
        .ok_or(InputError::new("need etherscan api key"))?;
    let data_input = matches
        .get_one::<String>("DATA")
        .ok_or(InputError::new("need data input"))?;

    Ok((
        Chain::from_str(chain_input).map_err(|e| InputError::new("chain input invalid"))?,
        contract_input
            .parse()
            .map_err(|e| InputError::new("contract address invalid"))?,
        key_input.to_owned(),
        hex::decode(data_input.trim_start_matches("0x"))
            .map_err(|e| InputError::new("data not hex"))?,
    ))
}
