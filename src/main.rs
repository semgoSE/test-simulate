use std::{sync::Arc, time::Instant};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::{Transaction, VersionedTransaction}};
use spl_memo::build_memo;

#[tokio::main]
async fn main() {

    // http://localhost:8899
    let rpc_client = RpcClient::new(
        "http://localhost:8899".to_string(),
    );

    let keypair: Keypair = Keypair::from_base58_string(
        "4LGo97SaEb1qYfrWh17CcpsMCyswVkWEECt9Jp7xsZormFsk4vTCAUbXczttty7fzTMDLkPLXHs7jBj9Ga6vqg9H",
    );
    
    let blockhash = rpc_client.get_latest_blockhash().await.expect("hash not found");

    // if Ok((blockhash)) =

    let frontrun_tx = VersionedTransaction::from(Transaction::new_signed_with_payer(
        &[
            build_memo(
                format!("{}", "hello 2")
                    .as_bytes(),
                &[],
            ),
        ],
        Some(&keypair.pubkey()),
        &[&keypair],
        blockhash,
    ));


    while true {
        let instant = Instant::now();
        let r = rpc_client.simulate_transaction(&frontrun_tx).await;
        println!("simulation time us {:?}", instant.elapsed());
    }        
}
