use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::RV64CPUContext;
use crate::{wrap_b_type, wrap_b_type_u, wrap_i_type, wrap_j_type};

pub const JAL_OPCODE: u8 = 0b110_1111;
pub const JALR_OPCODE: u8 = 0b110_0111;
pub const BRANCH_OPCODE: u8 = 0b110_0011;



struct JalOpcodeGroup {}

struct JalrOpcodeGroup {}

struct BranchOpcodeGroup {}

type BranchExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64);
type JalExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, imm: u64);
type JalrExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64);

fn exec_jal(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, imm: u64) {
    let old_pc = cpu_context.pc;

    cpu_context.set_register(rd as usize, old_pc + 4);

    cpu_context.pc = (old_pc.wrapping_add(imm));
}

fn exec_jalr(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) {
    cpu_context.set_register(rs1 as usize, (cpu_context.x[rs1 as usize].wrapping_add(imm)) & 0xfffffffffffffffe_u64);

    let old_pc = cpu_context.pc;

    cpu_context.pc = cpu_context.x[rs1 as usize];
    cpu_context.set_register(rd as usize, old_pc + 4);
}

fn exec_beq(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) {
    if cpu_context.x[rs1 as usize] == cpu_context.x[rs2 as usize] {
        cpu_context.pc = (cpu_context.pc.wrapping_add(imm));
    }
}

fn exec_bne(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) {
    if cpu_context.x[rs1 as usize] != cpu_context.x[rs2 as usize] {
        cpu_context.pc = (cpu_context.pc.wrapping_add(imm));
    }
}

fn exec_blt(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) {
    if (cpu_context.x[rs1 as usize] as i64) < (cpu_context.x[rs2 as usize] as i64) {
        cpu_context.pc = (cpu_context.pc.wrapping_add(imm));
    }
}

fn exec_bltu(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) {
    if cpu_context.x[rs1 as usize] < cpu_context.x[rs2 as usize] {
        cpu_context.pc = (cpu_context.pc.wrapping_add(imm));
    }
}

fn exec_bge(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) {
    if (cpu_context.x[rs1 as usize] as i64) >= (cpu_context.x[rs2 as usize] as i64) {
        cpu_context.pc = (cpu_context.pc.wrapping_add(imm));
    }
}

fn exec_bgeu(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) {
    if cpu_context.x[rs1 as usize] >= cpu_context.x[rs2 as usize] {
        cpu_context.pc = (cpu_context.pc.wrapping_add(imm));
    }
}

impl ParsableInstructionGroup for JalOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        wrap_j_type!(exec_jal)
    }
}

impl ParsableInstructionGroup for JalrOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        wrap_i_type!(exec_jalr)
    }
}

impl ParsableInstructionGroup for BranchOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;

        match (funct3) {
            0x0  => wrap_b_type!(exec_beq),
            0x1  => wrap_b_type!(exec_bne),
            0x4  => wrap_b_type!(exec_blt),
            0x6  => wrap_b_type!(exec_bltu),
            0x5  => wrap_b_type!(exec_bge),
            0x7  => wrap_b_type!(exec_bgeu),
            _ => |_,_| {},
        }
    }
}