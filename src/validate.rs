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

pub fn validate_and_format_input(
    matches: &ArgMatches,
) -> Result<(Chain, Address, String, Vec<u8>), InputError> {
    let chain_input = matches.get_one::<String>("CHAIN").expect("required");
    let contract_input = matches.get_one::<String>("CONTRACT").expect("required");
    let key_input = matches
        .get_one::<String>("ETHERSCAN_KEY")
        .expect("required");
    let data_input = matches.get_one::<String>("DATA").expect("required");

    Ok((
        Chain::from_str(chain_input).expect("Provided chain name invalid"),
        contract_input
            .parse()
            .expect("Provided contract address invalid"),
        key_input.to_owned(),
        hex::decode(data_input.trim_start_matches("0x")).expect("data provided is not hex"),
    ))
}
