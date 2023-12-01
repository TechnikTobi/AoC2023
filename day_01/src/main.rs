use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() 
{
	let calibration_strings = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut calibration_values = vec![];

	// Part 1:

	for line in &calibration_strings
	{
		calibration_values.push(two_digit_from_string(line));
	}
	
	let part_1_result = calibration_values.iter().sum::<u32>();

	// Part 2:

	let string_to_digit_map = HashMap::from([
		("one",   "1"),
		("two",   "2"),
		("three", "3"),
		("four",  "4"),
		("five",  "5"),
		("six",   "6"),
		("seven", "7"),
		("eight", "8"),
		("nine",  "9"),
		("zero",  "0"),
	]);

	calibration_values.clear();

	for line in &calibration_strings
	{
		let mut replaced_line_from_start = String::new(); 
		let mut replaced_line_from_end   = String::new();

		for character in line.chars()
		{
			replaced_line_from_start.push(character);
			for (word, digit) in &string_to_digit_map
			{
				replaced_line_from_start = replaced_line_from_start.replace(word, digit);
			}
		}

		for character in line.chars().rev()
		{
			replaced_line_from_end = character.to_string() + replaced_line_from_end.as_str();
			for (word, digit) in &string_to_digit_map
			{
				replaced_line_from_end = replaced_line_from_end.replace(word, digit);
			}
		}

		let digits_from_start = replaced_line_from_start.chars().filter(|c| c.is_digit(10)).collect::<String>();
		let digits_from_end   =   replaced_line_from_end.chars().filter(|c| c.is_digit(10)).collect::<String>();
		let first_digit = digits_from_start.chars().next().unwrap().to_digit(10).unwrap();
		let last_digit  =   digits_from_end.chars().last().unwrap().to_digit(10).unwrap();
		let calibration_value = first_digit * 10 + last_digit;

		calibration_values.push(calibration_value);
	}

	let part_2_result = calibration_values.iter().sum::<u32>();

	println!("Part 1 - Summed calibration values: {}", part_1_result);
	println!("Part 2 - Summed calibration values: {}", part_2_result);
}


pub fn read_vecs
<T: FromStr + std::default::Default + std::fmt::Display + std::clone::Clone>
(
	path: &std::path::Path
)
-> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Vec::<T>>::new();

	let mut temp = Vec::<T>::new();

	for result_line in lines
	{
		let line = result_line?;

		if line == ""
		{
			data.push(temp.clone());
			temp.clear();
		}
		else
		{
			temp.push(T::from_str(&line).unwrap_or_default());
		}

	}	

	return Ok(data);
}

pub fn
two_digit_from_string
(
	line: &str
)
-> u32
{
	let digits = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
	let first_digit = digits.chars().next().unwrap().to_digit(10).unwrap();
	let last_digit  = digits.chars().last().unwrap().to_digit(10).unwrap();
	let calibration_value = first_digit * 10 + last_digit;
	return calibration_value;
}

pub fn read_string_data
(
	path: &std::path::Path
)
-> Result<Vec<String>, Box<dyn std::error::Error>>
{
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<String>::new();

	for result_line in lines
	{
		let line = result_line?;
		data.push(line);
	}

	return Ok(data);	
}