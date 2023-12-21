use crate::cpu::{CPU, INS_LDA_ZP};
use crate::mem::Memory;

mod cpu;
mod mem;
mod instruction;

const MAX_MEM: usize = 1024 * 64;

fn main() {
    let mut cpu: CPU<MAX_MEM> = CPU::default();
    cpu.reset();
    cpu.memory().set(0xFF00,INS_LDA_ZP);
    cpu.memory().set(0xFF01,0x42);
    cpu.memory().set(0x42,0x84);
    cpu.execute(3);
    println!("done");
}
