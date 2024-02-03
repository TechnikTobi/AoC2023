mod input;

use crate::input::*;

fn main() 
{
	let input_vec = read_vecs::<u64>(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	
	println!("Hello, world!");
}
