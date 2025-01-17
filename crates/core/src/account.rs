use crate::account_types::{
    AccountCheckpoint,
    AccountFields,
    JZKT_ACCOUNT_BALANCE_FIELD,
    JZKT_ACCOUNT_NONCE_FIELD,
    JZKT_ACCOUNT_RWASM_BYTECODE_HASH_FIELD,
    JZKT_ACCOUNT_RWASM_BYTECODE_SIZE_FIELD,
    JZKT_ACCOUNT_SOURCE_BYTECODE_HASH_FIELD,
    JZKT_ACCOUNT_SOURCE_BYTECODE_SIZE_FIELD,
    JZKT_COMPRESSION_FLAGS,
};
use alloc::vec;
use byteorder::{ByteOrder, LittleEndian};
use fluentbase_sdk::{Bytes32, LowLevelAPI, LowLevelSDK};
use fluentbase_types::{Address, Bytes, ExitCode, B256, KECCAK_EMPTY, POSEIDON_EMPTY, U256};

#[derive(Debug, Clone)]
pub struct Account {
    pub address: Address,
    pub balance: U256,
    pub nonce: u64,
    pub source_bytecode_size: u64,
    pub source_bytecode_hash: B256,
    pub rwasm_bytecode_size: u64,
    pub rwasm_bytecode_hash: B256,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            address: Address::ZERO,
            rwasm_bytecode_size: 0,
            source_bytecode_size: 0,
            nonce: 0,
            balance: U256::ZERO,
            rwasm_bytecode_hash: POSEIDON_EMPTY,
            source_bytecode_hash: KECCAK_EMPTY,
        }
    }
}

impl Account {
    fn new(address: &Address) -> Self {
        Self {
            address: address.clone(),
            ..Default::default()
        }
    }

    pub fn new_from_jzkt(address: &Address) -> Self {
        let mut result = Self::new(address);
        let address_word = address.into_word();
        // code size and nonce
        let mut buffer32 = Bytes32::default();

        Account::jzkt_get_nonce(address_word.as_ptr(), buffer32.as_mut_ptr());
        result.nonce = LittleEndian::read_u64(&buffer32);

        Account::jzkt_get_balance(address_word.as_ptr(), unsafe {
            result.balance.as_le_slice_mut().as_mut_ptr()
        });

        Account::jzkt_get_bytecode_size(address_word.as_ptr(), buffer32.as_mut_ptr());
        result.rwasm_bytecode_size = LittleEndian::read_u64(&buffer32);

        Account::jzkt_get_bytecode_hash(
            address_word.as_ptr(),
            result.rwasm_bytecode_hash.as_mut_ptr(),
        );

        Account::jzkt_get_source_bytecode_size(address_word.as_ptr(), buffer32.as_mut_ptr());
        result.source_bytecode_size = LittleEndian::read_u64(&buffer32);

        Account::jzkt_get_source_bytecode_hash(
            address_word.as_ptr(),
            result.source_bytecode_hash.as_mut_ptr(),
        );

        result
    }

    #[inline]
    pub fn jzkt_get_nonce(address32_offset: *const u8, buffer32_le_offset: *mut u8) {
        LowLevelSDK::jzkt_get(
            address32_offset,
            JZKT_ACCOUNT_NONCE_FIELD,
            buffer32_le_offset,
        );
    }

    #[inline]
    pub fn jzkt_get_balance(address32_offset: *const u8, buffer32_le_offset: *mut u8) {
        LowLevelSDK::jzkt_get(
            address32_offset,
            JZKT_ACCOUNT_BALANCE_FIELD,
            buffer32_le_offset,
        );
    }

    #[inline]
    pub fn jzkt_get_bytecode_size(address32_offset: *const u8, buffer32_le_offset: *mut u8) {
        LowLevelSDK::jzkt_get(
            address32_offset,
            JZKT_ACCOUNT_RWASM_BYTECODE_SIZE_FIELD,
            buffer32_le_offset,
        );
    }

