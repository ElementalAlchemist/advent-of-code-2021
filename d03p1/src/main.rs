use std::fs;

const BITS_PER_LINE: usize = 12;

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

	let mut gamma: u32 = 0;
	let mut epsilon: u32 = 0;
	for i in 0..BITS_PER_LINE {
		let mut num_true: usize = 0;
		let mut num_false: usize = 0;
		for values in report.iter() {
			if values[i] {
				num_true += 1;
			} else {
				num_false += 1;
			}
		}
		gamma *= 2;
		epsilon *= 2;
		if num_true > num_false {
			gamma += 1;
		} else {
			epsilon += 1;
		}
	}
	println!("{}", gamma * epsilon);
}
