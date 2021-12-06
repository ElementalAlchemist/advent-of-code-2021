use std::fs;

fn main() {
	let days_to_birth: Vec<usize> = fs::read_to_string("input.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect();

	let mut num_fish_at_count: [usize; 9] = [0; 9];
	for fish in days_to_birth.iter() {
		num_fish_at_count[*fish] += 1;
	}

	for _ in 0..256 {
		let mut new_tracker: [usize; 9] = [0; 9];
		for (days, num_fish) in num_fish_at_count.iter().enumerate() {
			if days == 0 {
				new_tracker[6] += *num_fish;
				new_tracker[8] += *num_fish;
			} else {
				new_tracker[days - 1] += *num_fish;
			}
		}
		num_fish_at_count = new_tracker;
	}

	let total: usize = num_fish_at_count.iter().copied().sum();
	println!("{}", total);
}
