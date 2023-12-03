use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use regex::Regex;

pub const MAX_RED:   usize = 12;
pub const MAX_GREEN: usize = 13;
pub const MAX_BLUE:  usize = 14;

fn main() 
{
	let raw_game_strings = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut part_1_result = 0;
	let mut part_2_result = 0;

	let mut games = HashMap::new();

	for raw_game_string in raw_game_strings
	{
		let removed_game_vec    = raw_game_string.split("Game ").collect::<Vec<&str>>();
		let removed_game_string = removed_game_vec.last().unwrap().to_string();
		
		let new_game_vec  = removed_game_string.split(':').collect::<Vec<&str>>();
		let new_game_sets = new_game_vec.iter().last().unwrap().to_string();

		let     game_id = new_game_vec.iter().next().unwrap().parse::<usize>().unwrap();
		let mut game    = vec![];
		
		for new_game_set in new_game_sets.split(";")
		{
			let mut game_set = HashMap::new();
			for color_string in ["red", "green", "blue"]
			{
				let regex = Regex::new((String::from(r"\d+ ") + color_string).as_str()).unwrap();
				let mut color_value = 0;
				if let Some(regex_result) = regex.find(new_game_set)
				{
					color_value = regex_result.as_str().split(' ').next().unwrap().parse::<usize>().unwrap();
				}

				game_set.insert(color_string, color_value);
			}
			game.push(game_set);
		}
		games.insert(game_id, game);
	}

	// Part 1	
	for game in &games
	{
		let mut is_possible = true;
		
		for game_set in game.1
		{
			if 
				game_set.get("red"  ).unwrap_or(&0) > &(MAX_RED) ||
				game_set.get("green").unwrap_or(&0) > &(MAX_GREEN) ||
				game_set.get("blue" ).unwrap_or(&0) > &(MAX_BLUE)
			{
				is_possible = false;
			}
		}

		if is_possible
		{
			part_1_result += game.0
		}
	}
	
	// Part 2
	for game in &games
	{
		let mut min_red   = 0;
		let mut min_green = 0;
		let mut min_blue  = 0;

		for game_set in game.1
		{
			min_red   = std::cmp::max(min_red,   *game_set.get("red"  ).unwrap_or(&0));
			min_green = std::cmp::max(min_green, *game_set.get("green").unwrap_or(&0));
			min_blue  = std::cmp::max(min_blue,  *game_set.get("blue" ).unwrap_or(&0));
		}

		let game_power = min_red * min_green * min_blue;
		part_2_result += game_power;
	}

	// Output
	println!("Part 1 - Sum of Game IDs of possible Games: {}", part_1_result);
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