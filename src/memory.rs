use crate::memory::MemoryError::IndexOutOfBounds;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MemoryError {
    IndexOutOfBounds{ index: u16, size: usize},
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
            Err(IndexOutOfBounds{index, size: N})
        }
    }

    fn set(&mut self, index: u16, value: u8) -> Result<(), MemoryError> {
        if (index as usize) < N  {
            self.data[index as usize] = value;
            Ok(())
        } else {
            Err(IndexOutOfBounds{index, size: N})
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

#[cfg(test)]
mod tests {
    use crate::memory::{Memory, RAM};
    use crate::memory::MemoryError::IndexOutOfBounds;

    #[test]
    fn test_default_ram() {
        let ram:RAM<256> = RAM::default();
        for i in 0..256 {
            let entry = ram.get(i as u16);
            assert_eq!(entry, Ok(0));
        }
    }

    #[test]
    fn test_write_mem() {
        let mut ram:RAM<256> = RAM::default();
        for i in 0..256 {
            let result = ram.set(i, 1);
            assert_eq!(result, Ok(()));
            let entry = ram.get(i);
            assert_eq!(entry, Ok(1));
        }
    }

    #[test]
    fn test_get_index_out_of_bounds() {
        let ram:RAM<256> = RAM::default();
        let entry = ram.get(256);
        assert_eq!(entry, Err(IndexOutOfBounds{index: 256, size: 256}));
    }

    #[test]
    fn test_set_index_out_of_bounds() {
        let mut ram:RAM<256> = RAM::default();
        let result = ram.set(256, 1);
        assert_eq!(result, Err(IndexOutOfBounds{index: 256, size: 256}));
    }
}

