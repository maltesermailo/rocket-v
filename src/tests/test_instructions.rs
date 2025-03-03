use rstest::rstest;
use crate::emulator::instructions::rv64::RV64InstructionParser;
use crate::emulator::state::memory::{Device, MemoryManagementUnit};
use crate::emulator::state::rv64_cpu_context::{Exception, RV64CPUContext};

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
#[case::mul(0x024182b3, 0x7fffffffffffffff, 2, 0xfffffffffffffffe)]
#[case::mulh(0x024192b3, 0xffffffffffffffff, 2, 0xffffffffffffffff)]
#[case::mulhu(0x0241b2b3, 0xffffffffffffffff, 2, 0x1)]
#[case::mulhsu(0x0241a2b3, 2, 0xffffffffffffffff, 0x1)]
pub fn test_integer_ops(#[case] instr: u32, #[case] x3: u64, #[case] x4: u64, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

    cpu.set_register(3, x3);
    cpu.set_register(4, x4);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::addw(0x004182bb, 2, 2, 4)]
#[case::subw(0x404182bb, 4, 2, 2)]
#[case::sllw(0x004192bb, 2, 1, 4)]
#[case::srlw(0x0041d2bb, 4, 1, 2)]
#[case::sraw(0x4041d2bb, 0x80000000, 1, 0xc0000000)]
pub fn test_integer_ops_32(#[case] instr: u32, #[case] x3: u32, #[case] x4: u32, #[case] result: u32) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

    cpu.set_register(3, x3 as i64 as u64);
    cpu.set_register(4, x4 as i64 as u64);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result as i32 as i64 as u64);
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
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

    cpu.set_register(3, x3);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::addiw(0x0c818293, 2, 202)] //ADDI x5, x3, 200
#[case::slliw(0x0011929b, 2, 4)] //SLLI x5, x3, 1
#[case::srliw(0x0011d29b, 4, 2)] //SRLI x5, x3, 1
#[case::sraiw(0x4011d29b, 0x80000000, 0xc0000000)] //SRAI x5, x3, 1
pub fn test_integer_ops_imm_32(#[case] instr: u32, #[case] x3: u32, #[case] result: u32) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

    cpu.set_register(3, x3 as i64 as u64);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result as i32 as i64 as u64);
}

#[rstest]
#[case::lui(0x000c82b7, 0xc8000)]
pub fn test_lui(#[case] instr: u32, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::auipc(0x000c8297, 0xc9000)]
pub fn test_auipc(#[case] instr: u32, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

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
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

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
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(16384));

    cpu.set_register(3, rs1);
    cpu.memory.write().unwrap().write_double_word(0x1000, load);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok(), "exception {:?}", instr_result.expect_err("This shouldn't happen at all"));
    assert_eq!(cpu.x[5], result);
}

#[rstest]
#[case::ecall(0x00000073)]
#[case::ebreak(0x00100073)]
pub fn test_ecall(#[case] instr: u32) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(1024));

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(!instr_result.is_ok(), "no exception triggered, what happened?");
}

#[rstest]
#[case::lr_w(0x1001a2af, 0x1000, 42, 0, 42, 0)] // LR.W x5, (x3)
#[case::sc_w(0x1841a2af, 0x1000, 42, 24, 0, 24)] // SC.W x5, x4, (x3) - successful
#[case::sc_w_fail(0x1841a2af, 0x1000, 42, 24, 1, 42)] // SC.W x5, x4, (x3) - unsuccessful
#[case::amoswap_w(0x0841a2af, 0x1000, 42, 24, 42, 24)] // AMOSWAP.W x5, x4, (x3)
#[case::amoadd_w(0x0041a2af, 0x1000, 42, 24, 42, 66)] // AMOADD.W x5, x4, (x3)
#[case::amoxor_w(0x2041a2af, 0x1000, 42, 24, 42, 50)] // AMOXOR.W x5, x4, (x3)
#[case::amoor_w(0x4041a2af, 0x1000, 42, 24, 42, 58)] // AMOOR.W x5, x4, (x3)
#[case::amoand_w(0x6041a2af, 0x1000, 42, 24, 42, 8)] // AMOAND.W x5, x4, (x3)
#[case::amomin_w(0x8041a2af, 0x1000, 42, 24, 42, 24)] // AMOMIN.W x5, x4, (x3)
#[case::amomin_w_neg(0x8041a2af, 0x1000, 0xFFFFFFFF, 24, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFF)] // AMOMIN.W with negative value
#[case::amomax_w(0xa041a2af, 0x1000, 42, 100, 42, 100)] // AMOMAX.W x5, x4, (x3)
#[case::amominu_w(0xc041a2af, 0x1000, 42, 24, 42, 24)] // AMOMINU.W x5, x4, (x3)
#[case::amomaxu_w(0xe041a2af, 0x1000, 42, 100, 42, 100)] // AMOMAXU.W x5, x4, (x3)
pub fn test_amo_32(
    #[case] instr: u32,
    #[case] addr: u64,
    #[case] initial_mem: u32,
    #[case] rs2: u32,
    #[case] expected_rd: u64,
    #[case] expected_mem: u32
) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(16384));

    // Set up address in rs1
    cpu.set_register(3, addr);

    // Set up value in rs2
    cpu.set_register(4, rs2 as i32 as i64 as u64);

    // Initialize memory
    cpu.memory.write().unwrap().write_word(addr as usize, initial_mem);

    // For SC test with failure, invalidate reservation
    if instr == 0x1841a2af && expected_rd == 1 {
        cpu.memory.write().unwrap().clear_reservations_for_addr(addr);
    } else if instr == 0x1001a2af {  // LR.W
        // No need to set reservation as the instruction will do it
    } else if instr == 0x1841a2af {  // SC.D success
        cpu.memory.write().unwrap().set_reservation(0, addr);
    }

    let instr_fn = RV64InstructionParser::parse(instr);
    let result = instr_fn(&mut cpu, instr);

    assert!(result.is_ok(), "exception {:?}", result.expect_err("This shouldn't happen at all"));

    // Check register result
    assert_eq!(cpu.x[5], expected_rd);

    // Check memory result (except for LR which doesn't modify memory)
    if instr != 0x1001a2af {
        let mem_val = cpu.memory.read().unwrap().read_word(addr as usize);
        assert_eq!(mem_val, expected_mem);
    }
}

