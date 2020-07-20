
use crate::Bit;

/// virtual computer memory
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Memory {
	memory: Vec<u8>
}

/// an error resulting from memory
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryError {
	TooBigAddress
}

pub type MemoryResult<T> = Result<T, MemoryError>;

impl Memory {

	/// creates blank memory with all zeroes
	pub fn with_size(size: usize) -> Self {
		let mut memory = Vec::with_capacity(size / 8);
		for _i in 0..(size / 8) {
			memory.push(0);
		}

		Self {memory}
	}

	/// creates blank memory that starts with the executable, and the rest is zeroes
	pub fn from_executable_and_size(executable: Vec<u8>, size: usize) -> Self {
		let mut memory = executable;
		while memory.len() < size {
			memory.push(0);
		}

		Self {memory}
	}

	/// sets a specific bit to a given value
	/// returns an error if the address is out of bounds
	pub fn set_bit(&mut self, location: usize, value: Bit) -> Result<(), MemoryError> {
		if self.memory.len() > location / 8 {
			match value {
				Bit::One => self.memory[location / 8] |= 1 << (7 - location % 8),
				Bit::Zero => self.memory[location / 8] &= 0 << (7 - location % 8)
			}

			Ok(())
		} else {
			Err(MemoryError::TooBigAddress)
		}
	}

	/// returns the value of the bit
	/// returns an error if the bit is out of bounds
	pub fn get_bit(&self, location: usize) -> Result<Bit, MemoryError> {
		match self.memory.get(location / 8) {
			Some(byte) => Ok(Bit::from_u8((byte >> (location % 8)) & 1).unwrap()),
			None => Err(MemoryError::TooBigAddress)
		}
	}
}