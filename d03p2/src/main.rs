use std::fs;

const BITS_PER_LINE: usize = 12;

fn num_from_bits(input: &[bool]) -> u32 {
	let mut num: u32 = 0;
	for bit in input.iter() {
		num <<= 1;
		if *bit {
			num += 1;
		}
	}
	num
}

fn main() {
	let report: Vec<[bool; BITS_PER_LINE]> = {
		let input: Vec<String> = fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
			.map(|s| s.trim().to_owned())
			.collect();
		let mut report = Vec::new();
		for line in input {
			let mut line_data = [false; BITS_PER_LINE];
			for (index, bit) in line.chars().enumerate() {
				if bit == '1' {
					line_data[index] = true;
				}
			}
			report.push(line_data);
		}
		report
	};

	let mut oxygen: Vec<[bool; BITS_PER_LINE]> = report.clone();
	let mut co2: Vec<[bool; BITS_PER_LINE]> = report.clone();
	let mut oxygen_rating: Option<u32> = None;
	let mut co2_rating: Option<u32> = None;
	for i in 0..BITS_PER_LINE {
		let mut num_true: usize = 0;
		let mut num_false: usize = 0;
		for values in oxygen.iter() {
			if values[i] {
				num_true += 1;
			} else {
				num_false += 1;
			}
		}
		if num_true >= num_false {
			oxygen = oxygen.iter().filter(|v| v[i]).copied().collect();
		} else {
			oxygen = oxygen.iter().filter(|v| !v[i]).copied().collect();
		}
		if oxygen.len() == 1 {
			oxygen_rating = Some(num_from_bits(&oxygen[0]));
		}

		num_true = 0;
		num_false = 0;
		for values in co2.iter() {
			if values[i] {
				num_true += 1;
			} else {
				num_false += 1;
			}
		}
		if num_true < num_false {
			co2 = co2.iter().filter(|v| v[i]).copied().collect();
		} else {
			co2 = co2.iter().filter(|v| !v[i]).copied().collect();
		}
		if co2.len() == 1 {
			co2_rating = Some(num_from_bits(&co2[0]));
		}
	}

	let life_support = oxygen_rating.unwrap() * co2_rating.unwrap();
	println!("{}", life_support);
}
