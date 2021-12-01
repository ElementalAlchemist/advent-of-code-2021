use std::fs;

fn main() {
	let values: Vec<u32> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.trim().parse().unwrap())
		.collect();
	let mut value_iter = values.iter();
	let mut previous_value = *value_iter.next().unwrap();
	let mut increase_count: u32 = 0;
	for value in value_iter {
		if *value > previous_value {
			increase_count += 1;
		}
		previous_value = *value;
	}
	println!("{}", increase_count);
}
