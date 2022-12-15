pub(crate) mod numeric {
    use crate::lib::swapv2::swapv2::*;
    use anyhow::*;
    use ethers::{prelude::*, utils::*};
    use std::ops::{Add, Div, Mul, Sub};

    pub struct SandwichState {
        pub frontrun: DataGivenIn,
        pub victiom: DataGivenIn,
        pub backrun: DataGivenIn
    }

    fn calcf(
        amount_in: U256,
        user_amount_in: U256,
        reserve_wbnb: U256,
        reserve_token: U256,
    ) -> U256 {
        let DataGivenIn {
            new_reserve_a,
            new_reserve_b,
            ..
        } = get_data_given_in(amount_in, reserve_wbnb, reserve_token);
        let DataGivenIn { amount_out, .. } =
            get_data_given_in(user_amount_in, new_reserve_a, new_reserve_b);
        return amount_out;
    }

    pub fn passf(amount_out: U256, user_min_recv_token: U256) -> bool {
        return amount_out.ge(&user_min_recv_token);
    }

    fn binary_search(left: U256, right: U256, params: [U256; 4], bnb: U256) -> U256 {
        let tolerance = U256::from(parse_units("0.01", "ether").unwrap());

        if right
            .sub(left)
            .gt(&tolerance.mul(right.add(left).div(2)).div(bnb))
        {
            let mid: U256 = right.add(left).div(2);
            let out = calcf(mid, params[0], params[2], params[3]);
            if passf(out, params[1]) {
                return binary_search(mid, right, params, bnb);
            }
            return binary_search(left, mid, params, bnb);
        }

        let ret = right.add(left).div(2);
        if ret.lt(&U256::from(0)) {
            return U256::from(0);
        }
        return ret;
    }

    pub fn calc_sandwich_optimal_in(
        user_amount_in: U256,
        user_min_recv_token: U256,
        reserve_wbnb: U256,
        reserve_token: U256,
    ) -> U256 {
        let bnb: U256 = U256::from(parse_units("1", "ether").unwrap());

        let lower_bound = parse_units("0", "ether").unwrap();
        let upper_bound = parse_units("0.3", "ether").unwrap();

        return binary_search(
            U256::from(lower_bound),
            U256::from(upper_bound),
            [
                user_amount_in,
                user_min_recv_token,
                reserve_wbnb,
                reserve_token,
            ],
            bnb,
        );
    }

    pub fn cal_sandwich_state(
        optimal_sandwich_weth_in: U256,
        user_weth_in: U256,
        user_min_recv: U256,
        reserve_weth: U256,
        reserve_token: U256,
    ) -> Result<SandwichState> {
        let front_run_state: DataGivenIn =
            get_data_given_in(optimal_sandwich_weth_in, reserve_weth, reserve_token);
        
        let victiom_state:DataGivenIn = get_data_given_in(user_weth_in, front_run_state.new_reserve_a, front_run_state.new_reserve_b);

        let backrun_state: DataGivenIn = get_data_given_in(front_run_state.amount_out, victiom_state.new_reserve_b, victiom_state.new_reserve_a);

        if victiom_state.amount_out.lt(&user_min_recv) {
            return Err(anyhow!("不满足条件"));
        }
        Ok(SandwichState {
            frontrun: front_run_state,
            victiom: victiom_state,
            backrun: backrun_state,
        })
    }
}
