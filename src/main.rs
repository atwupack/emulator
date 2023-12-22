use crate::cpu::{CPU, INS_LDA_IM};
use crate::mem::{Memory, RAM};

mod cpu;
mod mem;
mod instruction;

const MAX_MEM: usize = 1024 * 64;

fn main() {

    let mut mem : RAM<MAX_MEM> = RAM::default();
    let _ = mem.set(0xFF00,INS_LDA_IM);
    let _ = mem.set(0xFF01,132);

    let mut cpu = CPU::new(mem);
    println!("{}",cpu.a);
    cpu.execute(2);
    println!("{}",cpu.a);
    println!("done");
}
