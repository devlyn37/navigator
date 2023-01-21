# Navigator

A CLI for things I find myself doing often in EVM land.

## Decode Command

I often parse through RPC errors and want to see what happened on contracts I'm not familiar with. This command helps by decoding hex encoded data.

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

running

`cargo run decode mainnet 0x1d9317911cf1003b42a965574c29f18a87a2858c <KEY> 0x0209c6b7000000000000000000000000292f9d08efcf1a3a988959190d44f48a53577f100000000000000000000000000000000000000000000000000000000000000001 --kind=function`

outputs

`function name: mintSeasonPassNFT, arguments: [Address(0x292f9d08efcf1a3a988959190d44f48a53577f10), Uint(1)]`

running

`cargo run decode goerli 0x98AA442ceFCAF0A7277D10889d07d04E90B37eA5 <KEY> 0xd2ade556 --kind=error`

outputs

`IncorrectValue, arguments: []`