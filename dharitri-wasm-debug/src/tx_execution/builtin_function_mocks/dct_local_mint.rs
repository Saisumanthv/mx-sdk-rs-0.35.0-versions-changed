use crate::num_bigint::BigUint;
use dharitri_wasm::api::DCT_LOCAL_MINT_FUNC_NAME;

use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult, TxResultCalls},
    world_mock::DctInstanceMetadata,
};

pub fn execute_local_mint(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() != 2 {
        let err_result = TxResult::from_vm_error("DCTLocalMint expects 2 arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }

    let token_identifier = tx_input.args[0].clone();
    let value = BigUint::from_bytes_be(tx_input.args[1].as_slice());

    tx_cache.increase_dct_balance(
        &tx_input.to,
        &token_identifier,
        0,
        &value,
        DctInstanceMetadata::default(),
    );

    let dct_nft_create_log = TxLog {
        address: tx_input.from,
        endpoint: DCT_LOCAL_MINT_FUNC_NAME.to_vec(),
        topics: vec![token_identifier.to_vec(), Vec::new(), value.to_bytes_be()],
        data: vec![],
    };

    let tx_result = TxResult {
        result_status: 0,
        result_message: String::new(),
        result_values: Vec::new(),
        result_logs: vec![dct_nft_create_log],
        result_calls: TxResultCalls::empty(),
    };

    (tx_result, tx_cache.into_blockchain_updates())
}