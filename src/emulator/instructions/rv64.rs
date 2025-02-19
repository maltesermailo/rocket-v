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
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) {
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

#[macro_export] macro_rules! wrap_b_type {
    ($exec_fn:ident) => {
        {
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) {
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
            fn wrapper(cpu_context: &mut RV64CPUContext, instr: u32) {
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