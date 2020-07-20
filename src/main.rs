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
}
