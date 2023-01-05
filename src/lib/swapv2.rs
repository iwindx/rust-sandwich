pub(crate) mod swapv2 {

    use std::ops::{Add, Div, Mul, Sub};
    use std::fmt;
    use ethers::{abi::Tokenizable, prelude::*, utils::*};

    pub struct DataGivenIn {
        pub amount_out: U256,
        pub new_reserve_a: U256,
        pub new_reserve_b: U256,
    }
    use crate::lib::constants::constants::{get_v2_pair_contract, get_swap_config};

    impl fmt::Debug for DataGivenIn {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "DataGivenIn {{amount_out: {}, new_reserve_a: {}, new_reserve_b: {}}}",
                self.amount_out, self.new_reserve_a, self.new_reserve_b
            )
        }
    }

    pub fn sort_tokens(token_a: Address, token_b: Address) -> [Address; 2] {
        if token_a.lt(&token_b) {
            return [token_a, token_b];
        }
        return [token_b, token_a];
    }

    pub async fn get_reserve(pair: Address, token_a: Address, token_b: Address) -> (U256, U256) {
        let [token0, _] = sort_tokens(token_a, token_b);
        let v2_pair = get_v2_pair_contract().await;
        let (_reserve0, _reserve1, _timestamp) = v2_pair
            .at(pair)
            .method::<_, (U256, U256, u32)>("getReserves", ())
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

        get_create2_address_from_hash(get_swap_config().factory, salt.to_vec(),get_swap_config().init_code_hash)
    }

    pub fn get_data_given_in(amount_in: U256, reserve_a: U256, reserve_b: U256) -> DataGivenIn {
        let a_in_with_fee: U256 = amount_in.mul(997);
        let numerator = a_in_with_fee.mul(reserve_b);
        let denominator = a_in_with_fee.add(reserve_a.mul(1000));
        let b_out = numerator.div(denominator);

        let mut new_reserve_b = reserve_b.sub(b_out);
        if new_reserve_b.lt(&U256::from(0)) || new_reserve_b.gt(&reserve_b) {
            new_reserve_b = U256::from(1);
        }
        let mut new_reserve_a = reserve_a.add(amount_in);
        if new_reserve_a.lt(&new_reserve_a) {
            new_reserve_a = U256::from(2_i32.pow(255) - 1);
        }
        return DataGivenIn {
            amount_out: b_out,
            new_reserve_a,
            new_reserve_b,
        };
    }
}
