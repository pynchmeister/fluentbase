use crate::{
    instruction::exported_memory_vec,
    zktrie_helpers::account_data_from_bytes,
    RuntimeContext,
};
use fluentbase_rwasm::{common::Trap, Caller};
use halo2curves::{bn256::Fr, group::ff::PrimeField};
use lazy_static::lazy_static;
use poseidon::Poseidon;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};
use zktrie::{AccountData, Hash, StoreData, ZkMemoryDb, ZkTrie, ACCOUNTSIZE, FIELDSIZE};

pub const KEYSIZE: usize = 20;
pub type KeyData = [u8; KEYSIZE];
static FILED_ERROR_READ: &str = "invalid input field";
static FILED_ERROR_OUT: &str = "output field fail";

extern "C" fn hash_scheme(
    a: *const u8,
    b: *const u8,
    domain: *const u8,
    out: *mut u8,
) -> *const i8 {
    use std::slice;
    let a: [u8; 32] =
        TryFrom::try_from(unsafe { slice::from_raw_parts(a, 32) }).expect("length specified");
    let b: [u8; 32] =
        TryFrom::try_from(unsafe { slice::from_raw_parts(b, 32) }).expect("length specified");
    let domain: [u8; 32] =
        TryFrom::try_from(unsafe { slice::from_raw_parts(domain, 32) }).expect("length specified");
    let out = unsafe { slice::from_raw_parts_mut(out, 32) };

    let fa = Fr::from_bytes(&a);
    let fa = if fa.is_some().into() {
        fa.unwrap()
    } else {
        return FILED_ERROR_READ.as_ptr().cast();
    };
    let fb = Fr::from_bytes(&b);
    let fb = if fb.is_some().into() {
        fb.unwrap()
    } else {
        return FILED_ERROR_READ.as_ptr().cast();
    };
    let fdomain = Fr::from_bytes(&domain);
    let _fdomain = if fdomain.is_some().into() {
        fdomain.unwrap()
    } else {
        return FILED_ERROR_READ.as_ptr().cast();
    };

    let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 56);
    hasher.update(&[fa, fb]);
    let h = hasher.squeeze();

    let repr_h = h.to_repr();
    if repr_h.len() == 32 {
        out.copy_from_slice(repr_h.as_ref());
        std::ptr::null()
    } else {
        FILED_ERROR_OUT.as_ptr().cast()
    }
}

lazy_static! {
    /// Use this boolean to initialize the hash scheme.
    pub static ref HASH_SCHEME_DONE: bool = {
        zktrie::init_hash_scheme(hash_scheme);
        true
    };
}

type TrieId = i32;
const TRIE_ID_DEFAULT: i32 = 1;
thread_local! {
    static ZK_MEMORY_DB: RefCell<ZkMemoryDb> = RefCell::new(ZkMemoryDb::new());
    static LAST_TRIE_ID: RefCell<TrieId> = RefCell::new(1);
    static TRIES: RefCell<HashMap<TrieId, Rc<RefCell<ZkTrie>>>> = RefCell::new(HashMap::new());
}

pub(crate) fn zktrie_open(
    mut caller: Caller<'_, RuntimeContext>,
    root_offset: i32,
    root_len: i32,
    keys_offset: i32,
    leafs_offset: i32,
    accounts_count: i32,
) -> Result<(), Trap> {
    ZK_MEMORY_DB.with(|db| {
        let committed_root: Hash =
            exported_memory_vec(&mut caller, root_offset as usize, root_len as usize)
                .try_into()
                .unwrap();
        let keys_data = exported_memory_vec(
            &mut caller,
            keys_offset as usize,
            accounts_count as usize * KEYSIZE,
        );
        let leafs_data = exported_memory_vec(
            &mut caller,
            leafs_offset as usize,
            accounts_count as usize * ACCOUNTSIZE,
        );
        let root_zero: Hash = [0; FIELDSIZE];
        let mut zk_trie: ZkTrie = db
            .borrow_mut()
            .new_trie(&root_zero)
            .ok_or(Trap::new("failed to init new trie"))?;
        let trie_id;
        trie_id = LAST_TRIE_ID.take();
        if trie_id != TRIE_ID_DEFAULT {
            return Err(Trap::new("only 1 trie allowed at the moment"));
        }

        // fill the trie with dump data
        for i in 0..accounts_count {
            let key_offset = i as usize * KEYSIZE;
            let leaf_offset = i as usize * ACCOUNTSIZE;
            let key_data: KeyData = keys_data[key_offset..key_offset + KEYSIZE]
                .try_into()
                .map_err(|_| Trap::new("key data wrong len"))?;
            let account_data: AccountData =
                account_data_from_bytes(&leafs_data[leaf_offset..leaf_offset + ACCOUNTSIZE])?;
            zk_trie
                .update_account(&key_data, &account_data)
                .map_err(|v| Trap::new(v.to_string()))?;
        }
        if zk_trie.root() != committed_root {
            return Err(Trap::new("computed root != committed root"));
        }

        TRIES.with_borrow_mut(|m| m.insert(trie_id, Rc::new(RefCell::new(zk_trie))));
        LAST_TRIE_ID.with_borrow_mut(|v| *v += 1);

        return Ok(());
    })
}

