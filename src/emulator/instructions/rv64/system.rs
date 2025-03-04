use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::{Exception, PrivilegeMode, RV64CPUContext};
use crate::{wrap_b_type, wrap_b_type_u, wrap_i_type, wrap_i_type_sh, wrap_j_type, wrap_s_type};
use crate::emulator::instructions::rv64::InstructionResult;

pub const SYSTEM_OPCODE: u8 = 0b111_0011;


pub struct SystemOpcodeGroup {}


type SystemExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64);

fn exec_ecall(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    match cpu_context.csrs.get_current_privilege() {
        PrivilegeMode::Machine => Err(Exception::EnvironmentCallFromMMode),
        PrivilegeMode::Supervisor => Err(Exception::EnvironmentCallFromSMode),
        PrivilegeMode::User => Err(Exception::EnvironmentCallFromUMode)
    }
}

fn exec_ebreak(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    Err(Exception::Breakpoint)
}

fn exec_csrrw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let old_value = cpu_context.csrs.read_csr(imm as u16, false)?;

    cpu_context.set_register(rd as usize, old_value);

    let value = cpu_context.x[rs1 as usize];
    cpu_context.csrs.write_csr(imm as u16, value, false)?;

    Ok(())
}

fn exec_csrrs(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let old_value = cpu_context.csrs.read_csr(imm as u16, false)?;

    cpu_context.set_register(rd as usize, old_value);

    if(rs1 == 0) { return Ok(()) }

    let value = cpu_context.x[rs1 as usize];
    cpu_context.csrs.write_csr(imm as u16, old_value | value, false)?;

    Ok(())
}

fn exec_csrrc(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let old_value = cpu_context.csrs.read_csr(imm as u16, false)?;

    cpu_context.set_register(rd as usize, old_value);

    if(rs1 == 0) { return Ok(()) }

    let value = !cpu_context.x[rs1 as usize];
    cpu_context.csrs.write_csr(imm as u16, old_value & value, false)?;

    Ok(())
}

fn exec_csrrwi(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let old_value = cpu_context.csrs.read_csr(imm as u16, false)?;

    cpu_context.set_register(rd as usize, old_value);

    let value = rs1;
    cpu_context.csrs.write_csr(imm as u16, value as u64, false)?;

    Ok(())
}

fn exec_csrrsi(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let old_value = cpu_context.csrs.read_csr(imm as u16, false)?;

    cpu_context.set_register(rd as usize, old_value);

    if(rs1 == 0) { return Ok(()) }

    let value = rs1 as u64;
    cpu_context.csrs.write_csr(imm as u16, old_value | value, false)?;

    Ok(())
}

fn exec_csrrci(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let old_value = cpu_context.csrs.read_csr(imm as u16, false)?;

    cpu_context.set_register(rd as usize, old_value);

    if(rs1 == 0) { return Ok(()) }

    let value = !(rs1 as u64);
    cpu_context.csrs.write_csr(imm as u16, old_value & value, false)?;

    Ok(())
}

impl ParsableInstructionGroup for SystemOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;

        match (funct3) {
            0x0  => {
                let funct12 = ((instr >> 20) & 0xFFF) as u16;

                match(funct12) {
                    0x0 => wrap_i_type_sh!(exec_ecall),
                    0x1 => wrap_i_type_sh!(exec_ebreak),
                    _ => |_,_| { Err(Exception::IllegalInstruction) },
                }
            },
            0x1 => wrap_i_type_sh!(exec_csrrw),
            0x2 => wrap_i_type_sh!(exec_csrrs),
            0x3 => wrap_i_type_sh!(exec_csrrc),
            0x5 => wrap_i_type_sh!(exec_csrrwi),
            0x6 => wrap_i_type_sh!(exec_csrrsi),
            0x7 => wrap_i_type_sh!(exec_csrrci),
            _ => |_,_| { Err(Exception::IllegalInstruction) },
        }
    }
}