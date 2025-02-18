use crate::emulator::state::rv64_cpu_context::RV64CPUContext;

pub const OPCODE_MASK: u32 = 0b1111111;

trait ParsableInstructionGroup {
    fn parse(instr: u32) -> InstructionFn;
}

trait Instruction {
    fn execute(&self, cpu: &mut RV64CPUContext);
}

type InstructionFn = fn(&mut RV64CPUContext);