use ethereum_types::{H160, H256};
use std::str::FromStr;
use web3::types::Bytes;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;

use tx_from_scratch::Transaction;

#[tokio::main]
async fn main() {
    // Construct Transaction
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

    // Add your private key
    // This is a know private key from hardhat test accounts
    let private_key =
        H256::from_str("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap();

    // Sign the transaction
    let tx_bytes = tx.sign(private_key.as_bytes());

    // Convert Vec<u8> to Bytes so it can be serialized
    let tx_bytes = Bytes::from(tx_bytes);

    // Convert it to JSON value
    let signed_tx = serde_json::to_value(tx_bytes).unwrap();

    // Start a Json RPC client
    let url = String::from("http://10.5.0.2:8545/");
    let client = HttpClientBuilder::default().build(url).unwrap();

    // Call the node with the signed signature
    let params = rpc_params![signed_tx];
    let response: Result<String, _> = client.request("eth_sendRawTransaction", params).await;

    // Print the result
    println!("TX: {:#?}", response);
}
