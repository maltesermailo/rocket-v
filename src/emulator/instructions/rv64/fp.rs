use std::ops::{Div, Mul};
use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::{CSRAddress, Exception, FFlags, PrivilegeMode, RV64CPUContext};
use crate::{wrap_b_type, wrap_b_type_u, wrap_i_type, wrap_i_type_sh, wrap_j_type, wrap_r_type, wrap_s_type};
use crate::emulator::instructions::rv64::InstructionResult;

pub const LOAD_FP_OPCODE: u8 = 0b111_0011;
pub const STORE_FP_OPCODE: u8 = 0b111_0011;
pub const OP_FP_OPCODE: u8 = 0b101_0011;


pub struct FloatingPointOpcodeGroup {}
pub struct LoadFloatingPointOpcodeGroup {}
pub struct StoreFloatingPointOpcodeGroup {}


type FloatingExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8);

#[inline(always)]
fn apply_rounding_mode(value: f64, mode: u8) -> f64 {
    match mode {
        0 => value,
        1 => value.trunc(),
        3 => value.ceil(),
        2 => value.floor(),
        _ => value.round(),
    }
}

fn update_fp_flags(cpu_context: &mut RV64CPUContext, op: &str, rs1_val: f64, rs2_val: f64, result: f64, is_f32: bool) -> Result<(), Exception> {
    let mut flags: u64 = 0;

    if is_f32 {
        // Convert to f32 for proper flag detection at single precision
        let rs1_f32 = rs1_val as f32;
        let rs2_f32 = rs2_val as f32;
        let result_f32 = result as f32;

        // Check for Invalid Operation (NV)
        if result_f32.is_nan() && !rs1_f32.is_nan() && !rs2_f32.is_nan() {
            flags |= FFlags::NV.bits();
        }

        // Check for Divide by Zero (DZ)
        if op == "div" && rs2_f32 == 0.0 {
            flags |= FFlags::DZ.bits();
        }

        // Check for Overflow (OF)
        if result_f32.is_infinite() && !rs1_f32.is_infinite() && !rs2_f32.is_infinite() {
            flags |= FFlags::OF.bits();
        }

        // Check for Underflow (UF)
        if let core::num::FpCategory::Subnormal = result_f32.classify() {
            flags |= FFlags::UF.bits();
        }
    } else {
        // Double precision checks
        // Check for Invalid Operation (NV)
        if result.is_nan() && !rs1_val.is_nan() && !rs2_val.is_nan() {
            flags |= FFlags::NV.bits();
        }

        // Check for Divide by Zero (DZ)
        if op == "div" && rs2_val == 0.0 {
            flags |= FFlags::DZ.bits();
        }

        // Check for Overflow (OF)
        if result.is_infinite() && !rs1_val.is_infinite() && !rs2_val.is_infinite() {
            flags |= FFlags::OF.bits();
        }

        // Check for Underflow (UF)
        if let core::num::FpCategory::Subnormal = result.classify() {
            flags |= FFlags::UF.bits();
        }
    }

    // If any flags were set, write them to FFLAGS CSR (OR with existing flags)
    if flags != 0 {
        let current_flags = cpu_context.csrs.read_csr(CSRAddress::FFlags as u16, false)?;
        cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, current_flags | flags, false)?;
    }

    Ok(())
}

fn exec_load_fp(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::LoadAccessFault);
    }

    match (instr >> 12) & 0x7 {
        0x2 => {
            let mut value: u64 = cpu_context.memory.read().unwrap().read_word(address as usize) as u64;

            if (value & 0x80000000) > 0 {
                value |= !0xFFFFFFFF_u64;
            }

            cpu_context.set_register_float(rd as usize, value as f32 as f64);
            Ok(())
        }
        0x3 => {
            let value: f64 = cpu_context.memory.read().unwrap().read_double_word(address as usize) as f64;

            cpu_context.set_register_float(rd as usize, value);
            Ok(())
        }
        _ => { Err(Exception::IllegalInstruction) }
    }
}

fn exec_store_fp(cpu_context: &mut RV64CPUContext, instr: u32, rs1: u8, rs2: u8, imm: u64) -> InstructionResult {
    let address = cpu_context.x[rs1 as usize].wrapping_add(imm);

    if address + 7 >= cpu_context.memory.read().unwrap().size() {
        return Err(Exception::StoreAccessFault);
    }

    match (instr >> 12) & 0x7 {
        0x2 => {
            cpu_context.memory.write().unwrap().write_word(address as usize, cpu_context.f[rs2 as usize] as u32);

            Ok(())
        }
        0x3 => {
            cpu_context.memory.write().unwrap().write_double_word(address as usize, cpu_context.f[rs2 as usize] as u64);

            Ok(())
        }
        _ => { Err(Exception::IllegalInstruction) }
    }
}

