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
}