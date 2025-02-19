use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::RV64CPUContext;
use crate::wrap_j_type;

pub const JAL_OPCODE: u8 = 0b000_0000;
pub const JALR_OPCODE: u8 = 0b001_0000;
pub const BRANCH_OPCODE: u8 = 0b001_0000;



struct JalOpcodeGroup {}

struct JalrOpcodeGroup {}

struct BranchOpcodeGroup {}

type BranchExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u8);

impl ParsableInstructionGroup for JalOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        todo!()
    }
}

impl ParsableInstructionGroup for BranchOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;

        match (funct3) {
            (0x0)  => wrap_j_type!(exec_beq),
            (0x4)  => wrap_j_type!(exec_bne),
            (0x6)  => wrap_j_type!(exec_blt),
            (0x7)  => wrap_j_type!(exec_bltu),
            (0x1)  => wrap_j_type!(exec_bge),
            (0x5)  => wrap_j_type!(exec_bgeu),
            _ => wrap_j_type!(exec_unknown),
        }
    }
}