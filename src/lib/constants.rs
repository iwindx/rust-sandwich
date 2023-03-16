pub(crate) mod constants {
    use dotenv::dotenv;
    use ethers::{abi::Abi, contract::Contract, prelude::*, utils::hex};
    use rust_embed::RustEmbed;
    use std::env;
    use std::sync::Arc;

    #[derive(RustEmbed)]
    #[folder = "src/abi/"]
    struct Asset;

    pub struct SwapConfig {
        pub router: Address,
        pub factory: Address,
        pub init_code_hash: Bytes,
    }
    fn get_env_name(key: &str) -> String {
        dotenv().ok();
        match env::var(key) {
            Ok(data) => data,
            Err(err) => panic!("rpc node 不存在: {}", err),
        }
    }

    pub async fn get_v2_pair_contract() -> Contract<Arc<Provider<Ws>>> {
        let client = get_wss_client().await;
        
        let address_zero = "0x0000000000000000000000000000000000000000"
            .parse::<Address>()
            .unwrap();
        
        let ipancake_v2_pair1 = Asset::get("IPancakeV2Pair.json").unwrap();
        let pair_str = std::str::from_utf8(ipancake_v2_pair1.data.as_ref()).unwrap();
        let abi: Abi = serde_json::from_str(pair_str).unwrap();
        Contract::new(address_zero, abi, client)
    }

    pub async fn get_wss_client() -> Arc<Provider<Ws>> {
        let client = match Provider::<Ws>::connect(get_env_name("RPC_URL_WSS")).await {
            Ok(client) => client,
            Err(err) => panic!("连接异常: {}", err),
        };
        Arc::new(client)
    }

    pub fn get_swap_config() -> SwapConfig {
        dotenv().ok();
        SwapConfig {
            router: get_env_name("SWAP_ROUTER_ADDRESS").parse::<Address>().unwrap(),
            factory: get_env_name("SWAP_FACTORY_ADDRESS").parse::<Address>().unwrap(),
            init_code_hash: Bytes::from(hex::decode(get_env_name("SAWP_INIT_CODE_HASH")).unwrap()),
        }
    }
}
