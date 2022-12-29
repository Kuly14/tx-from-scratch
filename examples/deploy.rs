use ethereum_types::H256;
use std::str::FromStr;
use web3::types::Bytes;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;

use tx_from_scratch::Transaction;

#[tokio::main]
pub async fn main() {
    // Deploy bytecode of the contract
    // You can get this with remix or hardhat
    let data = "608060405234801561001057600080fd5b5060b68061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c8063cc80f6f314602d575b600080fd5b60336047565b604051603e91906067565b60405180910390f35b6000600a905090565b6000819050919050565b6061816050565b82525050565b6000602082019050607a6000830184605a565b9291505056fea26469706673582212209b541ed574700ac76c591bec0d31d371ca1203903669cfb5c59334ea8952ed9564736f6c63430008110033";

    // Decode it to Vec<u8>
    let data = hex::decode(data).unwrap();

    let tx = Transaction {
        nonce: 234,
        // To is None because we want to deploy the contract
        to: None,
        chain_id: 988242,
        data,
        // Gas has to be more than the default because contract deployment can get expensive
        gas: 840000,
        ..Default::default()
    };

    let private_key =
        H256::from_str("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap();

    // Sign the transaction
    let tx_bytes = tx.sign(private_key.as_bytes());

    // Convert Vec<u8> to Bytes so it can be serialized
    let tx_bytes = Bytes::from(tx_bytes);

    // Convert it to JSON value
    let signed_tx = serde_json::to_value(tx_bytes).unwrap();
    println!("{:#?}", signed_tx);

    // Start a Json RPC client
    let url = String::from("http://10.5.0.2:8545/");
    let client = HttpClientBuilder::default().build(url).unwrap();

    // Call the node with the signed signature
    let params = rpc_params![signed_tx];
    let response: Result<String, _> = client.request("eth_sendRawTransaction", params).await;

    // Print the result
    println!("TX: {:#?}", response.unwrap());
}
