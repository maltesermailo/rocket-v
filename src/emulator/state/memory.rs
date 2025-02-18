pub struct Memory {
    memory: Vec<u8>, //Full memory pointer
}

enum MemoryType {
    IO = 1,
}

struct MemoryRegion {
    start: usize,
    length: usize,

    memory_type: MemoryType
}

//This is a very simple implementation for testing purposes only
impl Memory {
    pub(crate) fn new(size: usize) -> Self {
        Self { memory: vec![0; size] }
    }

    fn read(&self, addr: u8) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u8, byte: u8) {
        self.memory[addr as usize] = byte;
    }
}