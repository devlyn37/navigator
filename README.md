# Navigator

A CLI for things I find myself doing often in EVM land.

This is a WIP and implemented in Rust for fun

## Decode Command

I often parse through logs like this:

```
code: 'SERVER_ERROR',
    body: '{"jsonrpc":"2.0","id":44,"error":{"code":3,"message":"execution reverted","data":"0xc373e55d"}}',
    error: {
      code: 3,
      data: '0xc373e55d'
	requestBody: '{"method":"eth_estimateGas","params":[{"value":<VALUE>,"from":<FROM>,"to":<CONTRACT>,"data":"0x493610a600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000cc5060f9674aff5d32122b29d4c52373558f0fca"}],"id":44,"jsonrpc":"2.0"}',
    requestMethod: <METHOD>,
    url: <URL>
  }
},
```

this happens when contracts revert with custom errors (ex. shown in solidity below) are thrown

```
error InsufficientBalance(uint256 available, uint256 required);
```

It's difficult to grok what's going on from the error logs, the `decode` command decodes the hex data for errors and functions like they're provided in the example above.

It does so by:

- fetching the abi
- matching the function or error signature with the data
- decoding the parameters according to that matched function or error

### Usage

```
Usage: navigator decode --kind=<KIND> <CHAIN> <CONTRACT> <ETHERSCAN_KEY> <DATA>

Arguments:
  <CHAIN>          the chain the contract is deployed to
  <CONTRACT>       the contract the data is related to
  <ETHERSCAN_KEY>  an api key for etherscan
  <DATA>           the hex encoded data

Options:
      --kind=<KIND>  [possible values: function, error]
  -h, --help         Print help information
```

### Examples

decoding function data

```
cargo run decode mainnet 0x1d9317911cf1003b42a965574c29f18a87a2858c <KEY> 0x0209c6b7000000000000000000000000292f9d08efcf1a3a988959190d44f48a53577f100000000000000000000000000000000000000000000000000000000000000001 --kind=function
```

output:

```
function name: mintSeasonPassNFT, arguments: [Address(0x292f9d08efcf1a3a988959190d44f48a53577f10), Uint(1)]`
```

decoding error data

```
cargo run decode goerli 0x98AA442ceFCAF0A7277D10889d07d04E90B37eA5 <KEY> 0xd2ade556 --kind=error
```

output:

```
IncorrectValue, arguments: []
```