use crate::cpu::{Cpu, Mos6502};

pub trait InstructionInput<C: Cpu> {
    fn new(cpu: &mut C) -> Self;
}

impl<C: Cpu, I1:InstructionInput<C>, I2:InstructionInput<C>> InstructionInput<C> for (I1, I2) {
    fn new(cpu: &mut C) -> Self {
        let i1 = I1::new(cpu);
        let i2 = I2::new(cpu);
        (i1, i2)
    }
}

pub trait InstructionOutput<C: Cpu> {
    fn apply(&self, cpu: &mut C);
}

impl<C: Cpu, O1: InstructionOutput<C>, O2: InstructionOutput<C>> InstructionOutput<C> for (O1, O2) {

    fn apply(&self, cpu: &mut C) {
        self.0.apply(cpu);
        self.1.apply(cpu);
    }
}

pub struct Absolute {
    value: u8,
}

impl InstructionInput<Mos6502> for Absolute {
    fn new(cpu: &mut Mos6502) -> Self {
        let mut addr = cpu.fetch_next_byte() as u16;
        addr |= (cpu.fetch_next_byte() as u16) << 8;
        let value = cpu.read_byte(addr);
        Absolute { value }
    }
}

pub struct ZeroPageX {
    value: u8,
}

impl InstructionInput<Mos6502> for ZeroPageX {
    fn new(cpu: &mut Mos6502) -> Self {
        let mut zp = cpu.fetch_next_byte();
        zp = zp.wrapping_add(cpu.regs.x);
        let value = cpu.read_byte(zp as u16);
        ZeroPageX { value }
    }
}

pub struct ZeroPageY {
    value: u8,
}

impl InstructionInput<Mos6502> for ZeroPageY {
    fn new(cpu: &mut Mos6502) -> Self {
        let mut zp = cpu.fetch_next_byte();
        zp = zp.wrapping_add(cpu.regs.y);
        let value = cpu.read_byte(zp as u16);
        ZeroPageY { value }
    }
}

pub struct ZeroPage {
    value: u8,
}

impl InstructionInput<Mos6502> for ZeroPage {
    fn new(cpu: &mut Mos6502) -> Self {
        let zp = cpu.fetch_next_byte();
        let value = cpu.read_byte(zp as u16);
        ZeroPage { value }
    }
}

pub struct Immediate {
    value: u8,
}

impl InstructionInput<Mos6502> for Immediate {
    fn new(cpu: &mut Mos6502) -> Self {
        let value = cpu.fetch_next_byte();
        Immediate { value }
    }
}

pub struct Accumulator {
    value: u8,
}

impl InstructionInput<Mos6502> for Accumulator {
    fn new(cpu: &mut Mos6502) -> Self {
        Accumulator { value: cpu.regs.a }
    }
}

impl InstructionOutput<Mos6502> for Accumulator {
    fn apply(&self, cpu: &mut Mos6502) {
        cpu.regs.a = self.value;
        cpu.regs.z = cpu.regs.a == 0;
        cpu.regs.n = (cpu.regs.a & 0b10000000) > 0;
    }
}

pub trait Instruction<C: Cpu> {
    fn execute(&self, cpu: &mut C);
}

impl<C: Cpu, I1: InstructionInput<C>, O1: InstructionOutput<C>> Instruction<C> for fn(I1) -> O1 {
    fn execute(&self, cpu: &mut C) {
        let param = I1::new(cpu);
        let output = self(param);
        output.apply(cpu);
    }
}

pub fn lda_immediate(value: Immediate) -> Accumulator {
    Accumulator { value: value.value }
}

pub fn lda_zero_page(value: ZeroPage) -> Accumulator {
    Accumulator { value: value.value }
}

pub fn lda_zero_page_x(value: ZeroPageX) -> Accumulator {
    Accumulator { value: value.value }
}
