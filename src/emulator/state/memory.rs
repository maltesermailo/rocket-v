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

    pub(crate) fn read(&self, addr: usize) -> u8 {
        self.memory[addr]
    }

    pub(crate) fn write(&mut self, addr: usize, byte: u8) {
        self.memory[addr] = byte;
    }

    pub(crate) fn read_half_word(&self, addr: usize) -> u16 {
        (self.memory[addr] as u16) | ((self.memory[addr+1] as u16) << 8)
    }

    pub(crate) fn write_half_word(&mut self, addr: usize, half_word: u16) {
        let bytes = half_word.to_le_bytes();

        for i in 0..bytes.len() {
            self.memory[addr+i] = bytes[i];
        }
    }

    pub(crate) fn read_word(&self, addr: usize) -> u32 {
        (self.memory[addr] as u32) | ((self.memory[addr+1] as u32) << 8) | ((self.memory[addr+2] as u32) << 16) | ((self.memory[addr+3] as u32) << 24)
    }

    pub(crate) fn write_word(&mut self, addr: usize, word: u32) {
        let bytes = word.to_le_bytes();

        for i in 0..bytes.len() {
            self.memory[addr+i] = bytes[i];
        }
    }

    pub(crate) fn read_double_word(&self, addr: usize) -> u64 {
        self.read_word(addr) as u64 | (self.read_word(addr+4) << 32) as u64
    }

    pub(crate) fn write_double_word(&mut self, addr: usize, double_word: u64) {
        let bytes = double_word.to_le_bytes();

        for i in 0..bytes.len() {
            self.memory[addr+i] = bytes[i];
        }
    }

    pub(crate) fn size(&self) -> u64 {
        self.memory.len() as u64
    }
}