pub(crate) fn zktrie_get(id: &TrieId) -> Result<Rc<RefCell<ZkTrie>>, Trap> {
    TRIES.with(|t| {
        let tries = t.take();
        let v = tries.get(id).clone();
        if let Some(t) = v {
            return Ok(t.clone());
        }
        Err(Trap::new("not found"))
    })
}

fn get_account_data(key: &[u8], trie: RefMut<ZkTrie>) -> Option<AccountData> {
    let acc_data = trie.get_account(&key);
    if let Some(ad) = acc_data {
        return Some(ad);
    }
    return None;
}

fn get_store_data(key: &[u8], trie: RefMut<ZkTrie>) -> Option<StoreData> {
    let acc_data = trie.get_store(&key);
    if let Some(ad) = acc_data {
        return Some(ad);
    }
    return None;
}

macro_rules! impl_update {
    ($fn_name:ident, $data_extractor:ident, $field_updater:ident, $trie_updater:ident, ) => {
        pub fn $fn_name(
            mut caller: Caller<'_, RuntimeContext>,
            key_offset: i32,
            value_offset: i32,
        ) -> Result<(), Trap> {
            let key = exported_memory_vec(&mut caller, key_offset as usize, FIELDSIZE);
            let value = exported_memory_vec(&mut caller, value_offset as usize, FIELDSIZE);

            let trie = zktrie_get(&TRIE_ID_DEFAULT)?;
            let data = $data_extractor(&key, trie.borrow_mut());
            let mut data = data.unwrap_or(Default::default());
            let mut field_array: [u8; FIELDSIZE] = [0; FIELDSIZE];
            // BE or LE?
            field_array[FIELDSIZE - value.len()..FIELDSIZE].copy_from_slice(value.as_slice());
            $field_updater(&mut data, &field_array);
            let res = trie.borrow_mut().$trie_updater(&key, &data);
            if res.is_err() {
                return Err(Trap::new(format!(
                    "failed to update value: {}",
                    res.err().unwrap().to_string()
                )));
            }

            Ok(())
        }
    };
}

macro_rules! impl_get {
    ($fn_name:ident, $data_extractor:ident, $data_fetcher:ident, ) => {
        pub fn $fn_name(
            mut caller: Caller<'_, RuntimeContext>,
            key_offset: i32,
            output_offset: i32,
        ) -> Result<(), Trap> {
            let key = exported_memory_vec(&mut caller, key_offset as usize, FIELDSIZE);

            let trie = zktrie_get(&TRIE_ID_DEFAULT)?;
            let data = $data_extractor(&key, trie.borrow_mut());
            if !data.is_some() {
                return Err(Trap::new(format!("failed to get value")));
            }

            let data = $data_fetcher(&data.unwrap());
            caller.write_memory(output_offset as usize, data.as_slice());

            Ok(())
        }
    };
}

fn update_nonce(data: &mut AccountData, v: &[u8; FIELDSIZE]) {
    data[0] = *v;
}
fn fetch_nonce(data: &AccountData) -> [u8; FIELDSIZE] {
    data[0]
}

fn update_balance(data: &mut AccountData, v: &[u8; FIELDSIZE]) {
    data[1] = *v;
}
fn fetch_balance(data: &AccountData) -> [u8; FIELDSIZE] {
    data[1]
}

fn update_storage_root(data: &mut AccountData, v: &[u8; FIELDSIZE]) {
    data[2] = *v;
}
fn fetch_storage_root(data: &AccountData) -> [u8; FIELDSIZE] {
    data[2]
}

fn update_code_hash(data: &mut AccountData, v: &[u8; FIELDSIZE]) {
    data[3] = *v;
}
fn fetch_code_hash(data: &AccountData) -> [u8; FIELDSIZE] {
    data[3]
}

fn update_code_size(data: &mut AccountData, v: &[u8; FIELDSIZE]) {
    data[4] = *v;
}
fn fetch_code_size(data: &AccountData) -> [u8; FIELDSIZE] {
    data[4]
}

fn update_store(data: &mut StoreData, v: &[u8; FIELDSIZE]) {
    *data = *v;
}
fn fetch_store(data: &StoreData) -> [u8; FIELDSIZE] {
    *data
}

impl_update!(
    zktrie_update_nonce,
    get_account_data,
    update_nonce,
    update_account,
);
impl_update!(
    zktrie_update_balance,
    get_account_data,
    update_balance,
    update_account,
);
impl_update!(
    zktrie_update_storage_root,
    get_account_data,
    update_storage_root,
    update_account,
);
impl_update!(
    zktrie_update_code_hash,
    get_account_data,
    update_code_hash,
    update_account,
);
impl_update!(
    zktrie_update_code_size,
    get_account_data,
    update_code_size,
    update_account,
);

// account gets
impl_get!(zktrie_get_nonce, get_account_data, fetch_nonce,);
impl_get!(zktrie_get_balance, get_account_data, fetch_balance,);
impl_get!(
    zktrie_get_storage_root,
    get_account_data,
    fetch_storage_root,
);
impl_get!(zktrie_get_code_hash, get_account_data, fetch_code_hash,);
impl_get!(zktrie_get_code_size, get_account_data, fetch_code_size,);

// store updates
impl_update!(
    zktrie_update_store,
    get_store_data,
    update_store,
    update_store,
);

// store gets
impl_get!(zktrie_get_store, get_store_data, fetch_store,);