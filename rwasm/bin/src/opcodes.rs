use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    pub static ref OPCODE_NAME_TO_NUMBER: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("STOP", 0x00);
        m.insert("ADD", 0x01);
        m.insert("MUL", 0x02);
        m.insert("SUB", 0x03);
        m.insert("DIV", 0x04);
        m.insert("SDIV", 0x05);
        m.insert("MOD", 0x06);
        m.insert("SMOD", 0x07);
        m.insert("ADDMOD", 0x08);
        m.insert("MULMOD", 0x09);
        m.insert("EXP", 0x0A);
        m.insert("SIGNEXTEND", 0x0B);
        // 0x0C
        // 0x0D
        // 0x0E
        // 0x0F
        m.insert("LT", 0x10);
        m.insert("GT", 0x11);
        m.insert("SLT", 0x12);
        m.insert("SGT", 0x13);
        m.insert("EQ", 0x14);
        m.insert("ISZERO", 0x15);
        m.insert("AND", 0x16);
        m.insert("OR", 0x17);
        m.insert("XOR", 0x18);
        m.insert("NOT", 0x19);
        m.insert("BYTE", 0x1A);
        m.insert("SHL", 0x1B);
        m.insert("SHR", 0x1C);
        m.insert("SAR", 0x1D);
        // 0x1E
        // 0x1F
        m.insert("KECCAK256", 0x20);
        // 0x21
        // 0x22
        // 0x23
        // 0x24
        // 0x25
        // 0x26
        // 0x27
        // 0x28
        // 0x29
        // 0x2A
        // 0x2B
        // 0x2C
        // 0x2D
        // 0x2E
        // 0x2F
        m.insert("ADDRESS", 0x30);
        m.insert("BALANCE", 0x31);
        m.insert("ORIGIN", 0x32);
        m.insert("CALLER", 0x33);
        m.insert("CALLVALUE", 0x34);
        m.insert("CALLDATALOAD", 0x35);
        m.insert("CALLDATASIZE", 0x36);
        m.insert("CALLDATACOPY", 0x37);
        m.insert("CODESIZE", 0x38);
        m.insert("CODECOPY", 0x39);

        m.insert("GASPRICE", 0x3A);
        m.insert("EXTCODESIZE", 0x3B);
        m.insert("EXTCODECOPY", 0x3C);
        m.insert("RETURNDATASIZE", 0x3D);
        m.insert("RETURNDATACOPY", 0x3E);
        m.insert("EXTCODEHASH", 0x3F);
        m.insert("BLOCKHASH", 0x40);
        m.insert("COINBASE", 0x41);
        m.insert("TIMESTAMP", 0x42);
        m.insert("NUMBER", 0x43);
        m.insert("DIFFICULTY", 0x44);
        m.insert("GASLIMIT", 0x45);
        m.insert("CHAINID", 0x46);
        m.insert("SELFBALANCE", 0x47);
        m.insert("BASEFEE", 0x48);
        m.insert("BLOBHASH", 0x49);
        m.insert("BLOBBASEFEE", 0x4A);
        // 0x4B
        // 0x4C
        // 0x4D
        // 0x4E
        // 0x4F
        m.insert("POP", 0x50);
        m.insert("MLOAD", 0x51);
        m.insert("MSTORE", 0x52);
        m.insert("MSTORE8", 0x53);
        m.insert("SLOAD", 0x54);
        m.insert("SSTORE", 0x55);
        m.insert("JUMP", 0x56);
        m.insert("JUMPI", 0x57);
        m.insert("PC", 0x58);
        m.insert("MSIZE", 0x59);
        m.insert("GAS", 0x5A);
        m.insert("JUMPDEST", 0x5B);
        m.insert("TLOAD", 0x5C);
        m.insert("TSTORE", 0x5D);
        m.insert("MCOPY", 0x5E);

        m.insert("PUSH0", 0x5F);
        m.insert("PUSH1", 0x60);
        m.insert("PUSH2", 0x61);
        m.insert("PUSH3", 0x62);
        m.insert("PUSH4", 0x63);
        m.insert("PUSH5", 0x64);
        m.insert("PUSH6", 0x65);
        m.insert("PUSH7", 0x66);
        m.insert("PUSH8", 0x67);
        m.insert("PUSH9", 0x68);
        m.insert("PUSH10", 0x69);
        m.insert("PUSH11", 0x6A);
        m.insert("PUSH12", 0x6B);
        m.insert("PUSH13", 0x6C);
        m.insert("PUSH14", 0x6D);
        m.insert("PUSH15", 0x6E);
        m.insert("PUSH16", 0x6F);
        m.insert("PUSH17", 0x70);
        m.insert("PUSH18", 0x71);
        m.insert("PUSH19", 0x72);
        m.insert("PUSH20", 0x73);
        m.insert("PUSH21", 0x74);
        m.insert("PUSH22", 0x75);
        m.insert("PUSH23", 0x76);
        m.insert("PUSH24", 0x77);
        m.insert("PUSH25", 0x78);
        m.insert("PUSH26", 0x79);
        m.insert("PUSH27", 0x7A);
        m.insert("PUSH28", 0x7B);
        m.insert("PUSH29", 0x7C);
        m.insert("PUSH30", 0x7D);
        m.insert("PUSH31", 0x7E);
        m.insert("PUSH32", 0x7F);

        m.insert("DUP1", 0x80);
        m.insert("DUP2", 0x81);
        m.insert("DUP3", 0x82);
        m.insert("DUP4", 0x83);
        m.insert("DUP5", 0x84);
        m.insert("DUP6", 0x85);
        m.insert("DUP7", 0x86);
        m.insert("DUP8", 0x87);
        m.insert("DUP9", 0x88);
        m.insert("DUP10", 0x89);
        m.insert("DUP11", 0x8A);
        m.insert("DUP12", 0x8B);
        m.insert("DUP13", 0x8C);
        m.insert("DUP14", 0x8D);
        m.insert("DUP15", 0x8E);
        m.insert("DUP16", 0x8F);

        m.insert("SWAP1", 0x90);
        m.insert("SWAP2", 0x91);
        m.insert("SWAP3", 0x92);
        m.insert("SWAP4", 0x93);
        m.insert("SWAP5", 0x94);
        m.insert("SWAP6", 0x95);
        m.insert("SWAP7", 0x96);
        m.insert("SWAP8", 0x97);
        m.insert("SWAP9", 0x98);
        m.insert("SWAP10", 0x99);
        m.insert("SWAP11", 0x9A);
        m.insert("SWAP12", 0x9B);
        m.insert("SWAP13", 0x9C);
        m.insert("SWAP14", 0x9D);
        m.insert("SWAP15", 0x9E);
        m.insert("SWAP16", 0x9F);

        m.insert("LOG0", 0xA0);
        m.insert("LOG1", 0xA1);
        m.insert("LOG2", 0xA2);
        m.insert("LOG3", 0xA3);
        m.insert("LOG4", 0xA4);
        // 0xA5
        // 0xA6
        // 0xA7
        // 0xA8
        // 0xA9
        // 0xAA
        // 0xAB
        // 0xAC
        // 0xAD
        // 0xAE
        // 0xAF
        // 0xB0
        // 0xB1
        // 0xB2
        // 0xB3
        // 0xB4
        // 0xB5
        // 0xB6
        // 0xB7
        // 0xB8
        // 0xB9
        // 0xBA
        // 0xBB
        // 0xBC
        // 0xBD
        // 0xBE
        // 0xBF
        // 0xC0
        // 0xC1
        // 0xC2
        // 0xC3
        // 0xC4
        // 0xC5
        // 0xC6
        // 0xC7
        // 0xC8
        // 0xC9
        // 0xCA
        // 0xCB
        // 0xCC
        // 0xCD
        // 0xCE
        // 0xCF
        // 0xD0
        // 0xD1
        // 0xD2
        // 0xD3
        // 0xD4
        // 0xD5
        // 0xD6
        // 0xD7
        // 0xD8
        // 0xD9
        // 0xDA
        // 0xDB
        // 0xDC
        // 0xDD
        // 0xDE
        // 0xDF
        // 0xE0
        // 0xE1
        // 0xE2
        // 0xE3
        // 0xE4
        // 0xE5
        // 0xE6
        // 0xE7
        // 0xE8
        // 0xE9
        // 0xEA
        // 0xEB
        // 0xEC
        // 0xED
        // 0xEE
        // 0xEF
        m.insert("CREATE", 0xF0);
        m.insert("CALL", 0xF1);
        m.insert("CALLCODE", 0xF2);
        m.insert("RETURN", 0xF3);
        m.insert("DELEGATECALL", 0xF4);
        m.insert("CREATE2", 0xF5);
        // 0xF6
        // 0xF7
        // 0xF8
        // 0xF9
        m.insert("STATICCALL", 0xFA);
        // 0xFB
        // 0xFC
        m.insert("REVERT", 0xFD);
        m.insert("INVALID", 0xFE);
        m.insert("SELFDESTRUCT", 0xFF);

        m
    };
}