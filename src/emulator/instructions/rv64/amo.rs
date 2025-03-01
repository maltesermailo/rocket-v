use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::{Exception, PrivilegeMode, RV64CPUContext};
use crate::{wrap_b_type, wrap_b_type_u, wrap_i_type, wrap_i_type_sh, wrap_j_type, wrap_s_type};
use crate::emulator::instructions::rv64::InstructionResult;

pub const ATOMIC_OPCODE: u8 = 0b010_1111;


pub struct AtomicOpcodeGroup {}


type AtomicExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64);



impl ParsableInstructionGroup for AtomicOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;
        let funct5 = ((instr >> 27) & 0x1F) as u8;

        match (funct3, funct5) {
            (0x2, 0x2) => wrap_i_type!(exec_lr_w),
            (0x3, 0x2) => wrap_i_type!(exec_sc_w),
            (0x2, 0x3) => wrap_i_type!(exec_amoswap_w),
            (0x3, 0x3) => wrap_i_type!(exec_amoadd_w),
            (0x4, 0x3) => wrap_i_type!(exec_amoxor_w),
            (0x0, 0x3) => wrap_i_type!(exec_amoor_w),
            (0x1, 0x3) => wrap_i_type!(exec_amoand_w),
            (0x5, 0x3) => wrap_i_type!(exec_amomin_w),
            (0x6, 0x3) => wrap_i_type!(exec_amomax_w),
            (0x7, 0x3) => wrap_i_type!(exec_amominu_w),
            (0x8, 0x3) => wrap_i_type!(exec_amomaxu_w),
            _ => |_,_| { Err(Exception::IllegalInstruction) },
        }
    }
}