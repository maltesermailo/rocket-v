use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::{Exception, RV64CPUContext};
use crate::{wrap_i_type, wrap_i_type_sh, wrap_u_type};
use crate::emulator::instructions::rv64::InstructionResult;

pub const OP_IMM_OPCODE: u8 = 0b0010011;
pub const LUI_OPCODE: u8 = 0b0110111;
pub const AUIPC_OPCODE: u8 = 0b0010111;

pub struct IntOpImmOpcodeGroup {}

pub struct LuiOpcodeGroup {}
pub struct AuipcOpcodeGroup {}

type ExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> Result<(), Exception>;

fn exec_addi(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize].wrapping_add(imm));
    Ok(())
}

fn exec_xori(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] ^ imm);
    Ok(())
}

fn exec_ori(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] | imm);
    Ok(())
}

fn exec_andi(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] & imm);
    Ok(())
}

fn exec_slli(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let shift = (imm & 0x3F) as u32;  // RV64I uses 6-bit shift amount
    if shift >= 64 {
        return Err(Exception::IllegalInstruction);
    }
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] << shift);
    Ok(())
}

fn exec_srli_srai(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let shift = imm & 0x1F;  // RV64I uses 6-bit shift amount
    if shift >= 64 {
        return Err(Exception::IllegalInstruction);
    }

    let is_arith = (imm >> 5) == 0x20;

    if !is_arith {
        cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] >> shift);
    } else {
        cpu_context.set_register(rd as usize, ((cpu_context.x[rs1 as usize] as i64) >> shift) as u64);
    }
    Ok(())
}

fn exec_slti(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    cpu_context.set_register(rd as usize, if (cpu_context.x[rs1 as usize] as i64) < (imm as i64) { 1 } else { 0 });
    Ok(())
}

fn exec_sltiu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    cpu_context.set_register(rd as usize, if cpu_context.x[rs1 as usize] < imm { 1 } else { 0 });
    Ok(())
}

fn exec_lui(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, imm: u64) -> InstructionResult {
    cpu_context.set_register(rd as usize, imm);
    Ok(())
}

fn exec_auipc(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, imm: u64) -> InstructionResult {
    let old_pc = cpu_context.pc;
    cpu_context.set_register(rd as usize, old_pc.wrapping_add(imm));
    Ok(())
}

impl ParsableInstructionGroup for IntOpImmOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;

        match (funct3) {
            0x0 => wrap_i_type!(exec_addi),
            0x4  => wrap_i_type!(exec_xori),
            0x6  => wrap_i_type!(exec_ori),
            0x7  => wrap_i_type!(exec_andi),
            0x1  => wrap_i_type_sh!(exec_slli),
            0x5  => wrap_i_type_sh!(exec_srli_srai),
            0x2  => wrap_i_type!(exec_slti),
            0x3  => wrap_i_type!(exec_sltiu),
            _ => |_,_| { Err(Exception::IllegalInstruction) }
        }
    }
}

impl ParsableInstructionGroup for LuiOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        wrap_u_type!(exec_lui)
    }
}

impl ParsableInstructionGroup for AuipcOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        wrap_u_type!(exec_auipc)
    }
}