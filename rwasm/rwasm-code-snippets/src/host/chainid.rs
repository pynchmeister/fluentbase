use crate::{
    common::u256_from_slice,
    common_sp::{u256_push, SP_VAL_MEM_OFFSET_DEFAULT},
};
use fluentbase_sdk::evm::ExecutionContext;

#[no_mangle]
pub fn host_chainid() {
    let v = ExecutionContext::env_chain_id().to_be_bytes();

    u256_push(SP_VAL_MEM_OFFSET_DEFAULT, u256_from_slice(&v));
}