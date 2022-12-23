# Ethereum Transaction From Scratch In Rust

To use this crate first crate Transaction struct 

```rust
use ethereum_types::H160; 

let tx = Transaction {
    // Nonce of the transaction
    nonce: 225,

    // To Address
    to: Some(
	H160::from_str(&"70997970C51812dc3A010C7d01b50e0d17dc79C6")
	    .unwrap()
	    .to_fixed_bytes(),
    ),

    // Value
    value: 10000000000,

    // Chain ID
    chain_id: 988242,

    // Rest is default
    ..Default::default()
};
```

You can also specify data if you want to call or deploy a smart contract:

```rust
use ethereum_types::H160; 

let data = vec![0, 0, 0, 0];

let tx = Transaction {
    // Nonce of the transaction
    nonce: 225,

    // To Address
    to: Some(
	H160::from_str(&"70997970C51812dc3A010C7d01b50e0d17dc79C6")
	    .unwrap()
	    .to_fixed_bytes(),
    ),

    // Value
    value: 10000000000,

    // Chain ID
    chain_id: 988242,

    data,

    // Rest is default
    ..Default::default()
}
```


After creating the struct you can just call the `sign` method with your private key:

```rust
use ethereum_types::H256; 

// Add your private key
// This is a know private key from hardhat test accounts
let private_key =
    H256::from_str("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap();

// Sign the transaction
let tx_bytes = tx.sign(private_key.as_bytes());
```

If you want to send your signed transaction you will need to crate a json object from the signed bytes:

```rust
use web3::types::Bytes;

// Convert Vec<u8> to Bytes so it can be serialized
let tx_bytes = Bytes::from(tx_bytes);

// Convert it to JSON value
let signed_tx = serde_json::to_value(tx_bytes).unwrap();
```

You can find detailed example in [examples](https://github.com/Kuly14/eth-tx-from-scratch/tree/main/examples)

## Acknowledgments

Credit goes to [synlestidae](https://github.com/synlestidae) creator of [ethereum-tx-sign](https://github.com/synlestidae/ethereum-tx-sign). His crate helped me to understand how are ethereum transactions encoded and signed.

This crate is just simpler implementation of [ethereum-tx-sign](https://github.com/synlestidae/ethereum-tx-sign).

## Disclaimer

**This is untested, unaudited software don't use in production or with real crypto!!!**

