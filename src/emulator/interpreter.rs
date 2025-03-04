use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::{Arc, RwLock};
use rustyline::{DefaultEditor, Editor};
use rustyline::history::DefaultHistory;
use crate::emulator::instructions::rv64::RV64InstructionParser;
use crate::emulator::state::memory::{Device, MemoryManagementUnit};
use crate::emulator::state::rv64_cpu_context::{CSRAddress, Exception, MStatusFlags, PrivilegeMode, RV64CPUContext};
use crate::emulator::state::rv64_cpu_context::CSRAddress::MStatus;


pub struct RV64Platform {
    harts: Vec<Interpreter>,
    mmu: Arc<RwLock<MemoryManagementUnit>>,
    editor: Editor<(), DefaultHistory>,
    breakpoints: HashSet<u64>,
}

impl RV64Platform {
    pub fn new(threads: u64, memory_size: u64) -> RV64Platform {
        let mut platform = RV64Platform {
            harts: vec![],
            mmu: Arc::new(RwLock::new(MemoryManagementUnit::new(memory_size as usize))),
            breakpoints: HashSet::new(),
            editor: DefaultEditor::new().unwrap()
        };

        platform.harts.push(Interpreter::new(0x1000, platform.mmu.clone()));

        platform
    }

    pub fn load_disk_image(&mut self, disk_image: &str) {
        let path = Path::new(disk_image);

        if(path.is_file()) {
            let mut file = File::open(path).unwrap();
            let len = file.metadata().unwrap().len() as usize;
            let mut buf = vec![0; len];

            file.read(buf.as_mut_slice()).expect("Failed to read disk_image");

            self.mmu.write().unwrap().write(0x1000, len, buf.as_slice());
        }
    }

    pub fn debug_loop(&mut self, cycle_callback: fn(cycle: usize)) {
        println!("RISC-V Debugger. Type 'help' for commands.");

        loop {
            match self.editor.readline("debug> ") {
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str()).expect("Should work");
                    self.handle_debug_command(&line, cycle_callback);
                }
                Err(_) => break,
            }
        }
    }

    fn handle_debug_command(&mut self, line: &str, cycle_callback: fn(cycle: usize)) {
        let args: Vec<&str> = line.split_whitespace().collect();
        if args.is_empty() { return; }

        match args[0] {
            "s" | "step" => {
                if let Err(e) = self.harts.first_mut().unwrap().step() {
                    self.harts.first_mut().unwrap().handle_exception(e);
                }
            }
            "c" | "continue" => {
                while !self.breakpoints.contains(&self.harts.first_mut().unwrap().cpu_context.pc) {
                    if let Err(e) = self.harts.first_mut().unwrap().step()  {
                        self.harts.first_mut().unwrap().handle_exception(e);
                        break;
                    }
                }
                println!("Breakpoint hit at {:#x}", self.harts.first().unwrap().cpu_context.pc);
            }
            "b" | "break" => {
                if args.len() > 1 {
                    if let Ok(addr) = u64::from_str_radix(args[1].trim_start_matches("0x"), 16) {
                        self.breakpoints.insert(addr);
                        println!("Breakpoint set at {:#x}", addr);
                    }
                }
            },
            "p" | "print" => {
                self.harts.first().unwrap().print_state();
            },
            "h" | "help" => {
                println!("Command list:");
                println!("s | step => Advance program counter by one instruction");
                println!("c | continue => Advance program counter until breakpoint or exit");
                println!("b | break <x> => Set breakpoint at x");
                println!("p | print => Print register state");
            }
            _ => println!("Unknown command. Type 'help' for commands."),
        }
    }
}

pub(crate) struct Interpreter {
    cpu_context: RV64CPUContext,
    cycles: usize,
}

impl Interpreter {

    pub fn new(entrypoint: u64, memory_management_unit: Arc<RwLock<MemoryManagementUnit>>) -> Self {
        Interpreter { cpu_context: RV64CPUContext::new(entrypoint, memory_management_unit), cycles: 0 }
    }

    fn handle_exception_machine(&mut self, exception: Exception) {
        let mstatus = self.cpu_context.csrs.read_csr(CSRAddress::MStatus as u16, true).unwrap();

        if(MStatusFlags::from_bits_retain(mstatus).contains(MStatusFlags::MIE)) {
            let mtvec = self.cpu_context.csrs.read_csr(CSRAddress::MTVec as u16, true).unwrap();
            self.cpu_context.csrs.write_csr(CSRAddress::MCause as u16, exception as u64, true).unwrap();
            self.cpu_context.csrs.write_csr(CSRAddress::MEPC as u16, self.cpu_context.pc, true).unwrap();
            self.cpu_context.csrs.write_csr(CSRAddress::MStatus as u16, mstatus & !MStatusFlags::MIE.bits(), true).unwrap();

            let mode = (mtvec >> 2) & 0b11;

            if mode == 0 {
                self.cpu_context.pc = mtvec & !0b11;
            } else if mode == 1 {
                self.cpu_context.pc = (mtvec & !0b11) + 4 * exception as u64;
            } else {
                panic!("Unsupported mtvec mode");
            }
        } else {
            panic!("Machine mode exception with MIE disabled");
        }
    }

