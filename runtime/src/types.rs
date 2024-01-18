#[cfg(feature = "main")]
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone)]
pub enum ExitCode {
    // fluentbase error codes
    ExecutionHalted = -1001,
    NotSupportedCall = -1003,
    TransactError = -1004,
    RwasmOutputOverflow = -1005,
    InputDecodeFailure = -1006,
    PoseidonError = -1007,
    // trap error codes
    UnreachableCodeReached = -2006,
    MemoryOutOfBounds = -2007,
    TableOutOfBounds = -2008,
    IndirectCallToNull = -2009,
    IntegerDivisionByZero = -2010,
    IntegerOverflow = -2011,
    BadConversionToInteger = -2012,
    StackOverflow = -2013,
    BadSignature = -2014,
    OutOfFuel = -2015,
    GrowthOperationLimited = -2016,
    UnknownError = -2017,
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "main", derive(EnumIter))]
pub enum SysFuncIdx {
    #[default]
    UNKNOWN = 0x0000,
    // SYS host functions (starts with 0x0000)
    SYS_HALT = 0x0001,       // fluentbase_v1alpha::_sys_halt
    SYS_STATE = 0x0002,      // fluentbase_v1alpha::_sys_state
    SYS_READ = 0x0003,       // fluentbase_v1alpha::_sys_read
    SYS_INPUT_SIZE = 0x0004, // fluentbase_v1alpha::_sys_input_size
    SYS_WRITE = 0x0005,      // fluentbase_v1alpha::_sys_write
    SYS_EXEC = 0x000A,       // fluentbase_v1alpha::_sys_transact
    SYS_COMPILE = 0x000B,    // fluentbase_v1alpha::_sys_compile
    // crypto functions
    CRYPTO_KECCAK256 = 0x0101, // fluentbase_v1alpha::_sys_keccak256
    CRYPTO_POSEIDON = 0x0102,  // fluentbase_v1alpha::_sys_poseidon
    CRYPTO_POSEIDON2 = 0x0103, // fluentbase_v1alpha::_sys_poseidon2
    CRYPTO_ECRECOVER = 0x0104, // fluentbase_v1alpha::_sys_ecrecover
    // zktrie functions (0x5A54 means ZT)
    ZKTRIE_OPEN = 0x0201,     // fluentbase_v1alpha::_zktrie_open
    ZKTRIE_UPDATE = 0x0202,   // fluentbase_v1alpha::_zktrie_update
    ZKTRIE_FIELD = 0x0203,    // fluentbase_v1alpha::_zktrie_field
    ZKTRIE_ROOT = 0x0204,     // fluentbase_v1alpha::_zktrie_root
    ZKTRIE_ROLLBACK = 0x0205, // fluentbase_v1alpha::_zktrie_rollback
    ZKTRIE_COMMIT = 0x0206,   // fluentbase_v1alpha::_zktrie_commit
    ZKTRIE_STORE = 0x0207,    // fluentbase_v1alpha::_zktrie_store
    ZKTRIE_LOAD = 0x0208,     // fluentbase_v1alpha::_zktrie_load
    // WASI runtime (0x5741 means WA)
    WASI_PROC_EXIT = 0x0301,         // wasi_snapshot_preview1::proc_exit
    WASI_FD_WRITE = 0x0302,          // wasi_snapshot_preview1::fd_write
    WASI_ENVIRON_SIZES_GET = 0x0303, // wasi_snapshot_preview1::environ_sizes_get
    WASI_ENVIRON_GET = 0x0304,       // wasi_snapshot_preview1::environ_get
    WASI_ARGS_SIZES_GET = 0x0305,    // wasi_snapshot_preview1::args_sizes_get
    WASI_ARGS_GET = 0x0306,          // wasi_snapshot_preview1::args_get
    // mpt trie (0x4D54 means MT)
    MPT_OPEN = 0x0401,
    MPT_UPDATE = 0x0402,
    MPT_GET = 0x0403,
    MPT_GET_ROOT = 0x0404,
}

impl SysFuncIdx {
    pub fn fuel_cost(&self) -> u32 {
        match self {
            SysFuncIdx::SYS_HALT => 1,
            SysFuncIdx::SYS_STATE => 1,
            SysFuncIdx::SYS_READ => 1,
            SysFuncIdx::SYS_INPUT_SIZE => 1,
            SysFuncIdx::SYS_WRITE => 1,
            SysFuncIdx::SYS_EXEC => 1,
            SysFuncIdx::SYS_COMPILE => 1,
            SysFuncIdx::CRYPTO_KECCAK256 => 1,
            SysFuncIdx::CRYPTO_POSEIDON => 1,
            SysFuncIdx::CRYPTO_POSEIDON2 => 1,
            SysFuncIdx::CRYPTO_ECRECOVER => 1,
            SysFuncIdx::ZKTRIE_OPEN => 1,
            SysFuncIdx::ZKTRIE_UPDATE => 1,
            SysFuncIdx::ZKTRIE_FIELD => 1,
            SysFuncIdx::ZKTRIE_ROOT => 1,
            SysFuncIdx::ZKTRIE_ROLLBACK => 1,
            SysFuncIdx::ZKTRIE_COMMIT => 1,
            SysFuncIdx::ZKTRIE_STORE => 1,
            SysFuncIdx::ZKTRIE_LOAD => 1,
            SysFuncIdx::WASI_PROC_EXIT => 1,
            SysFuncIdx::WASI_FD_WRITE => 1,
            SysFuncIdx::WASI_ENVIRON_SIZES_GET => 1,
            SysFuncIdx::WASI_ENVIRON_GET => 1,
            SysFuncIdx::WASI_ARGS_SIZES_GET => 1,
            SysFuncIdx::WASI_ARGS_GET => 1,
            SysFuncIdx::MPT_OPEN => 1,
            SysFuncIdx::MPT_UPDATE => 1,
            SysFuncIdx::MPT_GET => 1,
            SysFuncIdx::MPT_GET_ROOT => 1,
            _ => unreachable!("not configured fuel for opcode: {:?}", self),
        }
    }
}
