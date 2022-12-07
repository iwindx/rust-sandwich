pub(crate) mod parse {
    use ethers::{abi::AbiDecode, contract::AbiError, prelude::*};
    use eyre::Result;
    // Abigen creates a SwapExactTokensForTokensCall struct that can be used to decode
    // the call data for the swapExactTokensForTokens function in the IUniswapV2Router02 contract
    abigen!(IPancakeV2RouterABI, "src/abi/IPancakeV2Router02.json");


    pub struct TxInput {
        pub amount_out_min: U256,
        pub path: [Address; 2],
        pub to: Address,
        pub deadline: U256,
    }

    pub fn parse_router_tx(tx_data: Bytes) -> Result<TxInput, AbiError> {
        let _decoded: Result<SwapExactETHForTokensCall, AbiError> =
            SwapExactETHForTokensCall::decode(&tx_data);

        if _decoded.is_err() {
            return Err(_decoded.err().unwrap());
        }
        let decoded = _decoded.unwrap();
        let mut path = decoded.path.into_iter();
        let from = path.next().unwrap();
        let to = path.next().unwrap();
        Ok(TxInput {
            amount_out_min: decoded.amount_out_min,
            path: [from, to],
            to: decoded.to,
            deadline: decoded.deadline,
        })
    }
}

