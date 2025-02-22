use crate::emulator::state::memory::Memory;
use bitflags::bitflags;

// MSTATUS register flags
bitflags! {
    pub struct MStatusFlags: u64 {
        const SIE = 1 << 1;    // Supervisor Interrupt Enable
        const MIE = 1 << 3;    // Machine Interrupt Enable
        const SPIE = 1 << 5;   // Supervisor Previous Interrupt Enable
        const UBE = 1 << 6;    // User Big-Endian
        const MPIE = 1 << 7;   // Machine Previous Interrupt Enable
        const SPP = 1 << 8;    // Supervisor Previous Privilege
        const SBE = 1 << 9;    // Supervisor Big-Endian
        const MBE = 1 << 10;   // Machine Big-Endian
        const SD = 1 << 63;    // State Dirty - summary bit
    }
}

// MIE register flags (Machine Interrupt Enable)
bitflags! {
    pub struct MIEFlags: u64 {
        const USIE = 1 << 0;   // User Software Interrupt Enable
        const SSIE = 1 << 1;   // Supervisor Software Interrupt Enable
        const MSIE = 1 << 3;   // Machine Software Interrupt Enable
        const UTIE = 1 << 4;   // User Timer Interrupt Enable
        const STIE = 1 << 5;   // Supervisor Timer Interrupt Enable
        const MTIE = 1 << 7;   // Machine Timer Interrupt Enable
        const UEIE = 1 << 8;   // User External Interrupt Enable
        const SEIE = 1 << 9;   // Supervisor External Interrupt Enable
        const MEIE = 1 << 11;  // Machine External Interrupt Enable
    }
}

// MIP register flags (Machine Interrupt Pending)
bitflags! {
    pub struct MIPFlags: u64 {
        const USIP = 1 << 0;   // User Software Interrupt Pending
        const SSIP = 1 << 1;   // Supervisor Software Interrupt Pending
        const MSIP = 1 << 3;   // Machine Software Interrupt Pending
        const UTIP = 1 << 4;   // User Timer Interrupt Pending
        const STIP = 1 << 5;   // Supervisor Timer Interrupt Pending
        const MTIP = 1 << 7;   // Machine Timer Interrupt Pending
        const UEIP = 1 << 8;   // User External Interrupt Pending
        const SEIP = 1 << 9;   // Supervisor External Interrupt Pending
        const MEIP = 1 << 11;  // Machine External Interrupt Pending
    }
}


#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
}

// CSR addresses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CSRAddress {
    // Machine Information Registers
    MVendorID = 0xF11,
    MArchID = 0xF12,
    MImpID = 0xF13,
    MHartID = 0xF14,

    // Machine Trap Setup
    MStatus = 0x300,
    MIsa = 0x301,
    MEDeleg = 0x302,
    MIDeleg = 0x303,
    MIE = 0x304,
    MTVec = 0x305,
    MCounterEn = 0x306,

    // Machine Trap Handling
    MScratch = 0x340,
    MEPC = 0x341,
    MCause = 0x342,
    MTVal = 0x343,
    MIP = 0x344,

    // Machine Counters
    MCycle = 0xB00,
    MInstRet = 0xB02,
}

const FS_SHIFT: u64 = 13;
const FS_MASK: u64 = 0b11 << FS_SHIFT;
const XS_SHIFT: u64 = 15;
const XS_MASK: u64 = 0b11 << XS_SHIFT;
const MPP_SHIFT: u64 = 11;
const MPP_MASK: u64 = 0b11 << MPP_SHIFT;

// MPP values
const MPP_USER: u64 = 0b00 << MPP_SHIFT;
const MPP_SUPERVISOR: u64 = 0b01 << MPP_SHIFT;
const MPP_MACHINE: u64 = 0b11 << MPP_SHIFT;

// Privilege levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeMode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

pub struct CSRFile {
    // Machine Information Registers
    mvendorid: u64,
    marchid: u64,
    mimpid: u64,
    mhartid: u64,

