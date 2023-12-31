use crate::instruction::Instruction;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct InstructionSet<C: Cpu> {
    instructions: BTreeMap<u8, Rc<dyn Instruction<C>>>,
}

impl<C: Cpu> Default for InstructionSet<C> {
    fn default() -> Self {
        InstructionSet {
            instructions: BTreeMap::new(),
        }
    }
}

impl<C: Cpu> InstructionSet<C> {
    pub fn get_ins(&self, op_code: u8) -> Option<Rc<dyn Instruction<C>>> {
        self.instructions.get(&op_code).map(Rc::clone)
    }

    pub fn add_ins(&mut self, op_code: u8, ins: Rc<dyn Instruction<C>>) {
        self.instructions.insert(op_code, ins);
    }
}

pub trait Cpu {}
