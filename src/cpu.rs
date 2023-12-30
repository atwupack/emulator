use crate::instruction::{
    lda_immediate, lda_zero_page, lda_zero_page_x, Accumulator, Immediate, Instruction, ZeroPage,
    ZeroPageX,
};
use crate::memory::Memory;
use std::collections::BTreeMap;
use std::rc::Rc;

struct InstructionSet<C: Cpu> {
    instructions: BTreeMap<u8, Rc<dyn Instruction<C>>>,
}

impl<C: Cpu> Default for InstructionSet<C> {
    fn default() -> Self {
        InstructionSet {
            instructions: BTreeMap::new(),
        }
    }
}

impl InstructionSet<Cpu6502> {
    fn init(&mut self) {
        self.add_ins(
            INS_LDA_IM,
            Rc::new(lda_immediate as fn(Immediate) -> Accumulator),
        );
        self.add_ins(
            INS_LDA_ZP,
            Rc::new(lda_zero_page as fn(ZeroPage) -> Accumulator),
        );
        self.add_ins(
            INS_LDA_ZPX,
            Rc::new(lda_zero_page_x as fn(ZeroPageX) -> Accumulator),
        );
    }
}

impl<C: Cpu> InstructionSet<C> {
    fn get_ins(&self, op_code: u8) -> Option<Rc<dyn Instruction<C>>> {
        self.instructions.get(&op_code).map(Rc::clone)
    }

    fn add_ins(&mut self, op_code: u8, ins: Rc<dyn Instruction<C>>) {
        self.instructions.insert(op_code, ins);
    }


}

pub const INS_LDA_IM: u8 = 0xA9;
pub const INS_LDA_ZP: u8 = 0xA5;
pub const INS_LDA_ZPX: u8 = 0xB5;

pub trait Cpu {
    type Registers;

    fn regs(&mut self) -> &mut Self::Registers;
}

pub struct Registers6502 {
    pc: u16, // program counter
    sp: u16, // stack pointer
    // registers
    pub a: u8,
    pub x: u8,
    pub y: u8,
    // status flags
    c: bool,
    pub z: bool,
    i: bool,
    d: bool,
    b: bool,
    v: bool,
    pub n: bool,
}

impl Default for Registers6502 {
    fn default() -> Self {
        Registers6502 {
            pc: 0xFF00,
            sp: 0x0100,
            a: 0,
            x: 0,
            y: 0,
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
        }
    }
}

pub struct Cpu6502 {

    pub(crate) regs: Registers6502,

    is: InstructionSet<Cpu6502>,

    memory: Box<dyn Memory>,

    cycles: usize,
}

impl Cpu for Cpu6502 {
    type Registers = Registers6502;

    fn regs(&mut self) -> &mut Self::Registers {
        todo!()
    }
}

impl Cpu6502 {
    pub fn new(mem: impl Memory + 'static) -> Self {
        let mut is = InstructionSet::default();
        is.init();
        Cpu6502 {
            regs: Registers6502::default(),

            memory: Box::new(mem),
            is,

            cycles: 0,
        }
    }

    pub fn fetch_next_byte(&mut self) -> u8 {
        let data = self.memory.get(self.regs.pc).unwrap();
        self.regs.pc += 1;
        self.cycles += 1;
        data
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let data = self.memory.get(address).unwrap();
        self.cycles += 1;
        data
    }

    fn get_instruction(&self, op_code: u8) -> Option<Rc<dyn Instruction<Cpu6502>>> {
        self.is.get_ins(op_code)
    }

    fn fetch_next_instruction(&mut self) -> Result<Rc<dyn Instruction<Cpu6502>>, u8> {
        let op_code = self.fetch_next_byte();
        let ins = self.get_instruction(op_code);
        match ins {
            Some(i) => Ok(i),
            None => Err(op_code),
        }
    }

    pub fn execute(&mut self, cycles: usize) {
        let start_cycles = self.cycles;
        let target_cycles = start_cycles + cycles;
        while self.cycles < target_cycles {
            let ins = self.fetch_next_instruction();
            match ins {
                Ok(i) => {
                    i.execute(self);
                }
                Err(op_code) => {
                    println!("Instruction not handled {}", op_code)
                }
            }
        }
    }
}