    fn handle_exception(&mut self, exception: Exception) {
        match self.cpu_context.csrs.get_current_privilege() {
            PrivilegeMode::Machine => {
                //Machine mode exceptions are passed directly to the handler
                self.handle_exception_machine(exception);
            }
            PrivilegeMode::Supervisor => {
                //Supervisor mode exceptions are passed directly to the handler due supervisor not being able to cause traps
                self.handle_exception_machine(exception);
            }
            PrivilegeMode::User => {
                //User mode exceptions are passed to the supervisor if not delegated.
                let medeleg = self.cpu_context.csrs.read_csr(CSRAddress::MEDeleg as u16, true).unwrap();

                if (medeleg << exception as u64 & 1 == 0) {
                    self.cpu_context.csrs.change_privilege(PrivilegeMode::Machine);

                    self.handle_exception_machine(exception);
                } else {
                    let stvec = self.cpu_context.csrs.read_csr(CSRAddress::STVec as u16, true).unwrap();
                    self.cpu_context.csrs.write_csr(CSRAddress::SCause as u16, exception as u64, true).unwrap();
                    self.cpu_context.csrs.write_csr(CSRAddress::SEPC as u16, self.cpu_context.pc, true).unwrap();
                    self.cpu_context.csrs.write_csr(CSRAddress::SStatus as u16, self.cpu_context.csrs.read_csr(CSRAddress::SStatus as u16, true).unwrap() & !MStatusFlags::SIE.bits(), true).unwrap();

                    let mode = (stvec >> 2) & 0b11;

                    self.cpu_context.csrs.change_privilege(PrivilegeMode::Supervisor);

                    if mode == 0 {
                        self.cpu_context.pc = stvec & !0b11;
                    } else if mode == 1 {
                        self.cpu_context.pc = (stvec & !0b11) + 4 * exception as u64;
                    } else {
                        panic!("Unsupported stvec mode");
                    }
                }
            }
        }
    }

    fn handle_interrupt(&mut self, interrupt_no: u64) {
        match interrupt_no {
            _ => {
                panic!("Interrupts?!");
            }
        }
    }

    fn print_state(&self) {
        println!("CPU STATE AT {:x}", self.cpu_context.pc);
        for i in 0..31 {
            println!("x{}: {:x}", i, self.cpu_context.x[i]);
        }

        println!("pc: {:x}", self.cpu_context.pc);
    }

    fn check_for_interrupt(&mut self) -> Result<(), Exception> {
        let mstatus = self.cpu_context.csrs.read_csr(CSRAddress::MStatus as u16, true)?;

        match self.cpu_context.csrs.get_current_privilege() {
            PrivilegeMode::Machine => {
                if (MStatusFlags::MIE.contains(MStatusFlags::from_bits_retain(mstatus))) {
                    let mie = self.cpu_context.csrs.read_csr(CSRAddress::MIE as u16, true)?;
                    //Check for pending interrupts in MIP
                    let mip = self.cpu_context.csrs.read_csr(CSRAddress::MIP as u16, true)?;

                    for i in 0..64 {
                        let shift = 1 << i;

                        if (mip & shift != 0) {
                            if (mie & shift != 0) {
                                //Handle interrupt
                                self.handle_interrupt(i);
                            }
                        }
                    }
                }
            }
            _ => {
                panic!("Unsupported privilege mode");
            }
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let old_pc = self.cpu_context.pc;
        
        self.cycles += 1;

        let current_pc = self.cpu_context.pc;
        let instr = self.cpu_context.memory.read().unwrap().read_word(current_pc as usize);

        let instr_fn = RV64InstructionParser::parse(instr);

        let execution_result = instr_fn(&mut self.cpu_context, instr);

        if self.cpu_context.pc == old_pc {
            self.cpu_context.pc += 4;
        }
        
        //Check for instruction exception
        if let Err(e) = execution_result {
            return Err(e);
        }

        self.check_for_interrupt()?;

        Ok(())
    }

    pub fn main_loop(&mut self, cycle_callback: fn(cycle: usize)) {
        loop {
            if let Err(e) = self.step() {
                self.handle_exception(e);
            }

            cycle_callback(self.cycles);
        }
    }
}