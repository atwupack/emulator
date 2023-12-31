use crate::cpu::Cpu;
use crate::instruction::{
    Absolute, Accumulator, Immediate, Instruction, InstructionInput, InstructionOutput, ZeroPage,
    ZeroPageX, ZeroPageY,
};
use crate::memory::Memory;
use std::rc::Rc;

pub const INS_LDA_IM: u8 = 0xA9;
pub const INS_LDA_ZP: u8 = 0xA5;
pub const INS_LDA_ZPX: u8 = 0xB5;

pub struct Mos6502 {
    pc: u16, // program counter
    sp: u16, // stack pointer
    // registers
    a: u8,
    x: u8,
    y: u8,
    // status flags
    c: bool,
    z: bool,
    i: bool,
    d: bool,
    b: bool,
    v: bool,
    n: bool,

    is: crate::cpu::InstructionSet<Mos6502>,

    memory: Box<dyn Memory>,

    cycles: usize,
}

impl Cpu for Mos6502 {}

impl Mos6502 {
    pub fn new(mem: impl Memory + 'static) -> Self {
        let mut is = crate::cpu::InstructionSet::default();
        is.init();
        Mos6502 {
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

            memory: Box::new(mem),
            is,

            cycles: 0,
        }
    }

    pub fn fetch_next_byte(&mut self) -> u8 {
        let data = self.memory.get(self.pc).unwrap();
        self.pc += 1;
        self.cycles += 1;
        data
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let data = self.memory.get(address).unwrap();
        self.cycles += 1;
        data
    }

    fn get_instruction(&self, op_code: u8) -> Option<Rc<dyn Instruction<Mos6502>>> {
        self.is.get_ins(op_code)
    }

    fn fetch_next_instruction(&mut self) -> Result<Rc<dyn Instruction<Mos6502>>, u8> {
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

impl crate::cpu::InstructionSet<Mos6502> {
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

impl InstructionInput<Mos6502> for Absolute {
    fn new(cpu: &mut Mos6502) -> Self {
        let mut addr = cpu.fetch_next_byte() as u16;
        addr |= (cpu.fetch_next_byte() as u16) << 8;
        let value = cpu.read_byte(addr);
        Absolute(value)
    }
}

impl InstructionInput<Mos6502> for ZeroPageX {
    fn new(cpu: &mut Mos6502) -> Self {
        let mut zp = cpu.fetch_next_byte();
        zp = zp.wrapping_add(cpu.x);
        let value = cpu.read_byte(zp as u16);
        ZeroPageX(value)
    }
}

impl InstructionInput<Mos6502> for ZeroPageY {
    fn new(cpu: &mut Mos6502) -> Self {
        let mut zp = cpu.fetch_next_byte();
        zp = zp.wrapping_add(cpu.y);
        let value = cpu.read_byte(zp as u16);
        ZeroPageY(value)
    }
}

impl InstructionInput<Mos6502> for ZeroPage {
    fn new(cpu: &mut Mos6502) -> Self {
        let zp = cpu.fetch_next_byte();
        let value = cpu.read_byte(zp as u16);
        ZeroPage(value)
    }
}

impl InstructionInput<Mos6502> for Immediate {
    fn new(cpu: &mut Mos6502) -> Self {
        let value = cpu.fetch_next_byte();
        Immediate(value)
    }
}

impl InstructionInput<Mos6502> for Accumulator {
    fn new(cpu: &mut Mos6502) -> Self {
        Accumulator(cpu.a)
    }
}

impl InstructionOutput<Mos6502> for Accumulator {
    fn apply(&self, cpu: &mut Mos6502) {
        cpu.a = self.0;
        cpu.z = cpu.a == 0;
        cpu.n = (cpu.a & 0b10000000) > 0;
    }
}

pub fn lda_immediate(value: Immediate) -> Accumulator {
    Accumulator(value.0)
}

pub fn lda_zero_page(value: ZeroPage) -> Accumulator {
    Accumulator(value.0)
}

pub fn lda_zero_page_x(value: ZeroPageX) -> Accumulator {
    Accumulator(value.0)
}
