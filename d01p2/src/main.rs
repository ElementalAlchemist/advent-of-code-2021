use std::fs;

fn main() {
	let values: Vec<u32> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.trim().parse().unwrap())
		.collect();
	let mut value_iter = values.iter();
	let mut previous_value_1 = *value_iter.next().unwrap();
	let mut previous_value_2 = *value_iter.next().unwrap();
	let mut previous_value_3 = *value_iter.next().unwrap();
	let mut increase_count: u32 = 0;
	for value in value_iter {
		if *value > previous_value_1 {
			increase_count += 1;
		}
		previous_value_1 = previous_value_2;
		previous_value_2 = previous_value_3;
		previous_value_3 = *value;
	}
	println!("{}", increase_count);
}
