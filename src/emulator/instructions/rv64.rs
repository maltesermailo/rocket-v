pub mod int_op;
pub mod int_op_imm;
pub mod jump_branch;

#[macro_export] macro_rules! wrap_r_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) {
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
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) {
                let rd = ((instr >> 7) & 0x1F) as u8;

                let imm110 = ((instr >> 21) & 0x3FF) << 1 as u32; //Bits 1 to 11
                let imm11 = ((instr >> 20) & 1) << 11 as u32; //Bit 11
                let imm1219 = ((instr >> 12) & 0xFF) << 12 as u32; //Bits 12 to 19
                let imm20 = ((instr >> 31) & 1) << 20 as u32; //Bit 20

                let imm = imm110 | imm11 | imm1219 | imm20;

                $exec_fn(cpu_context, instr, rd, imm)
            }
            wrapper
        }
    }
}

#[macro_export] macro_rules! wrap_i_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) {
                let rd = ((instr >> 7) & 0x1F) as u8;
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let mut imm = ((instr >> 20)) as u64;

                //Check if signed
                if((instr & (1<<31)) > 0) {
                    //Sign extend immediate with 1
                    imm |= !0xFFF;
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
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) {
                let rd = ((instr >> 7) & 0x1F) as u8;
                let rs1 = ((instr >> 15) & 0x1F) as u8;
                let imm = ((instr >> 20)) as u64;

                $exec_fn(cpu_context, instr, rd, rs1, imm)
            }
            wrapper
        }
    }
}