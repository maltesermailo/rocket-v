use std::sync::RwLock;
use crate::emulator::devices::{FramebufferDevice, RV64Device};
use crate::emulator::state::memory::{Device, MemoryType};

pub struct SimpleFramebufferDevice {
    framebuffer: RwLock<Vec<u8>>,
    size: usize
}

impl SimpleFramebufferDevice {
    pub fn new(size: usize) -> SimpleFramebufferDevice {
        Self {
            framebuffer: RwLock::new(vec![0; size]),
            size
        }
    }
}

impl RV64Device for SimpleFramebufferDevice {

    fn init(&self) {

    }

    fn destroy(&self) {

    }
}

impl Device for SimpleFramebufferDevice {
    fn read_byte(&self, addr: usize) -> u8 {
        self.framebuffer.read().unwrap()[addr]
    }

    fn write_byte(&mut self, addr: usize, byte: u8) {
        self.framebuffer.write().unwrap()[addr] = byte;
    }

    fn read_half_word(&self, addr: usize) -> u16 {
        (self.framebuffer.read().unwrap()[addr] as u16) | ((self.framebuffer.read().unwrap()[addr+1] as u16) << 8)
    }

    fn write_half_word(&mut self, addr: usize, half_word: u16) {
        let bytes = half_word.to_le_bytes();

        for i in 0..bytes.len() {
            self.framebuffer.write().unwrap()[addr+i] = bytes[i];
        }
    }

    fn read_word(&self, addr: usize) -> u32 {
        (self.framebuffer.read().unwrap()[addr] as u32) | ((self.framebuffer.read().unwrap()[addr+1] as u32) << 8) | ((self.framebuffer.read().unwrap()[addr+2] as u32) << 16) | ((self.framebuffer.read().unwrap()[addr+3] as u32) << 24)
    }

    fn write_word(&mut self, addr: usize, word: u32) {
        let bytes = word.to_le_bytes();

        for i in 0..bytes.len() {
            self.framebuffer.write().unwrap()[addr+i] = bytes[i];
        }
    }

    fn read_double_word(&self, addr: usize) -> u64 {
        self.read_word(addr) as u64 | (self.read_word(addr+4) as u64) << 32
    }

    fn write_double_word(&mut self, addr: usize, double_word: u64) {
        let bytes = double_word.to_le_bytes();

        for i in 0..bytes.len() {
            self.framebuffer.write().unwrap()[addr+i] = bytes[i];
        }
    }

    fn write(&mut self, addr: usize, len: usize, value: &[u8]) {
        for i in 0..len {
            self.framebuffer.write().unwrap()[addr+i] = value[i];
        }
    }

    fn read(&self, addr: usize, len: usize, buf: &mut [u8]) {
        for i in 0..len {
            buf[i] = self.framebuffer.read().unwrap()[addr+i];
        }
    }

    fn size(&self) -> u64 {
        self.size as u64
    }

    fn memory_type(&self) -> MemoryType {
        MemoryType::IO
    }
}