use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main()
{
	let raw_maps = read_vecs::<String>(
		// std::path::Path::new("./data/small_input.txt"),
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut part_1_result = u64::MAX;
	let mut part_2_result = u64::MAX;

	let mut raw_maps_iter = raw_maps.iter();

	// Unpack seeds
	let seeds = raw_maps_iter
		.next().unwrap()            // Get the first vector
		.first().unwrap()           // Get the first string from that vector (seed string vector has only one vector)
		.split(':').last().unwrap() // Get part after the ':'
		.trim()                     // Remove leading & trailing whitespace
		.split(' ').map(|raw_seed| raw_seed.parse::<u64>().unwrap()).collect::<Vec<u64>>();

	// Unpack mappings
	let mut mappings = vec![];
	
	loop
	{
		match raw_maps_iter.next()
		{
			Some(map) => { mappings.push(create_map_from_vec(&map)); },
			None      => break
		}
	}

	// Traverse mappings
	for seed in &seeds
	{
		let mut working_value = *seed;

		for mapping in &mappings
		{
			for partial_mapping in mapping
			{
				let mapping_start = partial_mapping.1;
				let mapping_end   = mapping_start + partial_mapping.2;
				if mapping_start <= working_value && working_value <= mapping_end
				{
					working_value = partial_mapping.0 + (working_value - mapping_start);
					break;
				}
			}
		}

		part_1_result = std::cmp::min(part_1_result, working_value);
	}

	// Part 2

	for i in (0..seeds.len()).step_by(2)
	{
		println!("{}", i);
		for seed in seeds[i]..seeds[i]+seeds[i+1]
		{
			let mut working_value = seed;

			for mapping in &mappings
			{
				for partial_mapping in mapping
				{
					let mapping_start = partial_mapping.1;
					let mapping_end   = mapping_start + partial_mapping.2;
					if mapping_start <= working_value && working_value <= mapping_end
					{
						working_value = partial_mapping.0 + (working_value - mapping_start);
						break;
					}
				}
			}

			part_2_result = std::cmp::min(part_1_result, working_value);
		}
	}

	// Output
	println!("Part 1 - Lowest location number: {}", part_1_result);
	println!("Part 2 - Lowest location number: {}", part_2_result);
}

pub fn create_map_from_vec
(
	vector: &Vec<String>
)
-> Vec<(u64, u64, u64)>
{
	vector
		.iter()
		.skip(1)
		.map(|map_string|
			(
				map_string.split(' ').nth(0).unwrap().parse::<u64>().unwrap(),
				map_string.split(' ').nth(1).unwrap().parse::<u64>().unwrap(),
				map_string.split(' ').nth(2).unwrap().parse::<u64>().unwrap()
			)
		)
		.collect::<Vec<(u64, u64, u64)>>()
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