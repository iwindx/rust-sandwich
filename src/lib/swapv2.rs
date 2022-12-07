pub(crate) mod swapv2 {

    use ethers::{
        abi::{AbiEncode, Token, Tokenizable},
        prelude::*,
        utils::*,
    };

    use crate::lib::constants::constants::get_v2_pair_contract;
    pub fn getExactWethTokenMinRecv(finalMinRecv: U256, path: [Address; 2]) {}

    pub fn sort_tokens(token_a: Address, token_b: Address) -> [Address; 2] {
        if token_a.lt(&token_b) {
            return [token_a, token_b];
        }
        return [token_b, token_a];
    }

    pub async fn get_reserve(pair: Address, token_a: Address, token_b: Address) -> (u128, u128) {
        let [token0, _] = sort_tokens(token_a, token_b);
        let v2_pair = get_v2_pair_contract().await;
        let (_reserve0, _reserve1, _timestamp) = v2_pair
            .at(pair)
            .method::<_, (u128, u128, u32)>("getReserves", ())
            .unwrap()
            .call()
            .await
            .unwrap();

        if token_a.eq(&token0) {
            return (_reserve0, _reserve1);
        }
        (_reserve1, _reserve0)
    }

    pub fn get_pair_address(token_a: Address, token_b: Address) -> Address {
        let [token0, token1] = sort_tokens(token_a, token_b);

        let token_prefix = format!("{}{}", "0x", token0.into_token().to_string());
        let calldata: Bytes = (token_prefix + &token1.into_token().to_string())
            .parse()
            .unwrap();

        let salt = keccak256(&calldata);

        let code = Bytes::from(
            hex::decode("00fb7f630766e6a796048ea87d01acd3068e8ff67d078148a3fa3f4a84f69bd5")
                .unwrap(),
        );

        let factory: Address = "0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73"
            .parse()
            .unwrap();

        get_create2_address_from_hash(factory, salt.to_vec(), code)
    }
}
