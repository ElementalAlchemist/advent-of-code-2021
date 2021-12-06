use std::fs;

fn main() {
	let mut days_to_birth: Vec<u8> = fs::read_to_string("input.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect();

	for _ in 0..80 {
		let mut new_tracker: Vec<u8> = Vec::new();
		for fish_days in days_to_birth.iter() {
			if *fish_days == 0 {
				new_tracker.push(6);
				new_tracker.push(8);
			} else {
				new_tracker.push(*fish_days - 1);
			}
		}
		days_to_birth = new_tracker;
	}

	println!("{}", days_to_birth.len());
}