    #[inline]
    pub fn jzkt_get_bytecode_hash(address32_offset: *const u8, buffer32_offset: *mut u8) {
        LowLevelSDK::jzkt_get(
            address32_offset,
            JZKT_ACCOUNT_RWASM_BYTECODE_HASH_FIELD,
            buffer32_offset,
        );
    }

    #[inline]
    pub fn jzkt_get_source_bytecode_size(address32_offset: *const u8, buffer32_le_offset: *mut u8) {
        LowLevelSDK::jzkt_get(
            address32_offset,
            JZKT_ACCOUNT_SOURCE_BYTECODE_SIZE_FIELD,
            buffer32_le_offset,
        );
    }

    #[inline]
    pub fn jzkt_get_source_bytecode_hash(address32_offset: *const u8, buffer32_offset: *mut u8) {
        LowLevelSDK::jzkt_get(
            address32_offset,
            JZKT_ACCOUNT_SOURCE_BYTECODE_HASH_FIELD,
            buffer32_offset,
        );
    }

    #[inline(always)]
    pub(crate) fn transfer_value(&mut self, to: &mut Self, value: &U256) -> bool {
        let from_balance = {
            let new_value = self.balance.checked_sub(*value);
            if new_value.is_none() {
                return false;
            }
            new_value.unwrap()
        };
        let to_balance = {
            let new_value = to.balance.checked_add(*value);
            if new_value.is_none() {
                return false;
            }
            new_value.unwrap()
        };
        self.balance = from_balance;
        to.balance = to_balance;
        true
    }

    pub fn get_fields(&self) -> AccountFields {
        let mut account_fields: AccountFields = Default::default();
        LittleEndian::write_u64(
            &mut account_fields[JZKT_ACCOUNT_RWASM_BYTECODE_SIZE_FIELD as usize][..],
            self.rwasm_bytecode_size,
        );
        LittleEndian::write_u64(
            &mut account_fields[JZKT_ACCOUNT_NONCE_FIELD as usize][..],
            self.nonce,
        );
        account_fields[JZKT_ACCOUNT_BALANCE_FIELD as usize]
            .copy_from_slice(&self.balance.as_le_slice());

        account_fields[JZKT_ACCOUNT_SOURCE_BYTECODE_HASH_FIELD as usize]
            .copy_from_slice(self.source_bytecode_hash.as_slice());
        account_fields[JZKT_ACCOUNT_RWASM_BYTECODE_HASH_FIELD as usize]
            .copy_from_slice(self.rwasm_bytecode_hash.as_slice());
        LittleEndian::write_u64(
            &mut account_fields[JZKT_ACCOUNT_SOURCE_BYTECODE_SIZE_FIELD as usize][..],
            self.source_bytecode_size,
        );

        account_fields
    }

    pub fn write_to_jzkt(&self) {
        let account_fields = self.get_fields();

        LowLevelSDK::jzkt_update(
            self.address.into_word().as_ptr(),
            JZKT_COMPRESSION_FLAGS,
            account_fields.as_ptr(),
            32 * account_fields.len() as u32,
        );
    }

    pub fn inc_nonce(&mut self) -> u64 {
        let prev_nonce = self.nonce;
        self.nonce += 1;
        assert_ne!(self.nonce, u64::MAX);
        prev_nonce
    }

    pub fn load_source_bytecode(&self) -> Bytes {
        let mut bytecode = vec![0u8; self.source_bytecode_size as usize];
        LowLevelSDK::jzkt_preimage_copy(self.source_bytecode_hash.as_ptr(), bytecode.as_mut_ptr());
        bytecode.into()
    }

    pub fn load_rwasm_bytecode(&self) -> Bytes {
        let mut bytecode = vec![0u8; self.rwasm_bytecode_size as usize];
        LowLevelSDK::jzkt_preimage_copy(self.rwasm_bytecode_hash.as_ptr(), bytecode.as_mut_ptr());
        bytecode.into()
    }

