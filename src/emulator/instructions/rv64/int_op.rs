pub const OP_OPCODE: u32 = 0b0110011;

struct IntOpOpcodeGroup {

}

type ExecutionFn = fn(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8);

fn parse_r_type(cpu_context: &mut RV64CPUContext, instr: u32, exec_fn: ExecutionFn) -> InstructionFn {
    |cpu_context: &mut RV64CPUContext, instr: u32| {
        let rd = ((instr >> 7) & 0x1F) as u8;
        let rs1 = ((instr >> 15) & 0x1F) as u8;
        let rs2 = ((instr >> 20) & 0x1F) as u8;

        exec_fn(cpu, rd, rs1, rs2);
    }
}


fn exec_add(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] + cpu_context.x[rs2 as usize];
}

fn exec_sub(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u8) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] - cpu_context.x[rs2 as usize];
}

fn exec_xor(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] ^ cpu_context.x[rs2 as usize];
}

fn exec_or(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] | cpu_context.x[rs2 as usize];
}

fn exec_and(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] & cpu_context.x[rs2 as usize];
}

fn exec_sll(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] << cpu_context.x[rs2 as usize];
}

fn exec_srl(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = cpu_context.x[rs1 as usize] >> cpu_context.x[rs2 as usize];
}

fn exec_sra(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = (cpu_context.x[rs1 as usize] as i64 >> cpu_context.x[rs2 as usize]) as u64;
}

fn exec_slt(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = if((cpu_context.x[rs1] as i64) < (cpu_context.x[rs2] as i64)) { 1 } else { 0 };
}

fn exec_sltu(cpu_context: &mut RV64CPUContext, instr: u32, rd: u8, rs1: u8, rs2: u32) {
    cpu_context.x[rd as usize] = if(cpu_context.x[rs1] < cpu_context.x[rs2]) { 1 } else { 0 };
}


impl ParseableOpcodeGroup for IntOpOpcodeGroup {
    fn parse(instr: u32) -> InstructionFn {
        let funct3 = ((instr >> 12) & 0x07) as u8;
        let funct7 = ((instr >> 25) & 0x7F) as u8;

        match (funct3, funct7) {
            (0x0, 0x0)  => parse_r_type(cpu_context, instr, exec_add),
            (0x0, 0x20) => parse_r_type(cpu_context, instr, exec_sub),
            (0x4, 0x0)  => parse_r_type(cpu_context, instr, exec_xor),
            (0x6, 0x0)  => parse_r_type(cpu_context, instr, exec_or),
            (0x7, 0x0)  => parse_r_type(cpu_context, instr, exec_and),
            (0x1, 0x0)  => parse_r_type(cpu_context, instr, exec_sll),
            (0x5, 0x0)  => parse_r_type(cpu_context, instr, exec_srl),
            (0x5, 0x20) => parse_r_type(cpu_context, instr, exec_sra),
            (0x2, 0x0)  => parse_r_type(cpu_context, instr, exec_slt),
            (0x3, 0x0)  => parse_r_type(cpu_context, instr, exec_sltu),
        }
    }
}