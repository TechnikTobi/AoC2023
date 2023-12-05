use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

fn main()
{
	let raw_cards = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut part_1_result = 0;
	let part_2_result;

	let mut card_counts = HashMap::new();
	for i in 1..=raw_cards.len()
	{
		card_counts.insert(i, 1);
	}
	let mut current_card_counter = 1;

	for raw_card in &raw_cards
	{
		let all_values = raw_card.split(':').last().unwrap();
		let mut left_and_right = all_values.split('|');
		let raw_winning_numbers  = left_and_right.next().unwrap();
		let raw_my_numbers       = left_and_right.next().unwrap();

		let winning_numbers = raw_winning_numbers.split(' ').map(|raw_number| raw_number.parse::<i32>()).collect::<Vec<Result<i32, ParseIntError>>>();
		let      my_numbers =      raw_my_numbers.split(' ').map(|raw_number| raw_number.parse::<i32>()).collect::<Vec<Result<i32, ParseIntError>>>();

		let mut local_card_counter = 0 as usize;
		let additative_card_count = card_counts.get(&current_card_counter).unwrap().clone();

		for my_number in my_numbers
		{
			if my_number.is_err()
			{
				continue;
			}

			if winning_numbers.contains(&my_number)
			{
				local_card_counter += 1;
				if let Some(card) = card_counts.get_mut(&(current_card_counter + local_card_counter)) {
					*card += additative_card_count;
				}
			}
		}

		part_1_result += 2u32.pow(local_card_counter as u32) / 2;
		current_card_counter += 1;
	}

	part_2_result = card_counts.values().sum::<u32>();


	// Output
	println!("Part 1 - Winnings: {}", part_1_result);
	println!("Part 2 - Card Counts: {}", part_2_result);
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