    // Machine Trap Setup
    mstatus: u64,
    misa: u64,
    medeleg: u64,
    mideleg: u64,
    mie: u64,
    mtvec: u64,
    mcounteren: u64,

    // Machine Trap Handling
    mscratch: u64,
    mepc: u64,
    mcause: u64,
    mtval: u64,
    mip: u64,

    // Machine Counters
    mcycle: u64,
    minstret: u64,

    // Current privilege level
    current_privilege: PrivilegeMode,
}

impl CSRFile {
    fn new() -> Self {
        Self {
            mvendorid: 0, //Vendor id
            marchid: 0, //Architecture id
            mimpid: 0, //Implementation id
            mhartid: 0, //Processor id
            mstatus: 0, //Machine Status
            misa: 0,  // Set this based on CPU features
            medeleg: 0, //Machine Exception Delegation Register
            mideleg: 0, //Machine Interrupt Delegation Register
            mie: 0, //Machine Interrupt Enable
            mtvec: 0, //Machine Trap Handler Base Address
            mcounteren: 0, //Machine Hardware Counter Enable
            mscratch: 0, //Thread-local storage
            mepc: 0, //Machine Exception Return Address
            mcause: 0, //Machine Exception cause
            mtval: 0, //Trap Value, contains page-fault address
            mip: 0, //Machine Pending Interrupts
            mcycle: 0, //Cycle counter
            minstret: 0, //Instructions retired counter
            current_privilege: PrivilegeMode::Machine,
        }
    }

    pub fn get_current_privilege(&self) -> PrivilegeMode {
        return self.current_privilege;
    }

    pub fn change_privilege(&mut self, privilege: PrivilegeMode) {
        self.current_privilege = privilege;
    }

    pub fn read_csr(&self, csr_addr: u16) -> Result<u64, Exception> {
        match csr_addr {
            // Machine Information Registers
            x if x == CSRAddress::MVendorID as u16 => Ok(self.mvendorid),
            x if x == CSRAddress::MArchID as u16 => Ok(self.marchid),
            x if x == CSRAddress::MImpID as u16 => Ok(self.mimpid),
            x if x == CSRAddress::MHartID as u16 => Ok(self.mhartid),

            // Machine Trap Setup
            x if x == CSRAddress::MStatus as u16 => Ok(self.read_mstatus()),
            x if x == CSRAddress::MIsa as u16 => Ok(self.misa),
            x if x == CSRAddress::MEDeleg as u16 => Ok(self.medeleg),
            x if x == CSRAddress::MIDeleg as u16 => Ok(self.mideleg),
            x if x == CSRAddress::MIE as u16 => Ok(/*self.read_mie()*/0),
            x if x == CSRAddress::MTVec as u16 => Ok(self.mtvec),
            x if x == CSRAddress::MCounterEn as u16 => Ok(self.mcounteren),

            // Machine Trap Handling
            x if x == CSRAddress::MScratch as u16 => Ok(self.mscratch),
            x if x == CSRAddress::MEPC as u16 => Ok(self.mepc),
            x if x == CSRAddress::MCause as u16 => Ok(self.mcause),
            x if x == CSRAddress::MTVal as u16 => Ok(self.mtval),
            x if x == CSRAddress::MIP as u16 => Ok(/*self.read_mip()*/ 0),

            // Machine Counters
            x if x == CSRAddress::MCycle as u16 => Ok(self.mcycle),
            x if x == CSRAddress::MInstRet as u16 => Ok(self.minstret),

            // Invalid or unimplemented CSR
            _ => Err(Exception::IllegalInstruction),
        }
    }