fn exec_fadd(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> InstructionResult {
    let mut rm = ((instr >> 12) & 0x7) as u8;

    if(rm == 16) {
        //Dynamic rounding
        rm = cpu_context.csrs.read_csr(CSRAddress::FRM as u16, false)? as u8;
    }

    match (instr >> 25) & 0x3 {
        0x0 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode((cpu_context.f[rs1 as usize] as f32 + cpu_context.f[rs2 as usize] as f32) as f64, rm));

            update_fp_flags(cpu_context, "add", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], true)?;
        }
        0x1 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode((cpu_context.f[rs1 as usize] as f32 + cpu_context.f[rs2 as usize] as f32) as f64, rm));

            update_fp_flags(cpu_context, "add", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)? }
    }

    Ok(())
}

fn exec_fsub(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> InstructionResult {
    let mut rm = ((instr >> 12) & 0x7) as u8;

    if(rm == 16) {
        //Dynamic rounding
        rm = cpu_context.csrs.read_csr(CSRAddress::FRM as u16, false)? as u8;
    }

    match (instr >> 25) & 0x3 {
        0x0 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode((cpu_context.f[rs1 as usize] as f32 - cpu_context.f[rs2 as usize] as f32) as f64, rm));

            update_fp_flags(cpu_context, "sub", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], true)?;
        }
        0x1 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode((cpu_context.f[rs1 as usize] as f32 - cpu_context.f[rs2 as usize] as f32) as f64, rm));

            update_fp_flags(cpu_context, "sub", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)? }
    }

    Ok(())
}

fn exec_fmul(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let mut rm = ((instr >> 12) & 0x7) as u8;

    if(rm == 16) {
        //Dynamic rounding
        rm = cpu_context.csrs.read_csr(CSRAddress::FRM as u16, false)? as u8;
    }

    match (instr >> 25) & 0x3 {
        0x0 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode((cpu_context.f[rs1 as usize] as f32).mul(cpu_context.f[rs2 as usize] as f32) as f64, rm));

            update_fp_flags(cpu_context, "fmul", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], true)?;
        }
        0x1 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode(cpu_context.f[rs1 as usize].mul(cpu_context.f[rs2 as usize]), rm));

            update_fp_flags(cpu_context, "fmul", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)? }
    }

    Ok(())
}

fn exec_fdiv(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let mut rm = ((instr >> 12) & 0x7) as u8;

    if(rm == 16) {
        //Dynamic rounding
        rm = cpu_context.csrs.read_csr(CSRAddress::FRM as u16, false)? as u8;
    }

    match (instr >> 25) & 0x3 {
        0x0 => {
            if(cpu_context.f[rs2 as usize] as f32 == 0f32) {
                update_fp_flags(cpu_context, "div", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], true)?;

                return Ok(());
            }

            cpu_context.set_register_float(rd as usize, apply_rounding_mode((cpu_context.f[rs1 as usize] as f32).div(cpu_context.f[rs2 as usize] as f32) as f64, rm));

            update_fp_flags(cpu_context, "div", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], true)?;
        }
        0x1 => {
            if(cpu_context.f[rs2 as usize] == 0f64) {
                update_fp_flags(cpu_context, "div", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;

                return Ok(());
            }

            cpu_context.set_register_float(rd as usize, apply_rounding_mode(cpu_context.f[rs1 as usize].div(cpu_context.f[rs2 as usize]), rm));
            update_fp_flags(cpu_context, "div", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)? }
    }

    Ok(())
}

fn exec_fsqrt(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let mut rm = ((instr >> 12) & 0x7) as u8;

    if(rm == 16) {
        //Dynamic rounding
        rm = cpu_context.csrs.read_csr(CSRAddress::FRM as u16, false)? as u8;
    }

    match (instr >> 25) & 0x3 {
        0x0 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode((cpu_context.f[rs1 as usize] as f32).sqrt() as f64, rm));
            update_fp_flags(cpu_context, "sqrt", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        0x1 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode(cpu_context.f[rs1 as usize].sqrt(), rm));
            update_fp_flags(cpu_context, "sqrt", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)? }
    }

    Ok(())
}

