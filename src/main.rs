
mod memory;
mod bit;

use std::io::Read;

use memory::Memory;
use bit::Bit;

fn main() -> std::io::Result<()> {

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

	// read file to executable
	let mut file = std::fs::File::open(file_name).expect("File does not exist");
	let mut executable = Vec::<u8>::new();
	file.read_to_end(&mut executable)?;

	// 2 Kb of RAM
	const RAM_SIZE : usize = 2_000; // TODO: make this configurable
	let mut ram = Memory::from_executable_and_size(executable, RAM_SIZE);

	Ok(())
}
