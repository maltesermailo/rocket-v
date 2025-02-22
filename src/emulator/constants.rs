pub const XLEN: usize = 64;
pub const ILEN: usize = 32;
pub const GPR_COUNT: usize = 32;


pub const SATP_MODE_BARE: u64 = 0;
pub const SATP_MODE_SV32: u64 = 1;
pub const SATP_MODE_SV39: u64 = 8;
pub const SATP_MODE_SV48: u64 = 9;