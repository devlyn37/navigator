use ethers::{abi::Contract, abi::Token};

pub fn error(contract: Contract, data: Vec<u8>) -> Option<(String, Vec<Token>)> {
    let found = contract
        .errors
        .into_values()
        .filter_map(|x| x.into_iter().nth(0))
        .find(|error| {
            let error_signature = &error.signature().to_fixed_bytes()[0..4];
            let data_signature = &data[0..4];
            return error_signature == data_signature;
        })?;

    let params = &data[4..];
    let decoded = found.decode(params).ok()?;
    Some((found.name.to_owned(), decoded))
}

pub fn function(contract: Contract, data: Vec<u8>) -> Option<(String, Vec<Token>)> {
    let found = contract
        .functions
        .into_values()
        .filter_map(|x| x.into_iter().nth(0))
        .find(|function| {
            let signature = &data[0..4];
            return signature == function.short_signature();
        })?;

    let params = &data[4..];
    let decoded = found.decode_input(params).ok()?;
    Some((found.name.to_owned(), decoded))
}
