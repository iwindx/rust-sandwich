mod lib;
use chrono::prelude::*;
use ethers::prelude::*;
use ethers::utils::format_units;
use eyre::Result;
use std::fmt;
use std::sync::Arc;
use lib::numeric::numeric::*;
use lib::parse::*;
use lib::swapv2::*;


impl fmt::Debug for SandwichState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SandwichState {{backrun: {:?}, frontrun: {:?}, victiom: {:?}}}", self.backrun, self.frontrun, self.victiom)
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let client = Provider::<Ws>::connect(
        "wss://ws-nd-624-743-149.p2pify.com/cea7fca36c7c64f53243ef5739238251",
    )
    .await?;
    println!("client.get_chainid(): {:?}", client.get_gas_price().await?);
    let client = Arc::new(client);

    let mut tx_stream = client
        .subscribe_pending_txs()
        .await?
        .transactions_unordered(256);
    let address = "0x10ed43c718714eb63d5aa57b78b54704e256024e".parse::<Address>()?;
    loop {
        if let Some(Ok(tx)) = tx_stream.next().await {
            if tx.to == Some(address) {
                // let hash = H256::from_str(
                //     "0x24574a25938c95d6b9313059bc78ad6a3f84a25dc9dc9b57a3adf0bf0307dfb2",
                // )?;
                // let transaction = client.get_transaction(hash).await?;

                // if let Some(tx) = transaction {
                let router_data_decoded = parse::parse_router_tx(tx.input.clone());
                if !router_data_decoded.is_err() {
                    let parse::TxInput {
                        amount_out_min,
                        path,
                        deadline,
                        ..
                    } = router_data_decoded.unwrap();

                    let dt = Utc::now().timestamp_millis() / 1000;

                    if  dt > i64::try_from(deadline.as_u64()).unwrap() {
                        continue;
                    }
                    // 用户正在发送准确的ETH（非WETH）
                    let user_amount_in = tx.value;

                    //注意：因为这是swapExactETHForTokens，所以路径总是这样
                    let [weth, token] = path;

                    let pair_to_sandwich = swapv2::get_pair_address(weth, token);

                    let (reserve_weth, reserve_token) =
                        swapv2::get_reserve(pair_to_sandwich, weth, token).await;

                    let optima_weth_in = calc_sandwich_optimal_in(
                        user_amount_in,
                        amount_out_min,
                        reserve_weth,
                        reserve_token,
                    );
                    let sandwich_states = match cal_sandwich_state(
                        optima_weth_in,
                        user_amount_in,
                        amount_out_min,
                        reserve_weth,
                        reserve_token,
                    ) {
                        Ok(state) => state,
                        Err(err) => {
                            println!("{}", err);
                            continue;
                        }
                    };
                    println!("sandwich_states: {:?}",sandwich_states);
                    if sandwich_states.backrun.amount_out.le(&optima_weth_in) {
                        println!("可能不赚钱，放弃了");
                        continue;
                    }
                    println!("get_block_number: {}", client.get_block_number().await?);
                    println!("optima_weth_in: {}, ", optima_weth_in);
                    println!("Sent tx: {}\n", serde_json::to_string(&tx)?);
                    println!("path: {:?}", path);
                    println!(
                        "\n目标交易 : {:?}\n optimalWethIn: {:?}\n 本次预计卖出费用: {:?}\n",
                        tx.hash,
                        format_units(optima_weth_in, "ether"),
                        format_units(sandwich_states.backrun.amount_out, "ether"),
                    );
                    
                }
                // }
            }
        }
    }
}
