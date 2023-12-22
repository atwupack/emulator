use crate::mem::MemoryError::IndexOutOfBounds;

#[derive(Debug)]
pub enum MemoryError {
    IndexOutOfBounds,
}

pub trait Memory {
    fn get(&self, index: u16) -> Result<u8, MemoryError>;

    fn set(&mut self, index: u16, value: u8) -> Result<(), MemoryError>;
}

pub struct RAM<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Memory for RAM<N> {
    fn get(&self, index: u16) -> Result<u8, MemoryError> {
        if (index as usize) < N  {
            Ok(self.data[index as usize])
        } else {
            Err(IndexOutOfBounds)
        }
    }

    fn set(&mut self, index: u16, value: u8) -> Result<(), MemoryError> {
        if (index as usize) < N  {
            self.data[index as usize] = value;
            Ok(())
        } else {
            Err(IndexOutOfBounds)
        }

    }
}

impl<const N: usize> Default for RAM<N> {
    fn default() -> Self {
        RAM {
            data: [0; N]
        }
    }
}

