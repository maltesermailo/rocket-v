use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::RV64CPUContext;
use crate::{wrap_i_type, wrap_i_type_sh};

pub const OP_IMM_OPCODE: u8 = 0b0010011;

struct IntOpImmOpcodeGroup {

}

type ExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64);


fn exec_addi(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] + imm;
}

fn exec_xori(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] ^ imm;
}

fn exec_ori(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] | imm;
}

fn exec_andi(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] & imm;
}

fn exec_slli(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    let shift = (imm & 0x1F) as u32;

    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] << shift;
}

fn exec_srli_srai(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    let shift = (imm & 0x1F) as u32;
    let is_arith = (imm >> 5) == 0x20;

    if(!is_arith) {
        cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] >> shift;
    } else {
        cpu_context.x[rd as usize] = (cpu_context.x[rs1 as usize] as i64 >> shift) as u64;
    }
}
fn exec_slti(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    cpu_context.x[rd as usize] = if((cpu_context.x[rs1 as usize] as i64) < (imm as i64)) { 1 } else { 0 };
}

fn exec_sltiu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    cpu_context.x[rd as usize] = if(cpu_context.x[rs1 as usize] < imm) { 1 } else { 0 };
}

fn exec_unknown(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {

}


impl ParsableInstructionGroup for IntOpImmOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;
        let funct7 = ((instr >> 25) & 0x7F) as u8;

        match (funct3, funct7) {
            (0x0, 0x0)  => wrap_i_type!(exec_addi),
            (0x4, 0x0)  => wrap_i_type!(exec_xori),
            (0x6, 0x0)  => wrap_i_type!(exec_ori),
            (0x7, 0x0)  => wrap_i_type!(exec_andi),
            (0x1, 0x0)  => wrap_i_type_sh!(exec_slli),
            (0x5, 0x0)  => wrap_i_type_sh!(exec_srli_srai),
            (0x2, 0x0)  => wrap_i_type!(exec_slti),
            (0x3, 0x0)  => wrap_i_type!(exec_sltiu),
            _ => wrap_i_type!(exec_unknown)
        }
    }
}