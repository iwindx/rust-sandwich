mod lib;
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;
use chrono::prelude::*;

use lib::parse::*;
use lib::swapv2::*;
use crate::lib::parse::parse::TxInput;
use crate::lib::swapv2::swapv2::get_reserve;
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
                println!("监听到交易:{}", tx.hash);
                if !router_data_decoded.is_err() {
                    println!("命中交易 Hash: {:?}", tx.hash);

                    let TxInput {amount_out_min, path, to, deadline} = router_data_decoded.unwrap();
                    println!(
                        "amountOutMin: {:?}, path: {:?}, to: {:?}, deadline: {:?}", amount_out_min, path, to, deadline
                    );

                    let dt = Utc::now().timestamp_millis() / 1000;
                    println!("deadline.as_u64(): {}, dt: {}", deadline.as_u64(), dt);
                    
                    if  dt > i64::try_from(deadline.as_u64()).unwrap() {
                        continue;
                    }
                    // 用户正在发送准确的ETH（非WETH）
                    let user_amount_in = tx.value;

                    //注意：因为这是swapExactETHForTokens，所以路径总是这样
                    let [weth, token] = path;

                    let pair_to_sandwich = swapv2::get_pair_address(weth, token);
                    println!("pair_to_sandwich: {:?}", pair_to_sandwich);
                    let result = get_reserve(pair_to_sandwich, weth, token).await;
                    println!("result:{:?}", result)
                    
                }
            }
        }
    }
}
