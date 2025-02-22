use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use crate::emulator::instructions::rv64::RV64InstructionParser;
use crate::emulator::state::rv64_cpu_context::{CSRAddress, Exception, MStatusFlags, RV64CPUContext};
use crate::emulator::state::rv64_cpu_context::CSRAddress::MStatus;

struct Interpreter {
    cpu_context: RV64CPUContext
}

impl Interpreter {

    pub fn new(entrypoint: u64, memory_size: usize) -> Self {
        Interpreter { cpu_context: RV64CPUContext::new(entrypoint, memory_size) }
    }

    pub fn load_disk_image(&mut self, disk_image: &str) {
        let path = Path::new(disk_image);

        if(path.is_file()) {
            let mut file = File::open(path).unwrap();
            let len = file.metadata().unwrap().len() as usize;
            let mut buf = vec![0; len];

            file.read(buf.as_mut_slice()).expect("Failed to read disk_image");

            self.cpu_context.memory.write(0, len, buf.as_slice());
        }
    }

    fn handle_exception(&mut self, exception: Exception) {
        match exception {
            Exception::EnvironmentCallFromMMode => {

            }
            _ => {

            }
        }
    }

    fn handle_interrupt(&mut self, interrupt_no: u64) {
        match interrupt_no {
            _ => {

            }
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            let current_pc = self.cpu_context.pc;
            let instr = self.cpu_context.memory.read_word(current_pc as usize);

            let instr_fn = RV64InstructionParser::parse(instr);

            let execution_result = instr_fn(&mut self.cpu_context, instr);

            //Check for instruction exception
            if let Err(e) = execution_result {
                self.handle_exception(e);

                continue;
            }

            match self.cpu_context.csrs.read_csr(CSRAddress::MStatus as u16) {
                Ok(mstatus) => {
                    if(MStatusFlags::MIE.contains(MStatusFlags::from_bits_retain(mstatus))) {
                        match self.cpu_context.csrs.read_csr(CSRAddress::MIE as u16) {
                            Ok(mie) => {
                                //Check for pending interrupts in MIP
                                let mip_result = self.cpu_context.csrs.read_csr(CSRAddress::MIP as u16);

                                //Check for csr read exception
                                if let Err(e) = mip_result {
                                    self.handle_exception(e);

                                    continue;
                                }

                                //Finally check for pending interrupts
                                let mip = mip_result.unwrap();

                                for i in 0..64 {
                                    let shift = 1 << i;

                                    if(mip & shift != 0) {
                                        if(mie & shift != 0) {
                                            //Handle interrupt
                                            self.handle_interrupt(i);
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                self.handle_exception(e);

                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    self.handle_exception(e);

                    continue;
                }
            }
        }
    }
}