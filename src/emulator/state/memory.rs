use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, RwLock};

pub struct MemoryManagementUnit {
    regions: BTreeMap<usize, MemoryRegion>,
    reservations: Mutex<HashMap<u64, u64>>,
}

struct MemoryRegion {
    start: usize,
    size: usize,
    device: Box<dyn Device>,
}

impl MemoryManagementUnit {
    pub(crate) fn new(memory_size: usize) -> Self {
        let mut mmu = Self {
            regions: BTreeMap::new(),
            reservations: Mutex::new(HashMap::new()),
        };

        mmu.add_region(0, memory_size, Box::new(Memory::new(memory_size)));

        mmu
    }

    pub fn new_guard(memory_size: usize) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(memory_size)))
    }

    fn add_region(&mut self, start: usize, size: usize, device: Box<dyn Device>) {
        // Check for overlaps
        if self.has_overlap(start, size) {
            panic!("Memory region overlap at {:#x}", start);
        }

        self.regions.insert(start, MemoryRegion { start, size, device });
    }

    fn has_overlap(&self, start: usize, size: usize) -> bool {
        let end = start + size;

        // Find the closest regions before and after our new region
        if let Some((_, region)) = self.regions.range(..=start).next_back() {
            if region.start + region.size > start {
                return true;
            }
        }

        if let Some((_, region)) = self.regions.range(start..).next() {
            if end > region.start {
                return true;
            }
        }

        false
    }

    fn find_region(&self, addr: usize) -> Option<&MemoryRegion> {
        // Find the last region that starts before or at our address
        self.regions.range(..=addr)
            .next_back()
            .map(|(_, region)| region)
            .filter(|region| addr < region.start + region.size)
    }

    fn find_region_mut(&mut self, addr: usize) -> Option<&mut MemoryRegion> {
        // Same as above but mutable
        self.regions.range_mut(..=addr)
            .next_back()
            .map(|(_, region)| region)
            .filter(|region| addr < region.start + region.size)
    }

    pub fn read(&self, addr: usize, size: usize, buf: &mut [u8]) {
        if let Some(region) = self.find_region(addr) {
            region.device.read(addr - region.start, size, buf);
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn write(&mut self, addr: usize, size: usize, buf: &[u8]) {
        if let Some(region) = self.find_region_mut(addr) {
            region.device.write(addr - region.start, size, buf);
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        if let Some(region) = self.find_region(addr) {
            region.device.read_byte(addr - region.start)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) {
        if let Some(region) = self.find_region_mut(addr) {
            region.device.write_byte(addr - region.start, value)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn read_half_word(&self, addr: usize) -> u16 {
        if let Some(region) = self.find_region(addr) {
            region.device.read_half_word(addr - region.start)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn write_half_word(&mut self, addr: usize, value: u16) {
        if let Some(region) = self.find_region_mut(addr) {
            region.device.write_half_word(addr - region.start, value)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn read_word(&self, addr: usize) -> u32 {
        if let Some(region) = self.find_region(addr) {
            region.device.read_word(addr - region.start)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn write_word(&mut self, addr: usize, value: u32) {
        if let Some(region) = self.find_region_mut(addr) {
            region.device.write_word(addr - region.start, value)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn read_double_word(&self, addr: usize) -> u64 {
        if let Some(region) = self.find_region(addr) {
            region.device.read_double_word(addr - region.start)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn write_double_word(&mut self, addr: usize, value: u64) {
        if let Some(region) = self.find_region_mut(addr) {
            region.device.write_double_word(addr - region.start, value)
        } else {
            panic!("Memory access violation at address {:#x}", addr)
        }
    }

    pub fn set_reservation(&self, hart_id: u64, addr: u64) {
        let mut reservations = self.reservations.lock().unwrap();
        reservations.insert(hart_id, addr);
    }

    pub fn check_reservation(&self, hart_id: u64, addr: u64) -> bool {
        let reservations = self.reservations.lock().unwrap();
        reservations.get(&hart_id) == Some(&addr)
    }

    pub fn clear_reservations_for_addr(&self, addr: u64) {
        let mut reservations = self.reservations.lock().unwrap();
        reservations.retain(|_, &mut v| v != addr);
    }

    pub fn size(&self) -> u64 {
        let mut len: u64 = 0;

        for region in self.regions.iter() {
            len += region.1.size as u64;
        }

        len
    }
}

pub struct Memory {
    memory: Vec<u8>, //Full memory pointer
}

pub enum MemoryType {
    RAM = 0,
    IO = 1,
}

pub(crate) trait Device {
    fn read_byte(&self, addr: usize) -> u8;
    fn write_byte(&mut self, addr: usize, value: u8);
    fn read_half_word(&self, addr: usize) -> u16;
    fn write_half_word(&mut self, addr: usize, value: u16);
    fn read_word(&self, addr: usize) -> u32;
    fn write_word(&mut self, addr: usize, value: u32);
    fn read_double_word(&self, addr: usize) -> u64;
    fn write_double_word(&mut self, addr: usize, value: u64);

    fn write(&mut self, addr: usize, len: usize, value: &[u8]);

    fn read(&self, addr: usize, len: usize, buf: &mut [u8]);

    fn size(&self) -> u64;

    fn memory_type(&self) -> MemoryType;
}


//This is a very simple implementation for testing purposes only
impl Memory {
    pub(crate) fn new(size: usize) -> Self {
        Self { memory: vec![0; size] }
    }
}

impl Device for Memory {
    fn read_byte(&self, addr: usize) -> u8 {
        self.memory[addr]
    }

    fn write_byte(&mut self, addr: usize, byte: u8) {
        self.memory[addr] = byte;
    }

    fn read_half_word(&self, addr: usize) -> u16 {
        (self.memory[addr] as u16) | ((self.memory[addr+1] as u16) << 8)
    }

    fn write_half_word(&mut self, addr: usize, half_word: u16) {
        let bytes = half_word.to_le_bytes();

        for i in 0..bytes.len() {
            self.memory[addr+i] = bytes[i];
        }
    }

    fn read_word(&self, addr: usize) -> u32 {
        (self.memory[addr] as u32) | ((self.memory[addr+1] as u32) << 8) | ((self.memory[addr+2] as u32) << 16) | ((self.memory[addr+3] as u32) << 24)
    }

    fn write_word(&mut self, addr: usize, word: u32) {
        let bytes = word.to_le_bytes();

        for i in 0..bytes.len() {
            self.memory[addr+i] = bytes[i];
        }
    }

    fn read_double_word(&self, addr: usize) -> u64 {
        self.read_word(addr) as u64 | (self.read_word(addr+4) as u64) << 32
    }

    fn write_double_word(&mut self, addr: usize, double_word: u64) {
        let bytes = double_word.to_le_bytes();

        for i in 0..bytes.len() {
            self.memory[addr+i] = bytes[i];
        }
    }

    fn write(&mut self, addr: usize, len: usize, value: &[u8]) {
        for i in 0..len {
            self.memory[addr+i] = value[i];
        }
    }

    fn read(&self, addr: usize, len: usize, buf: &mut [u8]) {
        for i in 0..len {
            buf[i] = self.memory[addr+i];
        }
    }

    fn size(&self) -> u64 {
        self.memory.len() as u64
    }

    fn memory_type(&self) -> MemoryType {
        MemoryType::RAM
    }
}