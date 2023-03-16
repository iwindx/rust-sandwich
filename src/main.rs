mod lib;
use chrono::prelude::*;
use ethers::prelude::*;
use ethers::utils::format_units;
use eyre::Result;
use lib::constants::constants::*;
use lib::numeric::numeric::*;
use lib::parse::*;
use lib::swapv2::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let client = get_wss_client().await;

    let mut tx_stream = client
        .subscribe_pending_txs()
        .await?
        .transactions_unordered(256);


    let address = get_swap_config().router;

    loop {
        if let Some(Ok(tx)) = tx_stream.next().await {
            dbg!("交易信息", &tx);

            // 不等于 dex 的交易跳过
            if tx.to != Some(address) {
                continue;
            }

            let router_data_decoded = parse::parse_router_tx(&tx.input);
            if router_data_decoded.is_err() {
                continue;
            }
            let parse::TxInput {
                amount_out_min,
                path,
                deadline,
                ..
            } = router_data_decoded.unwrap();

            let dt = Utc::now().timestamp_millis() / 1000;

            if dt > i64::try_from(deadline.as_u64()).unwrap() {
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
            println!("sandwich_states: {:?}", sandwich_states);
            
            if sandwich_states.backrun.amount_out.le(&optima_weth_in) {
                println!("可能不赚钱，放弃了");
                continue;
            }
            println!("get_block_number: {}", client.get_block_number().await?);
            println!("optima_weth_in: {}, ", optima_weth_in);
            println!("path: {:?}", path);
            println!(
                "\n目标交易hash : {} \n optimalWethIn: {:?} \n 本次预计卖出费用: {:?}\n",
                tx.hash,
                format_units(optima_weth_in, "ether"),
                format_units(sandwich_states.backrun.amount_out, "ether"),
            );
            panic!("结束");
        }
    }
}
