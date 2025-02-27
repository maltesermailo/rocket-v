use rstest::rstest;
use crate::emulator::instructions::rv64::RV64InstructionParser;
use crate::emulator::state::memory::Device;
use crate::emulator::state::rv64_cpu_context::RV64CPUContext;

#[rstest]
#[case::add(0x004182b3, 2, 2, 4)]
#[case::sub(0x404182b3, 4, 2, 2)]
#[case::xor(0x0041c2b3, 1, 1, 0)]
#[case::or(0x0041e2b3, 1, 2, 3)]
#[case::and(0x0041f2b3, 2, 1, 0)]
#[case::sll(0x004192b3, 2, 1, 4)]
#[case::srl(0x0041d2b3, 4, 1, 2)]
#[case::sra(0x4041d2b3, 0x8000000000000000, 1, 0xc000000000000000)]
#[case::slt(0x0041a2b3, 0x8000000000000000, 2, 1)] //-1 < 2 = 1
#[case::sltu(0x0041b2b3, 0x8000000000000000, 2, 0)] //
pub fn test_integer_ops(#[case] instr: u32, #[case] x3: u64, #[case] x4: u64, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, 1024);

    cpu.set_register(3, x3);
    cpu.set_register(4, x4);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::addi(0x0c818293, 2, 202)] //ADDI x5, x3, 200
#[case::xori(0x0011c293, 1, 0)] //XORI x5, x3, 1
#[case::ori(0x0021e293, 1, 3)] //ORI x5, x3, 2
#[case::andi(0x0011f293, 2, 0)] //ANDI x5, x3, 1
#[case::slli(0x00119293, 2, 4)] //SLLI x5, x3, 1
#[case::srli(0x0011d293, 4, 2)] //SRLI x5, x3, 1
#[case::srai(0x4011d293, 0x8000000000000000, 0xc000000000000000)] //SRAI x5, x3, 1
#[case::slti(0x0021a293, 0x8000000000000000, 1)] //-1 < 2 = 1
#[case::sltui(0x0021b293, 0x8000000000000000, 0)] // SLTUI x5, x3, 2
pub fn test_integer_ops_imm(#[case] instr: u32, #[case] x3: u64, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, 1024);

    cpu.set_register(3, x3);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::lui(0x000c82b7, 0xc8000)]
pub fn test_lui(#[case] instr: u32, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, 1024);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::auipc(0x000c8297, 0xc9000)]
pub fn test_auipc(#[case] instr: u32, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, 1024);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::jal(0x002002ef, 0, 0, 0x1002)]
#[case::jalr(0x002182e7, 0x1020, 0, 0x1022)]
#[case::beq(0x0c418463, 1, 1, 0x10C8)]
#[case::beq_2(0x0c418463, 1, 2, 0x1000)]
#[case::bne(0x0c419463, 0x1020, 0, 0x10C8)]
#[case::bge(0x0c41d463, 0x1020, 0, 0x10C8)]
#[case::bge_2(0x0c41d463, 0, 0x1020, 0x1000)]
#[case::bgeu(0x0c419463, 0x8000000000000000, 0x1020, 0x10C8)] //-1 < 4128
#[case::blt(0x0c41c463, 0x1020, 0, 0x1000)]
#[case::bltu(0x0c41e463, 0x1020, 0x8000000000000000, 0x10C8)]
pub fn test_jump_branch(#[case] instr: u32, #[case] rs1: u64, #[case] rs2: u64, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, 1024);

    cpu.set_register(3, rs1);
    cpu.set_register(4, rs2);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.pc, result);
}

#[rstest]
#[case::lb(0x40018283, 0xC00, 2, 2)]
#[case::lh(0x40019283, 0xC00, 0xFFFF, 0xffffffffffffffff)] //Despite being only 16 bits long, these always have to be 64-bit -1 since they are sign-extended into the 64-bit register, the unsigned variants do it like expected.
#[case::lw(0x4001a283, 0xC00, 0xFFFFFFFF, 0xffffffffffffffff)]
#[case::ld(0x4001b283, 0xC00, 0xffffffffffffffff, 0xffffffffffffffff)]
#[case::lbu(0x4001c283, 0xC00, 255, 255)]
#[case::lhu(0x4001d283, 0xC00, 0xFFFF, 0xFFFF)]
#[case::lwu(0x4001e283, 0xC00, 0xFFFFFFFF, 0xFFFFFFFF)]
pub fn test_load(#[case] instr: u32, #[case] rs1: u64, #[case] load: u64, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, 16384);

    cpu.set_register(3, rs1);
    cpu.memory.write_double_word(0x1000, load);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::ecall(0x00000073)]
#[case::ebreak(0x00100073)]
pub fn test_ecall(#[case] instr: u32) {
    let mut cpu = RV64CPUContext::new(0x1000, 1024);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(!instr_result.is_ok(), "no exception triggered, what happened?");
}