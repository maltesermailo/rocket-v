pub mod emulator;
pub mod tests;

use clap::Parser;
use crate::emulator::interpreter::{Interpreter, RV64Platform};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Memory size in MB
    #[arg(short, long, help = "Memory size in MB")]
    memory_size: usize,

    #[arg(short, long)]
    image_path: String,
}

fn main() {
    let args = Args::parse();

    let mut interpreter = RV64Platform::new(0x1000, (args.memory_size * 1024 * 1024) as u64);
    interpreter.load_disk_image(&args.image_path);

    interpreter.debug_loop(|_cycle| {

    });
}
