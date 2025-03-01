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
            _ => |_,_| { Err(Exception::IllegalInstruction) }
        }
    }
}