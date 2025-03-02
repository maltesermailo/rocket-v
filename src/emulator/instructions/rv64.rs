use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::instructions::rv64::amo::{AtomicOpcodeGroup, ATOMIC_OPCODE};
use crate::emulator::instructions::rv64::int_op::{IntOp32OpcodeGroup, IntOpOpcodeGroup, OP_OPCODE, OP_32_OPCODE};
use crate::emulator::instructions::rv64::int_op_imm::{IntOpImmOpcodeGroup, LuiOpcodeGroup, LUI_OPCODE, AUIPC_OPCODE, OP_IMM_OPCODE, OP_IMM_32_OPCODE, AuipcOpcodeGroup, IntOpImm32OpcodeGroup};
use crate::emulator::instructions::rv64::jump_branch::{JalOpcodeGroup, JalrOpcodeGroup, JAL_OPCODE, JALR_OPCODE, BRANCH_OPCODE, BranchOpcodeGroup};
use crate::emulator::instructions::rv64::load_store::{LoadOpcodeGroup, StoreOpcodeGroup, LOAD_OPCODE, STORE_OPCODE};
use crate::emulator::instructions::rv64::system::{SystemOpcodeGroup, SYSTEM_OPCODE};
use crate::emulator::state::rv64_cpu_context::Exception;

pub mod int_op;
pub mod int_op_imm;
pub mod jump_branch;
pub mod load_store;
pub mod system;
pub mod amo;

type InstructionResult = Result<(), Exception>;

pub struct RV64InstructionParser {
}

impl RV64InstructionParser {
    pub fn parse(instr: u32) -> InstructionFn {
        let opcode: u8 = (instr & 0x7F) as u8;

        match opcode {
            OP_OPCODE => IntOpOpcodeGroup::parse(instr),
            OP_32_OPCODE => IntOp32OpcodeGroup::parse(instr),
            OP_IMM_OPCODE => IntOpImmOpcodeGroup::parse(instr),
            OP_IMM_32_OPCODE => IntOpImm32OpcodeGroup::parse(instr),
            LUI_OPCODE => LuiOpcodeGroup::parse(instr),
            AUIPC_OPCODE => AuipcOpcodeGroup::parse(instr),
            JAL_OPCODE => JalOpcodeGroup::parse(instr),
            JALR_OPCODE => JalrOpcodeGroup::parse(instr),
            BRANCH_OPCODE => BranchOpcodeGroup::parse(instr),
            LOAD_OPCODE => LoadOpcodeGroup::parse(instr),
            STORE_OPCODE => StoreOpcodeGroup::parse(instr),
            SYSTEM_OPCODE => SystemOpcodeGroup::parse(instr),
            ATOMIC_OPCODE => AtomicOpcodeGroup::parse(instr),
            _ => { |_,_| { Err(Exception::IllegalInstruction) } }
        }
    }
}

#[macro_export] macro_rules! wrap_r_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let rd = ((instr >> 7) & 0x1F) as u8;
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let rs2 = ((instr >> 20) & 0x1F) as u8;
                $exec_fn(cpu_context, instr, rd, rs1, rs2)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_j_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let rd = ((instr >> 7) & 0x1F) as u8;

                let imm110 = (((instr >> 21) & 0x3FF) as u64) << 1; //Bits 1 to 11
                let imm11 = (((instr >> 20) & 1) as u64) << 11; //Bit 11
                let imm1219 = (((instr >> 12) & 0xFF) as u64) << 12; //Bits 12 to 19
                let imm20 = (((instr >> 31) & 1) as u64) << 20; //Bit 20

                let mut imm: u64 = (imm110 | imm11 | imm1219 | imm20);

                if(((instr >> 31) & 1) > 0) {
                    imm |= !0xFFFFF_u64;
                }

                $exec_fn(cpu_context, instr, rd, imm)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_i_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let rd = ((instr >> 7) & 0x1F) as u8;
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let mut imm = ((instr >> 20)) as u64;

                //Check if signed
                if((instr & (1<<31)) > 0) {
                    //Sign extend immediate with 1
                    imm |= !0xFFF_u64;
                }

                $exec_fn(cpu_context, instr, rd, rs1, imm)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_i_type_sh {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let rd = ((instr >> 7) & 0x1F) as u8;
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let imm = ((instr >> 20)) as u64;

                $exec_fn(cpu_context, instr, rd, rs1, imm)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_b_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let imm1_4 = (((instr >> 8) & 0xF) as u64) << 1; //Bits 1 to 4
                let imm5_10 = (((instr >> 25) & 0x3F) as u64) << 5; //Bits 5 to 10
                let imm11 = (((instr >> 7) & 1) as u64) << 11; //Bit 11
                let imm12 = (((instr >> 31) & 1) as u64) << 12; //Bit 12
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let rs2 = ((instr >> 20) & 0x1F) as u8;

                let mut imm = imm1_4 | imm5_10 | imm11 | imm12;

                if(((instr >> 31) & 1) > 1) {
                    imm |= !0xFFFFF;
                }

                $exec_fn(cpu_context, instr, rs1, rs2, imm)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_b_type_u {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let imm1_4 = (((instr >> 8) & 0xF) as u64) << 1; //Bits 1 to 4
                let imm5_10 = (((instr >> 25) & 0x3F) as u64) << 5; //Bits 5 to 10
                let imm11 = (((instr >> 7) & 1) as u64) << 11; //Bit 11
                let imm12 = (((instr >> 31) & 1) as u64) << 12; //Bit 12
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let rs2 = ((instr >> 20) & 0x1F) as u8;

                let imm = imm1_4 | imm5_10 | imm11 | imm12;

                $exec_fn(cpu_context, instr, rs1, rs2, imm)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_u_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let imm = (((instr >> 12) & 0xFFFFF) as u64) << 12; //Bits 1 to 4
                let rs1 = ((instr >> 7) & 0x1F) as u8;

                $exec_fn(cpu_context, instr, rs1, imm)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_s_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) -> Result<(), Exception> {
                let imm = (((instr >> 25) & 0x3F) as u64) << 5 | (((instr >> 7) & 0x1F) as u64);
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let rs2 = ((instr >> 20) & 0x1F) as u8;

                $exec_fn(cpu_context, instr, rs1, rs2, imm)
            }
            wrapper
        }
    }
}