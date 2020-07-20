#[derive(Debug)]
enum Bit {
	Zero,
	One
}

impl Bit {
	pub fn from_u8(num: u8) -> Option<Bit> {
		match num {
			0 => Some(Bit::Zero),
			1 => Some(Bit::One),
			_ => None
		}
	}
}

struct Memory {
	memory: Vec<u8>
}

#[derive(Debug)]
enum MemoryError {
	TooBigAddress
}

type MemoryResult<T> = Result<T, MemoryError>;

impl Memory {

	pub fn with_size(size: usize) -> Self {
		let mut memory = Vec::with_capacity(size / 8);
		for _i in 0..(size / 8) {
			memory.push(0);
		}

		Self {memory}
	}

	pub fn set_bit(&mut self, location: usize, value: Bit) -> Result<(), MemoryError> {
		if self.memory.len() > location / 8 {
			match value {
				Bit::One => self.memory[location / 8] |= 1 << (7 - location % 8),
				Bit::Zero => self.memory[location / 8] &= 0 << (7 - location % 8)
			}
			println!("{:08b}", self.memory[location / 8]);
			println!("{}", location % 8);

			Ok(())
		} else {
			Err(MemoryError::TooBigAddress)
		}
	}

	pub fn get_bit(&self, location: usize) -> Result<Bit, MemoryError> {
		match self.memory.get(location / 8) {
			Some(byte) => Ok(Bit::from_u8((byte >> (location % 8)) & 1).unwrap()),
			None => Err(MemoryError::TooBigAddress)
		}
	}
}

fn main() {

	// read file name
	let file_name = match std::env::args().next() {
		Some(n) => n,
		None => {
			let mut input = String::new();
			print!("Enter a bCPU executable filename: ");
			std::io::stdin().read_line(&mut input).expect("Unable to read input");

			input
		}
	};

	// 2 Kb of RAM
	const RAM_SIZE : usize = 2_000; // TODO: make this configurable
	let mut ram = Memory::with_size(RAM_SIZE);

	ram.set_bit(100, Bit::One).unwrap();
	println!("{:?}", ram.get_bit(100).unwrap());
	ram.set_bit(200, Bit::Zero).unwrap();
	println!("{:?}", ram.get_bit(200).unwrap());
}
