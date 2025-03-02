use std::cmp::{max, min};
use crate::emulator::instructions::{InstructionFn, ParsableInstructionGroup};
use crate::emulator::state::rv64_cpu_context::{Exception, PrivilegeMode, RV64CPUContext};
use crate::{wrap_r_type};
use crate::emulator::instructions::rv64::InstructionResult;

pub const ATOMIC_OPCODE: u8 = 0b010_1111;


pub struct AtomicOpcodeGroup {}


type AtomicExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, imm: u64);

fn exec_lr_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let value = {
        let memory = cpu_context.memory.write().unwrap();
        let value = memory.read_word(addr as usize) as i32;

        // Store reservation in global state
        memory.set_reservation(cpu_context.hart_id, addr);

        value
    };

    cpu_context.set_register(rd as usize, value as i64 as u64);
    Ok(())
}

fn exec_sc_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let value: u64 = {
        let mut memory = cpu_context.memory.write().unwrap();

        if !memory.check_reservation(cpu_context.hart_id, addr) {
            1_u64
        } else {
            memory.write_word(addr as usize, src as u32);
            memory.clear_reservations_for_addr(addr);

            0_u64
        }
    };

    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_amoswap_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize) as i32;
        memory.write_word(addr as usize, src as u32);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoadd_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize) as i32;

        let value = old_value.wrapping_add(src);
        memory.write_word(addr as usize, value as u32);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoxor_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize) as i32;

        let value = old_value ^ src;
        memory.write_word(addr as usize, value as u32);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoor_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize) as i32;

        let value = old_value | src;
        memory.write_word(addr as usize, value as u32);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoand_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize) as i32;

        let value = old_value & src;
        memory.write_word(addr as usize, value as u32);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amomin_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize) as i32;

        let value = min(old_value, src);
        memory.write_word(addr as usize, value as u32);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amomax_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize) as i32;

        let value = max(old_value, src);
        memory.write_word(addr as usize, value as u32);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amominu_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as u32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize);

        let value = min(old_value, src);
        memory.write_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as u64);

    Ok(())
}

fn exec_amomaxu_w(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as u32;

    if addr % 4 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_word(addr as usize);

        let value = max(old_value, src);
        memory.write_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as u64);

    Ok(())
}

fn exec_lr_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let value = {
        let memory = cpu_context.memory.write().unwrap();
        let value = memory.read_double_word(addr as usize);

        // Store reservation in global state
        memory.set_reservation(cpu_context.hart_id, addr);

        value
    };

    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_sc_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let value: u64 = {
        let mut memory = cpu_context.memory.write().unwrap();

        if !memory.check_reservation(cpu_context.hart_id, addr) {
            1_u64
        } else {
            memory.write_double_word(addr as usize, src);
            memory.clear_reservations_for_addr(addr);

            0_u64
        }
    };

    cpu_context.set_register(rd as usize, value);
    Ok(())
}

fn exec_amoswap_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize);
        memory.write_double_word(addr as usize, src);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoadd_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize);

        let value = old_value.wrapping_add(src);
        memory.write_double_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoxor_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize);

        let value = old_value ^ src;
        memory.write_double_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoor_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize);

        let value = old_value | src;
        memory.write_double_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amoand_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize);

        let value = old_value & src;
        memory.write_double_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amomin_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i64;

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize) as i64;

        let value = min(old_value, src);
        memory.write_double_word(addr as usize, value as u64);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amomax_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize] as i64;

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize) as i64;

        let value = max(old_value, src);
        memory.write_double_word(addr as usize, value as u64);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as i64 as u64);

    Ok(())
}

fn exec_amominu_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize);

        let value = min(old_value, src);
        memory.write_double_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as u64);

    Ok(())
}

fn exec_amomaxu_d(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) -> Result<(), Exception> {
    let addr = cpu_context.x[rs1 as usize];
    let src = cpu_context.x[rs2 as usize];

    if addr % 8 != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }

    let old_value = {
        let mut memory = cpu_context.memory.write().unwrap();
        let old_value = memory.read_double_word(addr as usize);

        let value = max(old_value, src);
        memory.write_double_word(addr as usize, value);

        old_value
    };

    cpu_context.set_register(rd as usize, old_value as u64);

    Ok(())
}

impl ParsableInstructionGroup for AtomicOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;
        let funct5 = ((instr >> 27) & 0x1F) as u8;

        match (funct3, funct5) {
            (0x2, 0x2) => wrap_r_type!(exec_lr_w),
            (0x2, 0x3) => wrap_r_type!(exec_sc_w),
            (0x2, 0x01) => wrap_r_type!(exec_amoswap_w),
            (0x2, 0x00) => wrap_r_type!(exec_amoadd_w),
            (0x2, 0x04) => wrap_r_type!(exec_amoxor_w),
            (0x2, 0x08) => wrap_r_type!(exec_amoor_w),
            (0x2, 0x0C) => wrap_r_type!(exec_amoand_w),
            (0x2, 0x10) => wrap_r_type!(exec_amomin_w),
            (0x2, 0x14) => wrap_r_type!(exec_amomax_w),
            (0x2, 0x18) => wrap_r_type!(exec_amominu_w),
            (0x2, 0x1c) => wrap_r_type!(exec_amomaxu_w),
            (0x3, 0x2) => wrap_r_type!(exec_lr_d),
            (0x3, 0x3) => wrap_r_type!(exec_sc_d),
            (0x3, 0x01) => wrap_r_type!(exec_amoswap_d),
            (0x3, 0x00) => wrap_r_type!(exec_amoadd_d),
            (0x3, 0x04) => wrap_r_type!(exec_amoxor_d),
            (0x3, 0x08) => wrap_r_type!(exec_amoor_d),
            (0x3, 0x0C) => wrap_r_type!(exec_amoand_d),
            (0x3, 0x10) => wrap_r_type!(exec_amomin_d),
            (0x3, 0x14) => wrap_r_type!(exec_amomax_d),
            (0x3, 0x18) => wrap_r_type!(exec_amominu_d),
            (0x3, 0x1c) => wrap_r_type!(exec_amomaxu_d),
            _ => |_,_| { Err(Exception::IllegalInstruction) },
        }
    }
}