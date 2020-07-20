use crate::Bit;
use crate::BitList;


#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Register {
	value: usize
}

impl Register {

	/// default value of zero
	pub fn new() -> Self {
		Self::default()
	}

	/// get a specific bit
	/// panics if the bit value is invalid
	fn get_bit(&self, bit: usize) -> Bit {
		Bit::from_usize((self.value >> bit) & 1).unwrap()
	}

	/// sets a bit
	/// panics if out of range
	fn set_bit(&mut self, bit: usize, value: Bit) {
		match value {
			Bit::One => self.value |= 1 << bit,
			Bit::Zero => self.value &= 0 << bit
		}
	}

	/// gets the specified numbered of bits at the specified start point
	/// returns none if out of bounds
	pub fn get_bits(&self, bit: usize, size: usize) -> Option<BitList> {
		match 2usize.checked_pow((bit + size) as u32) {
			None => None,
			Some(_) => {
				let mut bit = bit;
				let mut bits = Vec::<Bit>::with_capacity(size);

				while bit < bit + size {
					bits.push(self.get_bit(bit));
					bit += 1;
				}

				Some(BitList::from_vec(bits))
			}
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CmpResult {
	Equal,
	Greater,
	Error,
	Less
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct StatusRegister {
	value: usize
}

impl StatusRegister {

	/// default value
	pub fn new() -> Self {
		Self::default()
	}

	/// sets a bit
	/// panics if out of range
	fn set_bit(&mut self, bit: usize, value: Bit) {
		match value {
			Bit::One => self.value |= 1 << bit,
			Bit::Zero => self.value &= 0 << bit
		}
	}

	/// get a specific bit
	/// clears the bit after it's checked
	/// panics if the bit value is invalid
	fn get_bit(&mut self, bit: usize) -> Bit {
		let result = Bit::from_usize((self.value >> bit) & 1).unwrap();
		self.set_bit(bit, Bit::Zero);

		result
	}

	/// sets a bit to one
	/// panics if out of range
	fn set_flag(&mut self, bit: usize) {
		self.set_bit(bit, Bit::One);
	}

	// CHECK FLAGS

	pub fn cmp_result(&mut self) -> CmpResult {
		match self.get_bit(0) {
			Bit::Zero => match self.get_bit(1) {
				Bit::Zero => CmpResult::Error,
				Bit::One => CmpResult::Greater
			}
			Bit::One => match self.get_bit(1) {
				Bit::Zero => CmpResult::Less,
				Bit::One => CmpResult::Equal
			}
		}
	}

	pub fn divide_by_zero(&mut self) -> Bit {
		self.get_bit(2)
	}

	pub fn carry_flag(&mut self) -> Bit {
		self.get_bit(3)
	}

	pub fn zero_flag(&mut self) -> Bit {
		self.get_bit(4)
	}

	pub fn sign_flag(&mut self) -> Bit {
		self.get_bit(5)
	}

	pub fn buffer_overflow(&mut self) -> Bit {
		self.get_bit(6)
	}

	// SET FLAGS

	pub fn set_cmp_result(&mut self, result: CmpResult) {
		match result {
			CmpResult::Less => self.set_flag(0),
			CmpResult::Greater => self.set_flag(1),
			CmpResult::Equal => {
				self.set_flag(0);
				self.set_flag(1);
			}
			CmpResult::Error => {
				self.set_bit(0, Bit::Zero);
				self.set_bit(1, Bit::Zero);
			}
		}
	}

	pub fn set_divide_by_zero(&mut self) {
		self.set_flag(2);
	}

	pub fn set_carry_flag(&mut self) {
		self.set_flag(3);
	}

	pub fn set_zero_flag(&mut self) {
		self.set_flag(4);
	}

	pub fn set_sign_flag(&mut self) {
		self.set_flag(5);
	}

	pub fn set_buffer_overflow(&mut self) {
		self.set_flag(6);
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Cpu {
	general_registers: [Register; 13],
	instruction_pointer: Register,
	stack_pointer: Register,
	status_register: StatusRegister
}

impl Cpu {
	pub fn new() -> Self {
		Self::default()
	}
}