fn exec_fminmax(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let mode = ((instr >> 12) & 0x7) as u8;

    match (instr >> 25) & 0x3 {
        0x0 => {
            cpu_context.set_register_float(rd as usize, match mode {
                0x0 => (cpu_context.f[rs1 as usize] as f32).min(cpu_context.f[rs2 as usize] as f32) as f64,
                0x1 => (cpu_context.f[rs1 as usize] as f32).max(cpu_context.f[rs2 as usize] as f32) as f64,
                _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)?;
                    return Ok(());
                }
            });
            update_fp_flags(cpu_context, "minmax", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        0x1 => {
            cpu_context.set_register_float(rd as usize, match mode {
                0x0 => cpu_context.f[rs1 as usize].min(cpu_context.f[rs2 as usize]),
                0x1 => cpu_context.f[rs1 as usize].max(cpu_context.f[rs2 as usize]),
                _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)?;
                    return Ok(());
                }
            });
            update_fp_flags(cpu_context, "minmax", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => {  }
    }

    Ok(())
}

fn exec_fmadd(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let mut rm = ((instr >> 12) & 0x7) as u8;

    if(rm == 16) {
        //Dynamic rounding
        rm = cpu_context.csrs.read_csr(CSRAddress::FRM as u16, false)? as u8;
    }

    let rs3 = ((instr >> 27) & 0x1F) as u8;

    match (instr >> 25) & 0x3 {
        0x0 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode(((cpu_context.f[rs1 as usize] as f32) * (cpu_context.f[rs2 as usize] as f32) + (cpu_context.f[rs3 as usize] as f32)) as f64, rm));
            update_fp_flags(cpu_context, "fmadd", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        0x1 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode(cpu_context.f[rs1 as usize] * cpu_context.f[rs2 as usize] + cpu_context.f[rs3 as usize], rm));
            update_fp_flags(cpu_context, "fmadd", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)? }
    }

    Ok(())
}

fn exec_fmsub(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let mut rm = ((instr >> 12) & 0x7) as u8;

    if(rm == 16) {
        //Dynamic rounding
        rm = cpu_context.csrs.read_csr(CSRAddress::FRM as u16, false)? as u8;
    }

    let rs3 = ((instr >> 27) & 0x1F) as u8;

    match (instr >> 25) & 0x3 {
        0x0 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode(((cpu_context.f[rs1 as usize] as f32) * (cpu_context.f[rs2 as usize] as f32) - (cpu_context.f[rs3 as usize] as f32)) as f64, rm));
            update_fp_flags(cpu_context, "fmadd", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        0x1 => {
            cpu_context.set_register_float(rd as usize, apply_rounding_mode(cpu_context.f[rs1 as usize] * cpu_context.f[rs2 as usize] - cpu_context.f[rs3 as usize], rm));
            update_fp_flags(cpu_context, "fmadd", cpu_context.f[rs1 as usize], cpu_context.f[rs2 as usize], cpu_context.f[rd as usize], false)?;
        }
        _ => { cpu_context.csrs.write_csr(CSRAddress::FFlags as u16, FFlags::NV.bits(), false)? }
    }

    Ok(())
}

impl ParsableInstructionGroup for FloatingPointOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct5 = ((instr >> 27) & 0x1F) as u8;

        match (funct5) {
            0x0 => wrap_r_type!(exec_fadd),
            0x1 => wrap_r_type!(exec_fsub),
            0x2 => wrap_r_type!(exec_fmul),
            0x3 => wrap_r_type!(exec_fdiv),
            0x5 => wrap_r_type!(exec_fminmax),
            0xB => wrap_r_type!(exec_fsqrt),
            _ => |_,_| { Err(Exception::IllegalInstruction) },
        }
    }
}

impl ParsableInstructionGroup for LoadFloatingPointOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct5 = ((instr >> 27) & 0x1F) as u8;
        let fmt = ((instr >> 25) & 0x3) as u8;

        match (fmt, funct5) {
            (_, _) => wrap_i_type!(exec_load_fp),
        }
    }
}

impl ParsableInstructionGroup for StoreFloatingPointOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct5 = ((instr >> 27) & 0x1F) as u8;
        let fmt = ((instr >> 25) & 0x3) as u8;

        match (fmt, funct5) {
            (_, _) => wrap_s_type!(exec_store_fp),
        }
    }
}