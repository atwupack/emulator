use crate::cpu::CPU;

pub trait InstructionParam<const N: usize> {
    fn new(cpu: &mut CPU<N>) -> Self;
}

pub struct Immediate {
    value: u8,
}

impl<const N: usize> InstructionParam<N> for Immediate {
    fn new(cpu: &mut CPU<N>) -> Self {
        let value = cpu.fetch_next_byte();
        Immediate {
            value,
        }
    }
}

pub trait Instruction<const N: usize> {
    fn execute(self, cpu: &mut CPU<N>);
}

impl<const N: usize, F1: InstructionParam<N>> Instruction<N> for fn(F1)
{
    fn execute(self, cpu: &mut CPU<N>) {
        let param = F1::new(cpu);
        self(param);
    }
}