    pub fn write_csr(&mut self, csr_addr: u16, value: u64) -> Result<(), Exception> {
        match csr_addr {
            // Machine Information Registers - Read-Only
            x if x == CSRAddress::MVendorID as u16
                || x == CSRAddress::MArchID as u16
                || x == CSRAddress::MImpID as u16
                || x == CSRAddress::MHartID as u16 => Err(Exception::IllegalInstruction),

            // Machine Trap Setup
            x if x == CSRAddress::MStatus as u16 => {
                self.write_mstatus(value);
                Ok(())
            },
            x if x == CSRAddress::MIsa as u16 => {
                // MISA might be read-only or partially writable depending on implementation
                // Here we assume it's read-only
                Err(Exception::IllegalInstruction)
            },
            x if x == CSRAddress::MEDeleg as u16 => {
                self.medeleg = value;
                Ok(())
            },
            x if x == CSRAddress::MIDeleg as u16 => {
                self.mideleg = value;
                Ok(())
            },
            x if x == CSRAddress::MIE as u16 => {
                //self.write_mie(value);
                Ok(())
            },
            x if x == CSRAddress::MTVec as u16 => {
                self.mtvec = value;
                Ok(())
            },
            x if x == CSRAddress::MCounterEn as u16 => {
                self.mcounteren = value;
                Ok(())
            },

            // Machine Trap Handling
            x if x == CSRAddress::MScratch as u16 => {
                self.mscratch = value;
                Ok(())
            },
            x if x == CSRAddress::MEPC as u16 => {
                // MEPC is aligned to 2 bytes (instructions are at least 2 bytes)
                self.mepc = value & !0b1;
                Ok(())
            },
            x if x == CSRAddress::MCause as u16 => {
                self.mcause = value;
                Ok(())
            },
            x if x == CSRAddress::MTVal as u16 => {
                self.mtval = value;
                Ok(())
            },
            x if x == CSRAddress::MIP as u16 => {
                //self.write_mip(value);
                Ok(())
            },

            // Machine Counters
            x if x == CSRAddress::MCycle as u16 => {
                self.mcycle = value;
                Ok(())
            },
            x if x == CSRAddress::MInstRet as u16 => {
                self.minstret = value;
                Ok(())
            },

            // Invalid or unimplemented CSR
            _ => Err(Exception::IllegalInstruction),
        }
    }

    // Special handling for MSTATUS
    fn read_mstatus(&self) -> u64 {
        // The SD bit (bit 63) is a read-only bit that summarizes FS and XS
        let fs = (self.mstatus & FS_MASK) >> FS_SHIFT;
        let xs = (self.mstatus & XS_MASK) >> XS_SHIFT;

        // Set SD if FS or XS are 11 (dirty)
        let sd_bit = if fs == 0b11 || xs == 0b11 {
            MStatusFlags::SD.bits()
        } else {
            0
        };

        // Combine the computed SD bit with the stored MSTATUS value
        // Clear the existing SD bit first
        (self.mstatus & !MStatusFlags::SD.bits()) | sd_bit
    }

    fn write_mstatus(&mut self, value: u64) {
        // Extract the flags portion using bitflags
        let flags = MStatusFlags::from_bits_truncate(value) & !MStatusFlags::SD; // SD is read-only

        // Extract MPP (Machine Previous Privilege) - bits 11:12
        let mpp = (value & MPP_MASK) >> MPP_SHIFT;
        let valid_mpp = if mpp <= 0b11 { mpp } else { 0b11 };

        // Extract FS (Floating Point Status) - bits 13:14
        let fs = (value & FS_MASK) >> FS_SHIFT;

        // Extract XS (Extension Status) - bits 15:16
        let xs = (value & XS_MASK) >> XS_SHIFT;

        // Combine everything
        self.mstatus = flags.bits() |
            (valid_mpp << MPP_SHIFT) |
            (fs << FS_SHIFT) |
            (xs << XS_SHIFT);
    }
}

pub struct RV64CPUContext {
    pub(crate) x: [u64; 32], //General purpose registers
    pub(crate) pc: u64, //Program counter
    pub(crate) csrs: CSRFile,

    pub(crate) memory: Memory,
}

impl RV64CPUContext {
    pub fn new(pc: u64, memory_size: usize) -> Self {
        Self { x: [0; 32], pc: pc, memory: Memory::new(memory_size), csrs: CSRFile::new() }
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