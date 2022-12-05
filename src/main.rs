mod lib;
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use lib::parse::*;

use crate::lib::parse::parse::TxInput;
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let client = Provider::<Ws>::connect(
        "wss://ws-nd-624-743-149.p2pify.com/cea7fca36c7c64f53243ef5739238251",
    )
    .await?;
    let client = Arc::new(client);

    let mut tx_stream = client
        .subscribe_pending_txs()
        .await?
        .transactions_unordered(256);
    let address = "0x10ed43c718714eb63d5aa57b78b54704e256024e".parse::<Address>()?;
    loop {
        if let Some(Ok(tx)) = tx_stream.next().await {
            if tx.to == Some(address) {

                let router_data_decoded = parse::parse_router_tx(tx.input);

                if !router_data_decoded.is_err() {
                    println!("命中交易 Hash: {:?}", tx.hash);
                    let TxInput {amount_out_min, path, to, deadline} = router_data_decoded.unwrap();
                    println!(
                        "amountOutMin: {:?}, path: {:?}, to: {:?}, deadline: {:?}", amount_out_min, path, to, deadline
                    )
                }
            }
        }
    }
}