    pub fn update_source_bytecode(&mut self, bytecode: &Bytes) {
        let address_word = self.address.into_word();
        LowLevelSDK::crypto_keccak256(
            bytecode.as_ptr(),
            bytecode.len() as u32,
            self.source_bytecode_hash.as_mut_ptr(),
        );
        self.source_bytecode_size = bytecode.len() as u64;
        self.write_to_jzkt();
        // make sure preimage of this hash is stored
        let r = LowLevelSDK::jzkt_update_preimage(
            address_word.as_ptr(),
            JZKT_ACCOUNT_SOURCE_BYTECODE_HASH_FIELD,
            bytecode.as_ptr(),
            bytecode.len() as u32,
        );
        assert!(r, "account update_source_bytecode failed");
    }

    pub fn update_rwasm_bytecode(&mut self, bytecode: &Bytes) {
        let address_word = self.address.into_word();
        LowLevelSDK::crypto_poseidon(
            bytecode.as_ptr(),
            bytecode.len() as u32,
            self.rwasm_bytecode_hash.as_mut_ptr(),
        );
        self.rwasm_bytecode_size = bytecode.len() as u64;
        self.write_to_jzkt();
        // make sure preimage of this hash is stored
        let r = LowLevelSDK::jzkt_update_preimage(
            address_word.as_ptr(),
            JZKT_ACCOUNT_RWASM_BYTECODE_HASH_FIELD,
            bytecode.as_ptr(),
            bytecode.len() as u32,
        );
        assert!(r, "account update_bytecode failed");
    }

    pub fn checkpoint() -> AccountCheckpoint {
        LowLevelSDK::jzkt_checkpoint()
    }

    pub fn commit() -> B256 {
        let mut root = B256::ZERO;
        LowLevelSDK::jzkt_commit(root.as_mut_ptr());
        root
    }

    pub fn rollback(checkpoint: AccountCheckpoint) {
        LowLevelSDK::jzkt_rollback(checkpoint);
    }

    pub fn create_account(
        caller: &mut Account,
        callee: &mut Account,
        amount: U256,
    ) -> Result<(), ExitCode> {
        // make sure there is no creation collision
        if callee.rwasm_bytecode_hash != POSEIDON_EMPTY || callee.nonce != 0 {
            return Err(ExitCode::CreateCollision);
        }
        // change balance from caller and callee
        if let Err(exit_code) = Self::transfer(caller, callee, amount) {
            return Err(exit_code);
        }
        // change nonce (we are always on spurious dragon)
        caller.nonce = 1;
        Ok(())
    }

    pub fn sub_balance(&mut self, amount: U256) -> Result<(), ExitCode> {
        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(ExitCode::InsufficientBalance)?;
        Ok(())
    }

    pub fn sub_balance_saturating(&mut self, amount: U256) {
        self.balance = self.balance.saturating_sub(amount);
    }

    pub fn add_balance(&mut self, amount: U256) -> Result<(), ExitCode> {
        self.balance = self
            .balance
            .checked_add(amount)
            .ok_or(ExitCode::OverflowPayment)?;
        Ok(())
    }

    pub fn add_balance_saturating(&mut self, amount: U256) {
        self.balance = self.balance.saturating_add(amount);
    }

    pub fn transfer(from: &mut Account, to: &mut Account, amount: U256) -> Result<(), ExitCode> {
        // update balances
        from.sub_balance(amount)?;
        to.add_balance(amount)?;
        // commit new balances into jzkt
        from.write_to_jzkt();
        to.write_to_jzkt();
        Ok(())
    }

    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        self.nonce != 0
            || self.source_bytecode_hash != KECCAK_EMPTY
            || self.rwasm_bytecode_hash != POSEIDON_EMPTY
    }
}
