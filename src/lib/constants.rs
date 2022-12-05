pub(crate) mod constants {
    use std::{env, time::Duration};

    use dotenv;
    use ethers::prelude::*;
    #[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))] 
    pub async fn wss_provider() {
        dotenv::dotenv().ok();
        let ws_url = env::var("RPC_URL_WSS").unwrap();
        println!("{}", ws_url);
        let ws = Ws::connect("wss://bsc-mainnet.nodereal.io/ws/v1/b54be36f27f14269bb75706ac63314aa").unwrap();
        println!("{}", 2);
        let client = Provider::new(ws).interval(Duration::from_millis(2000));
        let mut tx_stream = client
            .subscribe_pending_txs()
            .await
            .unwrap()
            .transactions_unordered(256);

        loop {
            if let Some(Ok(tx)) = tx_stream.next().await {
                println!("{}",1);
                let _ = dbg!(client.trace_transaction(tx.hash).await);
            }
        }

    }
}
