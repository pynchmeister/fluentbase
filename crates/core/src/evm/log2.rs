use crate::{account_types::Topics, helpers::read_address_from_input};
use core::ptr;
use fluentbase_sdk::{
    evm::{ContractInput, IContractInput},
    Bytes32,
    LowLevelAPI,
    LowLevelSDK,
};

#[no_mangle]
pub fn _evm_log2(
    data_offset: *const u8,
    data_size: u32,
    topic32_1_offset: *const u8,
    topic32_2_offset: *const u8,
) {
    const TOPICS_COUNT: usize = 2;

    let mut address_bytes32 = Bytes32::default();
    let address =
        read_address_from_input(<ContractInput as IContractInput>::ContractAddress::FIELD_OFFSET);
    unsafe { ptr::copy(address.as_ptr(), address_bytes32[12..].as_mut_ptr(), 20) };

    let mut topics = Topics::<TOPICS_COUNT>::default();
    unsafe { ptr::copy(topic32_1_offset, topics[0].as_mut_ptr(), 1) }
    unsafe { ptr::copy(topic32_2_offset, topics[1].as_mut_ptr(), 1) }

    LowLevelSDK::jzkt_emit_log(
        address_bytes32.as_ptr(),
        topics.as_ptr(),
        TOPICS_COUNT as u32,
        data_offset,
        data_size,
    );
}
