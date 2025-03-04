use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::{Exception, RV64CPUContext};
use crate::wrap_r_type;

pub const OP_OPCODE: u8 = 0b0110011;
pub const OP_32_OPCODE: u8 = 0b0111011;

pub struct IntOpOpcodeGroup {}
pub struct IntOp32OpcodeGroup {}

type ExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception>;


fn exec_add(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] + cpu_context.x[rs2 as usize]);

    Ok(())
}

fn exec_sub(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] - cpu_context.x[rs2 as usize]);

    Ok(())
}

fn exec_xor(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] ^ cpu_context.x[rs2 as usize]);

    Ok(())
}

fn exec_or(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] | cpu_context.x[rs2 as usize]);

    Ok(())
}

fn exec_and(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] & cpu_context.x[rs2 as usize]);

    Ok(())
}

fn exec_sll(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] << cpu_context.x[rs2 as usize]);

    Ok(())
}

fn exec_srl(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] >> cpu_context.x[rs2 as usize]);

    Ok(())
}

fn exec_sra(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, (cpu_context.x[rs1 as usize] as i64 >> cpu_context.x[rs2 as usize]) as u64);

    Ok(())
}

fn exec_slt(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, if (cpu_context.x[rs1 as usize] as i64) < (cpu_context.x[rs2 as usize] as i64) { 1 } else { 0 });

    Ok(())
}

fn exec_sltu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, if cpu_context.x[rs1 as usize] < cpu_context.x[rs2 as usize] { 1 } else { 0 });

    Ok(())
}

fn exec_addw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, (cpu_context.x[rs1 as usize] as u32 + cpu_context.x[rs2 as usize] as u32) as i32 as i64 as u64);

    Ok(())
}

fn exec_subw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, (cpu_context.x[rs1 as usize] as u32 - cpu_context.x[rs2 as usize] as u32) as i32 as i64 as u64);

    Ok(())
}

fn exec_sllw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, ((cpu_context.x[rs1 as usize] as u32) << (cpu_context.x[rs2 as usize] as u32)) as i32 as i64 as u64);

    Ok(())
}

fn exec_srlw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, ((cpu_context.x[rs1 as usize] as u32) >> (cpu_context.x[rs2 as usize] as u32)) as i32 as i64 as u64);

    Ok(())
}

fn exec_sraw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, ((cpu_context.x[rs1 as usize] as i32) >> (cpu_context.x[rs2 as usize] as u32)) as i64 as u64);

    Ok(())
}

fn exec_mul(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize].wrapping_mul(cpu_context.x[rs2 as usize]));

    Ok(())
}

fn exec_mulh(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let a = cpu_context.x[rs1 as usize] as i64;
    let b = cpu_context.x[rs2 as usize] as i64;

    let result = ((a as i128 * b as i128) >> 64) as u64;

    cpu_context.set_register(rd as usize, result);

    Ok(())
}

fn exec_mulhu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let a = cpu_context.x[rs1 as usize];
    let b = cpu_context.x[rs2 as usize];

    let result = ((a as u128 * b as u128) >> 64) as u64;

    cpu_context.set_register(rd as usize, result);

    Ok(())
}

fn exec_mulhsu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let a = cpu_context.x[rs1 as usize] as i64 as i128;
    let b = cpu_context.x[rs2 as usize] as u128;

    let result = ((a * b as i128) >> 64) as u64;

    cpu_context.set_register(rd as usize, result);

    Ok(())
}

fn exec_mulw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    cpu_context.set_register(rd as usize, (cpu_context.x[rs1 as usize] as u32).wrapping_mul(cpu_context.x[rs2 as usize] as u32) as i32 as i64 as u64);

    Ok(())
}

fn exec_div(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize] as i64;
    let divisor = cpu_context.x[rs2 as usize] as i64;

    let result = if divisor == 0 {
        -1  // Division by zero must return -1 (0xFFFFFFFFFFFFFFFF)
    } else if dividend == i64::MIN && divisor == -1 {
        // Overflow case: (-2^63 / -1) is undefined, return -2^63
        i64::MIN
    } else {
        dividend / divisor
    };

    cpu_context.set_register(rd as usize, result as u64);

    Ok(())
}

fn exec_divu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize];
    let divisor = cpu_context.x[rs2 as usize];

    let result = if divisor == 0 {
        u64::MAX // Division by zero must max value
    } else {
        dividend / divisor
    };

    cpu_context.set_register(rd as usize, result);

    Ok(())
}

