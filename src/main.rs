use anyhow::{Context, Result};
use clap::{arg, Command};
use ethers::{
    abi::{Contract, Token},
    prelude::Client,
};
mod decode;
mod validate;

fn cli() -> Command {
    let decode_args = [
        arg!(<CHAIN> "the chain the contract is deployed to"),
        arg!(<CONTRACT> "the contract the data is related to"),
        arg!(<ETHERSCAN_KEY> "an api key for etherscan"),
        arg!(<DATA> "the hex encoded data"),
        arg!(--kind <KIND>)
            .value_parser(["function", "error"])
            .num_args(1)
            .required(true)
            .require_equals(true),
    ];

    Command::new("navigator")
        .about("Some tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("decode")
                .about("decodes hex ethereum call data")
                .arg_required_else_help(true)
                .args(decode_args),
        )
}

#[tokio::main] // https://i.redd.it/x9dh4zxf83yz.jpg
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("decode", sub_matches)) => {
            let (chain, address, key, data, data_type) = validate::decode_command(sub_matches)
                .with_context(|| "there's a problem with the input")?;

            let client =
                Client::new(chain, key).with_context(|| "error connecting to etherscan")?;
            let abi = client
                .contract_abi(address)
                .await
                .with_context(|| "error fetching abi from etherscan")?;

            let decoding_function: fn(
                Contract,
                Vec<u8>,
            ) -> Option<(std::string::String, Vec<Token>)>;
            match data_type.as_str() {
                "error" => decoding_function = decode::error,
                "function" => decoding_function = decode::function,
                _ => unreachable!(),
            }

            let (name, args) =
                decoding_function(abi, data).with_context(|| format!("error decoding data"))?;
            println!("{} name: {}, arguments: {:?}", data_type, name, args);
            Ok(())
        }
        _ => unreachable!(),
    }
}
