use crate::cpu::{Mos6502, INS_LDA_IM};
use crate::memory::{Memory, RAM};

mod cpu;
mod instruction;
mod memory;
mod mos6502;

const MAX_MEM: usize = 1024 * 64;

fn main() {
    let mut mem: RAM<MAX_MEM> = RAM::default();
    let _ = mem.set(0xFF00, INS_LDA_IM);
    let _ = mem.set(0xFF01, 132);

    let mut cpu = Mos6502::new(mem);
    println!("{}", cpu.regs.a);
    cpu.execute(2);
    println!("{}", cpu.regs.a);
    println!("done");
}
