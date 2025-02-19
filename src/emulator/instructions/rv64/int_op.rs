use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::RV64CPUContext;
use crate::wrap_r_type;

pub const OP_OPCODE: u32 = 0b0110011;

struct IntOpOpcodeGroup {

}

type ExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8);


fn exec_add(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] + cpu_context.x[rs2 as usize]);
}

fn exec_sub(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] - cpu_context.x[rs2 as usize]);
}

fn exec_xor(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] ^ cpu_context.x[rs2 as usize]);
}

fn exec_or(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] | cpu_context.x[rs2 as usize]);
}

fn exec_and(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] & cpu_context.x[rs2 as usize]);
}

fn exec_sll(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] << cpu_context.x[rs2 as usize]);
}

fn exec_srl(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, cpu_context.x[rs1 as usize] >> cpu_context.x[rs2 as usize]);
}

fn exec_sra(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, (cpu_context.x[rs1 as usize] as i64 >> cpu_context.x[rs2 as usize]) as u64);
}

fn exec_slt(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, if (cpu_context.x[rs1 as usize] as i64) < (cpu_context.x[rs2 as usize] as i64) { 1 } else { 0 });
}

fn exec_sltu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.set_register(rd as usize, if cpu_context.x[rs1 as usize] < cpu_context.x[rs2 as usize] { 1 } else { 0 });
}

fn exec_unknown(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {

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
            _ => |_,_| {}
        }
    }
}