fn exec_divw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize] as i32;
    let divisor = cpu_context.x[rs2 as usize] as i32;

    let result = if divisor == 0 {
        -1  // Division by zero must return -1 (0xFFFFFFFFFFFFFFFF)
    } else if dividend == i32::MIN && divisor == -1 {
        // Overflow case: (-2^63 / -1) is undefined, return -2^63
        i32::MIN as i64
    } else {
        (dividend / divisor) as i64
    };

    cpu_context.set_register(rd as usize, result as u64);

    Ok(())
}

fn exec_divuw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize] as u32;
    let divisor = cpu_context.x[rs2 as usize] as u32;

    let result = if divisor == 0 {
        u32::MAX as u64 // Division by zero must max value
    } else {
        (dividend / divisor) as u64
    };

    cpu_context.set_register(rd as usize, result);

    Ok(())
}

fn exec_rem(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize] as i64;
    let divisor = cpu_context.x[rs2 as usize] as i64;

    let result = if divisor == 0 {
        dividend
    } else if dividend == i64::MIN && divisor == -1 {
        // Overflow case: (-2^63 / -1) is undefined, return 0
        0
    } else {
        (dividend % divisor)
    };

    cpu_context.set_register(rd as usize, result as u64);

    Ok(())
}

fn exec_remu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize];
    let divisor = cpu_context.x[rs2 as usize];

    let result = if divisor == 0 {
        dividend // Division by zero must dividend
    } else {
        dividend % divisor
    };

    cpu_context.set_register(rd as usize, result);

    Ok(())
}

fn exec_remw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize] as i32;
    let divisor = cpu_context.x[rs2 as usize] as i32;

    let result = if divisor == 0 {
        dividend as i64
    } else if dividend == i32::MIN && divisor == -1 {
        // Overflow case: (-2^63 / -1) is undefined, return 0
        0i64
    } else {
        (dividend % divisor) as i64
    };

    cpu_context.set_register(rd as usize, result as u64);

    Ok(())
}

fn exec_remuw(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let dividend = cpu_context.x[rs1 as usize] as u32;
    let divisor = cpu_context.x[rs2 as usize] as u32;

    let result = if divisor == 0 {
        dividend as u64 // Division by zero must max value
    } else {
        (dividend % divisor) as u64
    };

    cpu_context.set_register(rd as usize, result);

    Ok(())
}


impl ParsableInstructionGroup for IntOpOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;
        let funct7 = ((instr >> 25) & 0x7F) as u8;

        match (funct3, funct7) {
            (0x0, 0x0)  => wrap_r_type!(exec_add),
            (0x0, 0x20) => wrap_r_type!(exec_sub),
            (0x4, 0x0)  => wrap_r_type!(exec_xor),
            (0x6, 0x0)  => wrap_r_type!(exec_or),
            (0x7, 0x0)  => wrap_r_type!(exec_and),
            (0x1, 0x0)  => wrap_r_type!(exec_sll),
            (0x5, 0x0)  => wrap_r_type!(exec_srl),
            (0x5, 0x20) => wrap_r_type!(exec_sra),
            (0x2, 0x0)  => wrap_r_type!(exec_slt),
            (0x3, 0x0)  => wrap_r_type!(exec_sltu),
            (0x0, 0x01) => wrap_r_type!(exec_mul),
            (0x1, 0x01) => wrap_r_type!(exec_mulh),
            (0x2, 0x01) => wrap_r_type!(exec_mulhsu),
            (0x3, 0x01) => wrap_r_type!(exec_mulhu),
            (0x4, 0x01) => wrap_r_type!(exec_div),
            (0x5, 0x01) => wrap_r_type!(exec_divu),
            (0x6, 0x01) => wrap_r_type!(exec_rem),
            (0x7, 0x01) => wrap_r_type!(exec_remu),
            _ => |_,_| { Err(Exception::IllegalInstruction) }
        }
    }
}

impl ParsableInstructionGroup for IntOp32OpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;
        let funct7 = ((instr >> 25) & 0x7F) as u8;

        match (funct3, funct7) {
            (0x0, 0x0)  => wrap_r_type!(exec_addw),
            (0x0, 0x20) => wrap_r_type!(exec_subw),
            (0x1, 0x0)  => wrap_r_type!(exec_sllw),
            (0x5, 0x0)  => wrap_r_type!(exec_srlw),
            (0x5, 0x20) => wrap_r_type!(exec_sraw),
            (0x0, 0x01) => wrap_r_type!(exec_mulw),
            (0x4, 0x01) => wrap_r_type!(exec_divw),
            (0x5, 0x01) => wrap_r_type!(exec_divuw),
            (0x6, 0x01) => wrap_r_type!(exec_remw),
            (0x7, 0x01) => wrap_r_type!(exec_remuw),
            _ => |_,_| { Err(Exception::IllegalInstruction) }
        }
    }
}