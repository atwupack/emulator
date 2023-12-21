use crate::mem::{Memory, RAM};
use std::collections::BTreeMap;
use std::rc::Rc;
use smallvec::SmallVec;

struct Instruction<const N: usize> {
    op_code: u8,
    exec: fn(SmallVec<[u8;5]>, &mut CPU<N>) -> (),
    data_size: u16,
}

impl<const N: usize> Instruction<N> {
    fn new(op_code: u8, data_size: u16, exec: fn(SmallVec<[u8; 5]>, &mut CPU<N>) -> ()) -> Self {
        Self {
            op_code,
            exec,
            data_size,
        }
    }
}

#[derive(Default)]
struct InstructionSet<const N: usize> {
    instructions: BTreeMap<u8, Rc<Instruction<N>>>,
}

impl<const N: usize> InstructionSet<N> {

    fn get_ins(&self, op_code: u8) -> Option<Rc<Instruction<N>>> {
        self.instructions.get(&op_code).map(Rc::clone)
    }

    fn add_ins(&mut self, ins: Instruction<N>) {
        self.instructions.insert(ins.op_code, Rc::new(ins));
    }

    fn init(&mut self) {
        self.add_ins(Instruction::new(INS_LDA_IM, 1, |data, cpu| {
            let value = data[0];
            cpu.a = value;
            cpu.lda_set_status();
        }));
        self.add_ins(Instruction::new(INS_LDA_ZP, 1, |data, cpu| {
            let zero_page_address = data[0];
            cpu.a = cpu.read_byte(zero_page_address);
            cpu.lda_set_status();
        }));
        self.add_ins(Instruction::new(INS_LDA_ZPX, 1, |data, cpu| {
            let mut zero_page_address = data[0];
            zero_page_address += cpu.x;
            cpu.cycles -= 1;
            cpu.a = cpu.read_byte(zero_page_address);
            cpu.lda_set_status();
        }));

    }
}

pub const INS_LDA_IM: u8 = 0xA9;
pub const INS_LDA_ZP: u8 = 0xA5;
pub const INS_LDA_ZPX: u8 = 0xB5;

#[derive(Default)]
pub struct CPU<const N: usize> {
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

    is: InstructionSet<N>,

    memory: RAM<N>,

    cycles: usize,
}

impl<const N: usize> CPU<N> {
    pub fn memory(&mut self) -> &mut RAM<N> {
        &mut self.memory
    }

    pub fn reset(&mut self) {
        self.pc = 0xFF00;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.c = false;
        self.z = false;
        self.i = false;
        self.d = false;
        self.b = false;
        self.v = false;
        self.n = false;

        self.memory.initialise();
        self.is.init();

        self.cycles = 0;
    }

    fn fetch_next_bytes(&mut self, n: u16) -> SmallVec<[u8;5]> {
        let data = self.memory.get_vec(self.pc, n).unwrap();
        self.pc += n;
        self.cycles -= n as usize;
        data
    }

    pub fn fetch_next_byte(&mut self) -> u8 {
        let data = self.memory.get(self.pc).unwrap();
        self.pc += 1;
        self.cycles -= 1;
        data
    }

    fn fetch_next_word(&mut self) -> u16 {
        let mut data = self.memory.get(self.pc).unwrap() as u16;
        data |= (self.memory.get(self.pc).unwrap() as u16) << 8;
        self.pc += 2;
        self.cycles -= 2;
        data
    }

    fn read_byte(&mut self, address: u8) -> u8 {
        let data = self.memory.get(address as u16).unwrap();
        self.cycles -= 1;
        data
    }

    fn lda_set_status(&mut self) {
        self.z = self.a == 0;
        self.n = (self.a & 0b10000000) > 0;
    }

    fn get_instruction(&self, op_code: u8) -> Option<Rc<Instruction<N>>> {
        self.is.get_ins(op_code)
    }

    fn fetch_next_instruction(&mut self) -> Result<(Rc<Instruction<N>>, SmallVec<[u8;5]>), u8> {
        let op_code = self.fetch_next_byte();
        let ins = self.get_instruction(op_code);
        match ins {
            Some(i) => {
                let data = self.fetch_next_bytes(i.data_size);
                Ok((i, data))
            }
            None => {
                Err(op_code)
            }
        }
    }

    pub fn execute(&mut self, cycles: usize) {
        let start_cycles = self.cycles;
        let target_cycles = start_cycles + cycles;
        while self.cycles < target_cycles {
            let ins = self.fetch_next_instruction();
            match ins {
                Ok((i, d)) => {
                    (i.exec)(d, self);
                }
                Err(op_code) => {
                    println!("Instruction not handled {}", op_code)
                }
            }
        }
    }
}
