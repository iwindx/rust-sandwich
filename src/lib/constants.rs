pub(crate) mod constants {
    use ethers::{contract::Contract, prelude::*, abi::Abi};
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "src/abi/"]
    struct Asset;

    pub async fn get_v2_pair_contract() -> Contract<Provider<Ws>> {
        let client = Provider::<Ws>::connect(
            "wss://ws-nd-624-743-149.p2pify.com/cea7fca36c7c64f53243ef5739238251",
        )
        .await.unwrap();
        let address_zero = "0x0000000000000000000000000000000000000000"
            .parse::<Address>()
            .unwrap();
        let ipancake_v2_pair1 = Asset::get("IPancakeV2Pair.json").unwrap();
        let pair_str = std::str::from_utf8(ipancake_v2_pair1.data.as_ref()).unwrap();
        let abi: Abi = serde_json::from_str(pair_str).unwrap();
        Contract::new(address_zero, abi, client)
    }
}
