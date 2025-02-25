use rstest::rstest;
use crate::emulator::instructions::rv64::RV64InstructionParser;
use crate::emulator::state::rv64_cpu_context::RV64CPUContext;

#[rstest]
#[case::add(0x004182b3, 2, 2, 4)]
pub fn test_add(#[case] instr: u32, #[case] x3: u64, #[case] x4: u64, #[case] result: u64) {
    let mut cpu = RV64CPUContext::new(0x1000, 1024);

    cpu.set_register(3, x3);
    cpu.set_register(4, x4);

    let instr_fn = RV64InstructionParser::parse(instr);

    let instr_result = instr_fn(&mut cpu, instr);

    assert!(instr_result.is_ok());
    assert_eq!(cpu.x[5], result);
}