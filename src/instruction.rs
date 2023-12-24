use crate::cpu::CPU;

pub trait InstructionInput {
    fn new(cpu: &mut CPU) -> Self;
}

pub trait InstructionOutput {
    fn apply(&self, cpu: &mut CPU);
}

pub struct Absolute {
    value: u8,
}

impl InstructionInput for Absolute {
    fn new(cpu: &mut CPU) -> Self {
        let mut addr = cpu.fetch_next_byte() as u16;
        addr |= (cpu.fetch_next_byte() as u16) << 8;
        let value = cpu.read_byte(addr);
        Absolute { value }
    }
}

pub struct ZeroPageX {
    value: u8,
}

impl InstructionInput for ZeroPageX {
    fn new(cpu: &mut CPU) -> Self {
        let mut zp = cpu.fetch_next_byte();
        zp = zp.wrapping_add(cpu.x);
        let value = cpu.read_byte(zp as u16);
        ZeroPageX { value }
    }
}

pub struct ZeroPageY {
    value: u8,
}

impl InstructionInput for ZeroPageY {
    fn new(cpu: &mut CPU) -> Self {
        let mut zp = cpu.fetch_next_byte();
        zp = zp.wrapping_add(cpu.y);
        let value = cpu.read_byte(zp as u16);
        ZeroPageY { value }
    }
}

pub struct ZeroPage {
    value: u8,
}

impl InstructionInput for ZeroPage {
    fn new(cpu: &mut CPU) -> Self {
        let zp = cpu.fetch_next_byte();
        let value = cpu.read_byte(zp as u16);
        ZeroPage { value }
    }
}

pub struct Immediate {
    value: u8,
}

impl InstructionInput for Immediate {
    fn new(cpu: &mut CPU) -> Self {
        let value = cpu.fetch_next_byte();
        Immediate { value }
    }
}

pub struct Accumulator {
    value: u8,
}

impl InstructionInput for Accumulator {
    fn new(cpu: &mut CPU) -> Self {
        Accumulator { value: cpu.a }
    }
}

impl InstructionOutput for Accumulator {
    fn apply(&self, cpu: &mut CPU) {
        cpu.a = self.value;
        cpu.z = cpu.a == 0;
        cpu.n = (cpu.a & 0b10000000) > 0;
    }
}

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU);
}

impl<I1: InstructionInput, O1: InstructionOutput> Instruction for fn(I1) -> O1 {
    fn execute(&self, cpu: &mut CPU) {
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
