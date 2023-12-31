use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() 
{
	let raw_lines = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let part_1_result;
	let mut part_2_result = 0;

	let mut current_number_buffer = String::new();
	let mut current_number_good_neighbors = vec![];

	let min_x = 0;
	let max_x = raw_lines.iter().next().unwrap().len() as i32 - 1;
	let min_y = 0;
	let max_y = raw_lines.len() as i32 - 1;

	let mut numbers = vec![];

	for y in 0..raw_lines.len()
	{
		for x in 0..raw_lines[y].len()
		{
			let character = raw_lines.iter().nth(y).unwrap().chars().nth(x).unwrap();

			if character.is_digit(10)
			{
				current_number_buffer.push(character);
				
				for env_y in std::cmp::max(min_y, y as i32-1)..=std::cmp::min(max_y, y  as i32+1)
				{
					for env_x in std::cmp::max(min_x, x as i32-1)..=std::cmp::min(max_x, x as i32+1)
					{
						let neighbor = raw_lines.iter().nth(env_y as usize).unwrap().chars().nth(env_x as usize).unwrap();
						if neighbor != '.' && !neighbor.is_digit(10)
						{
							current_number_good_neighbors.push((env_x, env_y));
						}
					}
				}
			}

			if !character.is_digit(10) && current_number_buffer.len() > 0
			{
				if current_number_good_neighbors.len() > 0
				{
					numbers.push((current_number_buffer.parse::<i32>().unwrap(), current_number_good_neighbors.clone()));
				}
				current_number_buffer.clear();
				current_number_good_neighbors.clear();
			}
		}
	}

	part_1_result = numbers.iter().map(|(number, _)| number).sum::<i32>();

	for y in 0..raw_lines.len()
	{
		for x in 0..raw_lines[y].len()
		{
			let character = raw_lines.iter().nth(y).unwrap().chars().nth(x).unwrap();

			if character != '*'
			{
				continue;
			}

			let mut part_numbers = vec![];

			for (number, neighbors) in &numbers
			{
				if neighbors.contains(&(x as i32, y as i32))
				{
					part_numbers.push(number);
				}
			}

			assert!(part_numbers.len() > 0 && part_numbers.len() <= 2);

			if part_numbers.len() == 2
			{
				part_2_result += part_numbers[0] * part_numbers[1];
			}
		}
	}

	// Output
	println!("Part 1 - Sum of Part Numbers: {}", part_1_result);
	println!("Part 2 - Sum of Power of Game Sets: {}", part_2_result);
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

	data.push(temp.clone());

	return Ok(data);
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