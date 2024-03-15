use crate::account_types::JZKT_ACCOUNT_BALANCE_FIELD;
use alloc::vec::Vec;
use byteorder::{ByteOrder, LittleEndian};
use fluentbase_sdk::{
    evm::{ContractInput, IContractInput},
    Bytes32,
    LowLevelAPI,
    LowLevelSDK,
};
use fluentbase_types::{Address, B256, STATE_DEPLOY, STATE_MAIN, U256};
use revm_interpreter::primitives::ShanghaiSpec;
use rwasm_codegen::{Compiler, CompilerConfig, FuncOrExport, ImportLinker, ImportLinkerDefaults};

pub type DefaultEvmSpec = ShanghaiSpec;

#[inline]
pub(crate) fn get_contract_input_offset_and_len() -> (u32, u32) {
    let mut header = [0u8; 8];
    LowLevelSDK::sys_read(
        &mut header,
        <ContractInput as IContractInput>::ContractInput::FIELD_OFFSET as u32,
    );
    let offset = LittleEndian::read_u32(&header[0..4]);
    let length = LittleEndian::read_u32(&header[4..8]);
    (offset, length)
}

#[inline(always)]
pub(crate) fn read_address_from_input(offset: usize) -> Address {
    let mut address = [0u8; Address::len_bytes()];
    LowLevelSDK::sys_read(&mut address, offset as u32);
    Address::from(address)
}

#[inline(always)]
pub(crate) fn read_balance(address: Address, value: &mut U256) {
    let mut bytes32 = Bytes32::default();
    unsafe {
        core::ptr::copy(address.as_ptr(), bytes32.as_mut_ptr(), 20);
    }
    LowLevelSDK::jzkt_get(bytes32.as_ptr(), JZKT_ACCOUNT_BALANCE_FIELD, unsafe {
        value.as_le_slice_mut().as_mut_ptr()
    });
}

#[inline(always)]
pub fn calc_create_address(deployer: &Address, nonce: u64) -> Address {
    use alloy_rlp::{Encodable, EMPTY_LIST_CODE, EMPTY_STRING_CODE};
    const MAX_LEN: usize = 1 + (1 + 20) + 9;
    let len = 22 + nonce.length();
    debug_assert!(len <= MAX_LEN);
    let mut out = [0u8; MAX_LEN + 1];
    out[0] = EMPTY_LIST_CODE + len as u8 - 1;
    out[1] = EMPTY_STRING_CODE + 20;
    out[2..22].copy_from_slice(deployer.as_slice());
    nonce.encode(&mut &mut out[22..]);
    LowLevelSDK::crypto_keccak256(out.as_ptr(), out.len() as u32, out.as_mut_ptr());
    Address::from_word(B256::from(out))
}

#[inline(always)]
pub fn calc_create2_address(deployer: &Address, salt: &B256, init_code_hash: &B256) -> Address {
    let mut bytes = [0; 85];
    bytes[0] = 0xff;
    bytes[1..21].copy_from_slice(deployer.as_slice());
    bytes[21..53].copy_from_slice(salt.as_slice());
    bytes[53..85].copy_from_slice(init_code_hash.as_slice());
    LowLevelSDK::crypto_keccak256(bytes.as_ptr(), bytes.len() as u32, bytes.as_mut_ptr());
    let bytes32: [u8; 32] = bytes[0..32].try_into().unwrap();
    Address::from_word(B256::from(bytes32))
}

#[inline(always)]
pub fn wasm2rwasm(bytecode: &[u8], is_deploy: bool) -> Vec<u8> {
    let mut import_linker = ImportLinker::default();
    ImportLinkerDefaults::new_v1alpha().register_import_funcs(&mut import_linker);
    let mut compiler =
        Compiler::new_with_linker(bytecode, CompilerConfig::default(), Some(&import_linker))
            .unwrap();
    compiler
        .translate(FuncOrExport::Export(if is_deploy {
            "deploy"
        } else {
            "main"
        }))
        .unwrap();
    compiler.finalize().unwrap()
}

#[inline(always)]
pub fn rwasm_exec(bytecode: &[u8], input: &[u8], gas_limit: u32, is_deploy: bool) {
    let exit_code = LowLevelSDK::sys_exec(
        bytecode.as_ptr(),
        bytecode.len() as u32,
        input.as_ptr(),
        input.len() as u32,
        core::ptr::null_mut(),
        0,
        &gas_limit as *const u32,
        if is_deploy { STATE_DEPLOY } else { STATE_MAIN },
    );
    if exit_code != 0 {
        panic!("failed to execute rwasm bytecode, exit code: {}", exit_code);
    }
}