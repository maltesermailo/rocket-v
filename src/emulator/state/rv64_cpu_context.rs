use crate::emulator::state::memory::Memory;

#[derive(Debug)]
pub enum Exception {
    LoadAccessFault,
    StoreAccessFault,
    IllegalInstruction,
    MisalignedLoad,
    MisalignedStore,
    // Add other RISC-V exceptions as needed
}

pub struct RV64CPUContext {
    pub(crate) x: [u64; 32], //General purpose registers
    pub(crate) pc: u64, //Program counter

    pub(crate) memory: Memory,
}

impl RV64CPUContext {
    fn new(memory_size: usize) -> Self {
        Self { x: [0; 32], pc: 0, memory: Memory::new(memory_size) }
    }

    #[inline(always)]
    pub(crate) fn set_register(&mut self, register: usize, value: u64) {
        if(register == 0) {
            return
        }

        if(register > 31) {
            return
        }

        self.x[register] = value;
    }
}