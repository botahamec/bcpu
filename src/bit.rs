/// A bit, either one or zero
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Bit {
	Zero,
	One
}

impl Bit {
	/// converts a number into a bit
	/// returns None if the input is not zero or one
	pub fn from_u8(num: u8) -> Option<Bit> {
		match num {
			0 => Some(Bit::Zero),
			1 => Some(Bit::One),
			_ => None
		}
	}

	/// converts a number into a bit
	/// returns None if the input is not zero or one
	pub fn from_usize(num: usize) -> Option<Bit> {
		match num {
			0 => Some(Bit::Zero),
			1 => Some(Bit::One),
			_ => None
		}
	}
}

/// A list of bits
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct BitList {
	bits: Vec<Bit>
}

impl BitList {
	pub fn with_capacity(capacity: usize) -> Self {
		Self {bits: Vec::with_capacity(capacity)}
	}

	pub fn from_vec(vec: Vec<Bit>) -> Self {
		Self {bits: vec}
	}

	pub fn len(&self) -> usize {
		self.bits.len()
	}

	pub fn get(&self, index: usize) -> Option<&Bit> {
		self.bits.get(index)
	}

	pub fn set_index(&mut self, index: usize, value: Bit) -> Option<()> {
		if index < self.len() {
			self.bits[index] = value;
			Some(())
		} else {
			None
		}
	}
}