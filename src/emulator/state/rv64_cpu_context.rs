use std::sync::{Arc, RwLock};
use crate::emulator::state::memory::{Memory, MemoryManagementUnit};
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


#[derive(Debug, Copy, Clone)]
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

    // Supervisor Trap Setup
    SStatus = 0x100,
    SIE = 0x104,
    STVec = 0x105,
    SCounterEn = 0x106,

    // Supervisor Trap Handling
    SScratch = 0x140,
    SEPC = 0x141,
    SCause = 0x142,
    STVal = 0x143,
    SIP = 0x144,

    // Supervisor MMU
    SATP = 0x180,

    // Supervisor Debug
    SContext = 0x5A8,

    // Supervisor State Enable
    SStateEn0 = 0x10D,
    SStateEn1 = 0x10E,
    SStateEn2 = 0x10F,
    SStateEn3 = 0x110,

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

    // Supervisor Trap Setup
    sstatus: u64,
    sie: u64,
    stvec: u64,
    scounteren: u64,

    // Supervisor Trap Handling
    sscratch: u64,
    sepc: u64,
    scause: u64,
    stval: u64,
    sip: u64,

    // Supervisor MMU
    satp: u64, //Root page table

    // Supervisor debug register
    scontext: u64,

    // Supervisor State Enable Registers
    sstateen0: u64,
    sstateen1: u64,
    sstateen2: u64,
    sstateen3: u64,

    //Floating point registers
    fflags: u64,
    frm: u64,
    fcsr: u64,

    //User mode registers
    cycle: u64,
    time: u64,
    instret: u64,

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
            sstatus: 0, //Supervisor status
            sie: 0, //Supervisor Interrupt Enable
            stvec: 0,//Supervisor Trap Handler Base Address
            scounteren: 0,//Supervisor Hardware Counter Enable
            sscratch: 0,//Thread-local storage
            sepc: 0,//Supervisor Exception Return Address
            scause: 0,//Supervisor Exception cause
            stval: 0,//Trap Value, contains page-fault address
            sip: 0,//Supervisor Pending Interrupts
            satp: 0,//Supervisor Root Page Table
            scontext: 0,
            sstateen0: 0,
            sstateen1: 0,
            sstateen2: 0,
            sstateen3: 0,
            fflags: 0,
            frm: 0,
            fcsr: 0,
            cycle: 0,
            time: 0,
            instret: 0,
            current_privilege: PrivilegeMode::Machine,
        }
    }

    pub fn get_current_privilege(&self) -> PrivilegeMode {
        return self.current_privilege;
    }

    pub fn change_privilege(&mut self, privilege: PrivilegeMode) {
        self.current_privilege = privilege;
    }

    // Helper method to determine required privilege level for a CSR
    fn get_required_privilege_for_csr(&self, csr_addr: u16) -> PrivilegeMode {
        // In RISC-V, CSR address space is divided based on privilege:
        // The two most significant bits indicate the lowest privilege that can access the CSR
        match (csr_addr >> 10) & 0b11 {
            0b00 => PrivilegeMode::User,      // User/unprivileged CSRs (0x000-0x3FF)
            0b01 => PrivilegeMode::Supervisor, // Supervisor CSRs (0x400-0x7FF)
            0b10 => PrivilegeMode::Machine,    // Reserved for hypervisor CSRs (0x800-0xBFF) - treat as machine
            0b11 => PrivilegeMode::Machine,    // Machine CSRs (0xC00-0xFFF)
            _ => unreachable!(), // This should never happen as we're only using 2 bits
        }
    }

    // Helper method to check if a CSR is read-only
    fn is_csr_read_only(&self, csr_addr: u16) -> bool {
        // In RISC-V, bits 11:10 of the CSR address indicate the access mode
        match (csr_addr >> 10) & 0b11 {
            0b11 => true,  // Read-only CSRs
            _ => false,    // Read-write CSRs
        }
    }

    // Additional method to check if counter CSRs are accessible based on privilege
    fn is_counter_accessible(&self, csr_addr: u16) -> bool {
        // For user-mode access to counters, check MCOUNTEREN/SCOUNTEREN
        if self.current_privilege == PrivilegeMode::User {
            if csr_addr == 0xC00 || csr_addr == 0xC01 || csr_addr == 0xC02 { // cycle, time, instret
                // Check if MCOUNTEREN and SCOUNTEREN allow access
                let counter_bit = 1 << (csr_addr & 0x1F);
                return (self.mcounteren & counter_bit) != 0 && (self.scounteren & counter_bit) != 0;
            }
        }
        // For supervisor-mode access to counters, check only MCOUNTEREN
        else if self.current_privilege == PrivilegeMode::Supervisor {
            if csr_addr == 0xC00 || csr_addr == 0xC01 || csr_addr == 0xC02 { // cycle, time, instret
                // Check if MCOUNTEREN allows access
                let counter_bit = 1 << (csr_addr & 0x1F);
                return (self.mcounteren & counter_bit) != 0;
            }
        }
        // Machine mode has access to all counters
        true
    }

    // Combined method to check CSR accessibility
    pub fn is_csr_accessible(&self, csr_addr: u16) -> bool {
        // Check privilege level
        let required_privilege = self.get_required_privilege_for_csr(csr_addr);
        if (self.current_privilege as u8) < (required_privilege as u8) {
            return false;
        }

        // Special case for counter CSRs
        if (csr_addr >= 0xC00 && csr_addr <= 0xC1F) {
            return self.is_counter_accessible(csr_addr);
        }

        true
    }

    pub fn read_csr(&self, csr_addr: u16, override_privs: bool) -> Result<u64, Exception> {
        let required_privilege = self.get_required_privilege_for_csr(csr_addr);

        if (self.current_privilege as u8) < (required_privilege as u8) && !override_privs {
            return Err(Exception::IllegalInstruction);
        }

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

            // Supervisor Trap Setup
            x if x == CSRAddress::SStatus as u16 => Ok(self.read_sstatus()),
            x if x == CSRAddress::SIE as u16 => Ok(self.read_sie()),
            x if x == CSRAddress::STVec as u16 => Ok(self.stvec),
            x if x == CSRAddress::SCounterEn as u16 => Ok(self.scounteren),

            // Supervisor Trap Handling
            x if x == CSRAddress::SScratch as u16 => Ok(self.sscratch),
            x if x == CSRAddress::SEPC as u16 => Ok(self.sepc),
            x if x == CSRAddress::SCause as u16 => Ok(self.scause),
            x if x == CSRAddress::STVal as u16 => Ok(self.stval),
            x if x == CSRAddress::SIP as u16 => Ok(self.read_sip()),

            // Supervisor MMU
            x if x == CSRAddress::SATP as u16 => Ok(self.satp),

            // Supervisor Debug
            x if x == CSRAddress::SContext as u16 => Ok(self.scontext),

            // Supervisor State Enable
            x if x == CSRAddress::SStateEn0 as u16 => Ok(self.sstateen0),
            x if x == CSRAddress::SStateEn1 as u16 => Ok(self.sstateen1),
            x if x == CSRAddress::SStateEn2 as u16 => Ok(self.sstateen2),
            x if x == CSRAddress::SStateEn3 as u16 => Ok(self.sstateen3),

            // Invalid or unimplemented CSR
            _ => Err(Exception::IllegalInstruction),
        }
    }

    pub fn write_csr(&mut self, csr_addr: u16, value: u64, override_privs: bool) -> Result<(), Exception> {
        let required_privilege = self.get_required_privilege_for_csr(csr_addr);

        if (self.current_privilege as u8) < (required_privilege as u8) && !override_privs {
            return Err(Exception::IllegalInstruction);
        }

        if self.is_csr_read_only(csr_addr) {
            return Err(Exception::IllegalInstruction);
        }

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
                self.write_mie(value);
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

            // Supervisor Trap Setup
            x if x == CSRAddress::SStatus as u16 => {
                self.write_sstatus(value);
                Ok(())
            },
            x if x == CSRAddress::SIE as u16 => {
                self.write_sie(value);
                Ok(())
            },
            x if x == CSRAddress::STVec as u16 => {
                self.stvec = value;
                Ok(())
            },
            x if x == CSRAddress::SCounterEn as u16 => {
                self.scounteren = value;
                Ok(())
            },

            // Supervisor Trap Handling
            x if x == CSRAddress::SScratch as u16 => {
                self.sscratch = value;
                Ok(())
            },
            x if x == CSRAddress::SEPC as u16 => {
                self.sepc = value & !0b1;
                Ok(())
            },
            x if x == CSRAddress::SCause as u16 => {
                self.scause = value;
                Ok(())
            },
            x if x == CSRAddress::STVal as u16 => {
                self.stval = value;
                Ok(())
            },
            x if x == CSRAddress::SIP as u16 => {
                self.write_sip(value);
                Ok(())
            },

            // Supervisor MMU
            x if x == CSRAddress::SATP as u16 => {
                // Additional check for SATP: when in S-mode, writing to SATP might be disabled
                // by TVM bit in MSTATUS (trap virtual memory)
                if self.current_privilege == PrivilegeMode::Supervisor {
                    // Check if TVM bit is set (bit 20 in MSTATUS)
                    if (self.mstatus & (1 << 20)) != 0 {
                        return Err(Exception::IllegalInstruction);
                    }
                }
                self.satp = value;
                Ok(())
            },

            // Supervisor Debug
            x if x == CSRAddress::SContext as u16 => {
                self.scontext = value;
                Ok(())
            },

            // Supervisor State Enable
            x if x == CSRAddress::SStateEn0 as u16 => {
                self.sstateen0 = value;
                Ok(())
            },
            x if x == CSRAddress::SStateEn1 as u16 => {
                self.sstateen1 = value;
                Ok(())
            },
            x if x == CSRAddress::SStateEn2 as u16 => {
                self.sstateen2 = value;
                Ok(())
            },
            x if x == CSRAddress::SStateEn3 as u16 => {
                self.sstateen3 = value;
                Ok(())
            },

            // Invalid or unimplemented CSR
            _ => Err(Exception::IllegalInstruction),
        }
    }

    // SSTATUS is a subset of MSTATUS
    fn read_sstatus(&self) -> u64 {
        // SSTATUS is a subset of MSTATUS
        // Mask to extract the S-mode accessible bits
        let mask = MStatusFlags::SIE.bits()
            | MStatusFlags::SPIE.bits()
            | MStatusFlags::UBE.bits()
            | MStatusFlags::SPP.bits()
            | MStatusFlags::SBE.bits()
            | FS_MASK  // FS bits
            | XS_MASK  // XS bits
            | MStatusFlags::SD.bits();

        self.mstatus & mask
    }

    fn write_sstatus(&mut self, value: u64) {
        // Mask for writable bits in SSTATUS
        let mask = MStatusFlags::SIE.bits()
            | MStatusFlags::SPIE.bits()
            | MStatusFlags::UBE.bits()
            | MStatusFlags::SPP.bits()
            | MStatusFlags::SBE.bits()
            | FS_MASK;  // FS bits
        // XS bits are typically read-only

        // Clear the writable bits
        let cleared = self.mstatus & !mask;

        // Set the new values for writable bits
        self.mstatus = cleared | (value & mask);

        // Update the SD bit based on FS and XS
        self.update_sd_bit();
    }

    // SIE is a subset of MIE
    fn read_sie(&self) -> u64 {
        // SIE is MIE masked by mideleg
        self.mie & self.mideleg
    }

    fn write_sie(&mut self, value: u64) {
        // Only delegated interrupts can be controlled via SIE
        let mask = self.mideleg;

        // Clear the delegated bits in MIE
        let cleared = self.mie & !mask;

        // Set the new values for delegated bits
        self.mie = cleared | (value & mask);
    }

    // SIP is a subset of MIP
    fn read_sip(&self) -> u64 {
        // SIP is MIP masked by mideleg
        self.mip & self.mideleg
    }

    fn write_sip(&mut self, value: u64) {
        // Only certain bits of SIP are writable by software
        // And only those that are delegated
        let writable_mask = MIPFlags::USIP.bits() | MIPFlags::SSIP.bits();
        let delegated_mask = self.mideleg;

        let effective_mask = writable_mask & delegated_mask;

        // Clear the writable and delegated bits
        let cleared = self.mip & !effective_mask;

        // Set the new values for writable and delegated bits
        self.mip = cleared | (value & effective_mask);
    }

    // Helper to update the SD bit based on FS and XS
    fn update_sd_bit(&mut self) {
        let fs = (self.mstatus & FS_MASK) >> FS_SHIFT;
        let xs = (self.mstatus & XS_MASK) >> XS_SHIFT;

        // Clear the SD bit
        self.mstatus &= !MStatusFlags::SD.bits();

        // Set SD if FS or XS are 11 (dirty)
        if fs == 0b11 || xs == 0b11 {
            self.mstatus |= MStatusFlags::SD.bits();
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

    fn write_mie(&mut self, value: u64) {
        // Only delegated interrupts can be controlled via SIE
        let mie = MIEFlags::from_bits_truncate(value);

        // Set the new values for delegated bits
        self.mie = mie.bits();
    }
}

pub struct RV64CPUContext {
    pub(crate) x: [u64; 32], //General purpose registers
    pub(crate) f: [f64; 32], //Floating point registers
    pub(crate) pc: u64, //Program counter
    pub(crate) csrs: CSRFile,
    pub(crate) hart_id: u64,

    pub(crate) memory: Arc<RwLock<MemoryManagementUnit>>,
}

impl RV64CPUContext {
    pub fn new(pc: u64, memory: Arc<RwLock<MemoryManagementUnit>>) -> Self {
        Self { x: [0; 32], f: [0f64; 32], pc, memory: memory, csrs: CSRFile::new(), hart_id: 0 }
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