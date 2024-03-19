use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    match client.get_slot() {
        Ok(block_height) => println!("Current block height: {}", block_height),
        Err(e) => eprintln!("Error : {:?}", e),
    }
}
