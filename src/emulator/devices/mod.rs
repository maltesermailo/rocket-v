use std::sync::Arc;
use crate::emulator::state::memory::Device;

pub mod simple_fb;

trait RV64Device {
    fn init(&self);

    fn destroy(&self);
}

trait FramebufferDevice {
    fn read(&mut self, addr: usize, len: usize, buf: &mut [u8]);
    fn write(&mut self, addr: usize, len: usize, value: &[u8]);
}