use crate::cpu::{CPU, INS_LDA_ZP};
use crate::mem::{Memory, RAM};

mod cpu;
mod mem;
mod instruction;

const MAX_MEM: usize = 1024 * 64;

fn main() {

    let mut mem : RAM<MAX_MEM> = RAM::default();
    mem.set(0xFF00,INS_LDA_ZP);
    mem.set(0xFF01,0x42);
    mem.set(0x42,0x84);

    let mut cpu = CPU::new(mem);
    cpu.execute(3);
    println!("done");
}