#[rstest]
#[case::lr_d(0x1001b2af, 0x1000, 0xABCDEF1234567890, 0xABCDEF1234567890, 0xABCDEF1234567890, 0xABCDEF1234567890)] // LR.D x5, (x3)
#[case::sc_d(0x1841b2af, 0x1000, 0xABCDEF1234567890, 0x1122334455667788, 0, 0x1122334455667788)] // SC.D x5, x4, (x3) - successful
#[case::sc_d_fail(0x1841b2af, 0x1000, 0xABCDEF1234567890, 0x1122334455667788, 1, 0xABCDEF1234567890)] // SC.D x5, x4, (x3) - unsuccessful
#[case::amoswap_d(0x0841b2af, 0x1000, 0xABCDEF1234567890, 0x1122334455667788, 0xABCDEF1234567890, 0x1122334455667788)] // AMOSWAP.D
#[case::amoadd_d(0x0041b2af, 0x1000, 0xABCDEF1234567890, 0x1122334455667788, 0xABCDEF1234567890, 0xbcf0225689bcf018)] // AMOADD.D
#[case::amoxor_d(0x2041b2af, 0x1000, 0xABCDEF1234567890, 0x1122334455667788, 0xABCDEF1234567890, 0xbaefdc5661300f18)] // AMOXOR.D
#[case::amoor_d(0x4041b2af, 0x1000, 0xABCDEF1234567890, 0x1122334455667788, 0xABCDEF1234567890, 0xbbefff5675767f98)] // AMOOR.D
#[case::amoand_d(0x6041b2af, 0x1000, 0xABCDEF1234567890, 0x1122334455667788, 0xABCDEF1234567890, 0x100230014467080)] // AMOAND.D
#[case::amomin_d(0x8041b2af, 0x1000, 0x8000000000000000, 0x1, 0x8000000000000000, 0x8000000000000000)] // AMOMIN.D with negative
#[case::amomax_d(0xa041b2af, 0x1000, 0x1, 0x2, 0x1, 0x2)] // AMOMAX.D
#[case::amominu_d(0xc041b2af, 0x1000, 0x8000000000000000, 0x1, 0x8000000000000000, 0x1)] // AMOMINU.D with "negative"
#[case::amomaxu_d(0xe041b2af, 0x1000, 0x1, 0x8000000000000000, 0x1, 0x8000000000000000)] // AMOMAXU.D
pub fn test_amo_64(
    #[case] instr: u32,
    #[case] addr: u64,
    #[case] initial_mem: u64,
    #[case] rs2: u64,
    #[case] expected_rd: u64,
    #[case] expected_mem: u64
) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(16384));

    // Set up address in rs1
    cpu.set_register(3, addr);

    // Set up value in rs2
    cpu.set_register(4, rs2);

    // Initialize memory
    cpu.memory.write().unwrap().write_double_word(addr as usize, initial_mem);

    // For SC test with failure, invalidate reservation
    if instr == 0x1841b2af && expected_rd == 1 {
        cpu.memory.write().unwrap().clear_reservations_for_addr(addr);
    } else if instr == 0x1001b2af {  // LR.D
        // No need to set reservation as the instruction will do it
    } else if instr == 0x1841b2af {  // SC.D success
        cpu.memory.write().unwrap().set_reservation(0, addr);
    }

    let instr_fn = RV64InstructionParser::parse(instr);
    let result = instr_fn(&mut cpu, instr);

    assert!(result.is_ok(), "exception {:?}", result.expect_err("This shouldn't happen at all"));

    // Check register result
    assert_eq!(cpu.x[5], expected_rd);

    // Check memory result (except for LR which doesn't modify memory)
    if instr != 0x1001b2af {
        let mem_val = cpu.memory.read().unwrap().read_double_word(addr as usize);
        assert_eq!(mem_val, expected_mem);
    }
}

#[rstest]
#[case::misaligned_w(0x0041a2af, 0x1001, 42, 24)]  // AMOADD.W with misaligned address
#[case::misaligned_d(0x0041b2af, 0x1004, 0xABCDEF1234567890, 0x1122334455667788)]  // AMOADD.D with misaligned address
pub fn test_amo_misaligned(
    #[case] instr: u32,
    #[case] addr: u64,
    #[case] initial_mem: u64,
    #[case] rs2: u64
) {
    let mut cpu = RV64CPUContext::new(0x1000, MemoryManagementUnit::new_guard(16384));

    // Set up address in rs1
    cpu.set_register(3, addr);

    // Set up value in rs2
    cpu.set_register(4, rs2);

    let instr_fn = RV64InstructionParser::parse(instr);
    let result = instr_fn(&mut cpu, instr);

    assert!(result.is_err(), "expected misaligned exception but got success");
    assert!(matches!(result.unwrap_err(), Exception::LoadAddressMisaligned));
}