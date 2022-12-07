pub(crate) mod constants {
    use std::fs::File;
    use ethers::{contract::Contract, prelude::*, abi::Abi};

    pub async fn get_v2_pair_contract() -> Contract<Provider<Ws>> {
        let client = Provider::<Ws>::connect(
            "wss://ws-nd-624-743-149.p2pify.com/cea7fca36c7c64f53243ef5739238251",
        )
        .await.unwrap();
        let address_zero = "0x0000000000000000000000000000000000000000"
            .parse::<Address>()
            .unwrap();
        let ipancake_v2_pair = File::open("src/abi/IPancakeV2Pair.json").unwrap();
        
        let abi: Abi = serde_json::from_reader(ipancake_v2_pair).unwrap();
        Contract::new(address_zero, abi, client)
    }
}
