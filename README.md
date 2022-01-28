# btc-price

stores five last price of btc, returns average 

```
build.sh
near deploy --accountId btcprice.letscamp.testnet --wasmFile res/btc_price.wasm
near call btcprice.letscamp.testnet new --accountId letscamp.testnet
near call btcprice.letscamp.testnet add_price '{"price": 33}' --accountId letscamp.testnet
near view btcprice.letscamp.testnet get_avg  --accountId letscamp.testnet


```

