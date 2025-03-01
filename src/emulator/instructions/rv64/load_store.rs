use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::{Exception, RV64CPUContext};
use crate::{wrap_b_type, wrap_b_type_u, wrap_i_type, wrap_i_type_sh, wrap_j_type, wrap_s_type};
use crate::emulator::instructions::rv64::InstructionResult;
use crate::emulator::state::memory::Device;

pub const LOAD_OPCODE: u8 = 0b0000011;
pub const STORE_OPCODE: u8 = 0b0100011;



pub struct LoadOpcodeGroup {}

pub struct StoreOpcodeGroup {}


type LoadExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64);
type StoreExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, imm: u64);

fn exec_load_byte(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    let mut value: u64 = cpu_context.memory.read().unwrap().read_byte(address as usize) as u64;

    if (value & 0x80) > 0 {
        value |= !0xFF_u64;
    }

    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_load_hword(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 1 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    let mut value: u64 = cpu_context.memory.read().unwrap().read_half_word(address as usize) as u64;

    if (value & 0x8000) > 0 {
        value |= !0xFFFF_u64;
    }

    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_load_word(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 3 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    let mut value: u64 = cpu_context.memory.write().unwrap().read_word(address as usize) as u64;

    if (value & 0x80000000) > 0 {
        value |= !0xFFFFFFFF_u64;
    }

    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_load_dword(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 7 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    let value: u64 = cpu_context.memory.read().unwrap().read_double_word(address as usize);
    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_load_byte_unsigned(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    let value: u64 = cpu_context.memory.read().unwrap().read_byte(address as usize) as u64;
    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_load_hword_unsigned(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 1 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    let value: u64 = cpu_context.memory.read().unwrap().read_half_word(address as usize) as u64;
    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_load_word_unsigned(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 3 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    let value: u64 = cpu_context.memory.read().unwrap().read_word(address as usize) as u64;
    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_store_byte(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::StoreAccessFault);
    }

    cpu_context.memory.write().unwrap().write_byte(address as usize, cpu_context.x[rs2 as usize] as u8);
    Ok(())
}

fn exec_store_half_word(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 1 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::StoreAccessFault);
    }

    cpu_context.memory.write().unwrap().write_half_word(address as usize, cpu_context.x[rs2 as usize] as u16);
    Ok(())
}

fn exec_store_word(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 3 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::StoreAccessFault);
    }

    cpu_context.memory.write().unwrap().write_word(address as usize, cpu_context.x[rs2 as usize] as u32);
    Ok(())
}

fn exec_store_dword(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 7 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::StoreAccessFault);
    }

    cpu_context.memory.write().unwrap().write_double_word(address as usize, cpu_context.x[rs2 as usize]);
    Ok(())
}

impl ParsableInstructionGroup for LoadOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;

        match (funct3) {
            0x0  => wrap_i_type!(exec_load_byte),
            0x1  => wrap_i_type!(exec_load_hword),
            0x2  => wrap_i_type!(exec_load_word),
            0x3  => wrap_i_type!(exec_load_dword),
            0x4  => wrap_i_type_sh!(exec_load_byte_unsigned),
            0x5  => wrap_i_type_sh!(exec_load_hword_unsigned),
            0x6  => wrap_i_type_sh!(exec_load_word_unsigned),
            _ => |_,_| { Err(Exception::IllegalInstruction) },
        }
    }
}

impl ParsableInstructionGroup for StoreOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;

        match (funct3) {
            0x0  => wrap_s_type!(exec_store_byte),
            0x1  => wrap_s_type!(exec_store_half_word),
            0x2  => wrap_s_type!(exec_store_word),
            0x3  => wrap_s_type!(exec_store_dword),
            _ => |_,_| { Err(Exception::IllegalInstruction) },
        }
    }
}