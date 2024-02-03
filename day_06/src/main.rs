use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn main() 
{
	let input_path = std::path::Path::new("./data/input.txt");	
	let times = read_vec_in_line(input_path, 0);
	let dists = read_vec_in_line(input_path, 1);

	println!("{:?}", times);
	println!("{:?}", dists);
}

pub fn read_vec_in_line
(
	path: &std::path::Path,
	n:    usize	
)
-> Vec<u64>
{
	read_line(path, n).unwrap()
		.split(':').last().unwrap()
		.trim().split(' ')
		.map(|x| x.trim())
		.filter(|x| x.len() > 0)
		.map(|x| x.parse::<u64>().unwrap())
		.collect::<Vec<u64>>()
}

pub fn read_line
(
	path: &std::path::Path,
	n:    usize
)
-> Result<String, Box<dyn std::error::Error>>
{
	let file  = File::open(path)?;
	let lines = BufReader::new(file).lines();

	for (i, result_line) in lines.enumerate()
	{
		if i == n
		{
			return Ok(result_line?);
		}
	}

	return Ok("".to_string());
}
