use crate::{
    translator::{host::Host, instructions::opcode, translator::Translator},
    utilities::{
        WASM_I64_BITS,
        WASM_I64_HIGH_32_BIT_MASK,
        WASM_I64_IN_EVM_WORD_COUNT,
        WASM_I64_LOW_32_BIT_MASK,
    },
};
use fluentbase_rwasm::rwasm::{instruction::INSTRUCTION_BYTES, InstructionSet};
use std::mem;

pub(super) fn replace_current_opcode_with_code_snippet(
    translator: &mut Translator<'_>,
    host: &mut dyn Host,
) {
    let instruction_set = host.instruction_set();
    let opcode = translator.opcode_prev();
    let mut instruction_set_replace = translator.get_code_snippet(opcode).clone();
    instruction_set_replace.fix_br_offsets(instruction_set.len() as i32 * INSTRUCTION_BYTES as i32);
    instruction_set
        .instr
        .extend(instruction_set_replace.instr.iter());
    // result postprocessing based on opcode
    const I64_STORE_OFFSET: usize = 0;
    match opcode {
        // bitwise
        opcode::BYTE
        | opcode::EQ
        | opcode::GAS
        | opcode::LT
        | opcode::GT
        | opcode::SAR
        | opcode::SGT
        | opcode::SHL
        | opcode::SHR
        | opcode::SLT
        // arithmetic
        | opcode::SUB => {
            // TODO get rid of this hack
            const OFFSET_GARBAGE_COUNT: usize = 3;
            (0..OFFSET_GARBAGE_COUNT).for_each(|_| instruction_set.op_drop());

            // const INPUT_COUNT: usize = 11;
            // (0..INPUT_COUNT).for_each(|_| instruction_set.op_drop());
            //
            // const OUTPUT_COUNT: usize = 4;
            // for i in 0..OUTPUT_COUNT {
            //     instruction_set.op_i64_const(I64_STORE_OFFSET + i * mem::size_of::<i64>());
            //     instruction_set.op_i64_load(0);
            // }
        }
        _ => {
            panic!("no postprocessing defined for 0x{:x?} opcode", opcode)
        }
    }
}

pub(super) fn duplicate_stack_value(
    instruction_set: &mut InstructionSet,
    stack_pos_shift: &mut i32,
    item_stack_pos: usize,
) {
    instruction_set.op_local_get(item_stack_pos as u32);
    *stack_pos_shift += 1;
}

pub(super) fn evm_word_param_stack_pos(
    stack_pos_shift: i32,
    part_idx: usize,
    is_b_param: bool,
    start_from_be: bool,
) -> usize {
    if start_from_be {
        WASM_I64_IN_EVM_WORD_COUNT * if is_b_param { 0 } else { 1 }
            + part_idx
            + stack_pos_shift as usize
    } else {
        WASM_I64_IN_EVM_WORD_COUNT * if is_b_param { 1 } else { 2 } - part_idx
            + stack_pos_shift as usize
    }
}

pub(super) fn duplicate_i64_part_of_evm_word(
    instruction_set: &mut InstructionSet,
    stack_pos_shift: &mut i32,
    part_idx: usize,
    is_b_param: bool,
    start_from_left: bool,
) {
    duplicate_stack_value(
        instruction_set,
        stack_pos_shift,
        evm_word_param_stack_pos(*stack_pos_shift, part_idx, is_b_param, start_from_left),
    );
}
pub(super) fn i64_shift_part(
    instruction_set: &mut InstructionSet,
    _stack_pos_shift: &mut i32,
    shift_low_high: bool,
) {
    instruction_set.op_i64_const(WASM_I64_BITS / 2);
    if shift_low_high {
        // *stack_pos_shift += 1;
        instruction_set.op_i64_shl();
    // *stack_pos_shift -= 1;
    } else {
        // *stack_pos_shift += 1;
        instruction_set.op_i64_shr_u();
        // *stack_pos_shift -= 1;
    }
}
pub(super) fn fetch_i64_part_as_i32(
    instruction_set: &mut InstructionSet,
    stack_pos_shift: &mut i32,
    drop_high_part: bool,
) {
    instruction_set.op_i64_const(if drop_high_part {
        WASM_I64_LOW_32_BIT_MASK
    } else {
        WASM_I64_HIGH_32_BIT_MASK
    });
    // *stack_pos_shift += 1;
    instruction_set.op_i64_and();
    // *stack_pos_shift -= 1;

    if !drop_high_part {
        i64_shift_part(instruction_set, stack_pos_shift, false);
    }
}
pub(super) fn wasm_add(instruction_set: &mut InstructionSet, stack_pos_shift: &mut i32) {
    instruction_set.op_i64_add();
    *stack_pos_shift -= 1;
}
pub(super) fn wasm_and(instruction_set: &mut InstructionSet, stack_pos_shift: &mut i32) {
    instruction_set.op_i64_and();
    *stack_pos_shift -= 1;
}
pub(super) fn wasm_or(instruction_set: &mut InstructionSet, stack_pos_shift: &mut i32) {
    instruction_set.op_i64_or();
    *stack_pos_shift -= 1;
}
pub(super) fn wasm_xor(instruction_set: &mut InstructionSet, stack_pos_shift: &mut i32) {
    instruction_set.op_i64_xor();
    *stack_pos_shift -= 1;
}
pub(super) fn wasm_not(instruction_set: &mut InstructionSet, _stack_pos_shift: &mut i32) {
    instruction_set.op_i64_const(-1);
    instruction_set.op_i64_sub();
}
pub(super) fn wasm_drop_n(
    instruction_set: &mut InstructionSet,
    stack_pos_shift: &mut i32,
    count: usize,
) {
    for _ in 0..count {
        instruction_set.op_drop();
    }
    *stack_pos_shift -= count as i32;
}
pub(super) fn assign_to_stack_and_drop(
    instruction_set: &mut InstructionSet,
    stack_pos_shift: &mut i32,
    stack_pos: usize,
) {
    instruction_set.op_local_set(stack_pos as u32);
    *stack_pos_shift -= 1;
}
pub(super) fn split_i64_repr_of_i32_sum_into_overflow_and_normal_parts(
    instruction_set: &mut InstructionSet,
    stack_pos_shift: &mut i32,
    do_upgrade_to_high_part: bool,
) {
    // split value onto overflow part (which is greater 0xffffffffff) and normal and them on stack
    // so overflow part is on top puts overflow value on top of the stack and normal value next
    // to it
    duplicate_stack_value(instruction_set, stack_pos_shift, 1);
    // extract overflow part
    fetch_i64_part_as_i32(instruction_set, stack_pos_shift, false);
    duplicate_stack_value(instruction_set, stack_pos_shift, 2);
    // extract normal part
    fetch_i64_part_as_i32(instruction_set, stack_pos_shift, true);
    if do_upgrade_to_high_part {
        i64_shift_part(instruction_set, stack_pos_shift, true);
    }
    // replace initial value with normal part
    instruction_set.op_local_set(3);
    *stack_pos_shift += 1;
}