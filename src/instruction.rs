use crate::cpu::Cpu;

pub trait InstructionInput<C: Cpu> {
    fn new(cpu: &mut C) -> Self;
}

impl<C: Cpu, I1: InstructionInput<C>, I2: InstructionInput<C>> InstructionInput<C> for (I1, I2) {
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

pub struct Absolute(pub u8);

pub struct ZeroPageX(pub u8);

pub struct ZeroPageY(pub u8);

pub struct ZeroPage(pub u8);

pub struct Immediate(pub u8);

pub struct Accumulator(pub u8);

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
