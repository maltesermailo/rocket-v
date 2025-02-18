use crate::emulator::state::memory::Memory;

pub struct RV64CPUContext {
    x: [u64; 32], //General purpose registers
    pc: u64, //Program counter

    memory: Memory,
}

impl RV64CPUContext {
    fn new(memory_size: usize) -> Self {
        Self { x: [0; 32], pc: 0, memory: Memory::new(memory_size) }
    }
}