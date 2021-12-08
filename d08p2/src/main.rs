use std::fs;

fn digit_is_zero(digit: &str, one_segments: &str, four_segments: &str) -> bool {
	digit.len() == 6
		&& one_segments.chars().all(|c| digit.contains(c))
		&& four_segments.chars().any(|c| !digit.contains(c))
}

fn digit_is_one(digit: &str) -> bool {
	digit.len() == 2
}

fn digit_is_two(digit: &str, four_segments: &str) -> bool {
	digit.len() == 5 && four_segments.chars().filter(|c| digit.contains(*c)).count() == 2
}

fn digit_is_three(digit: &str, one_segments: &str) -> bool {
	digit.len() == 5 && one_segments.chars().all(|c| digit.contains(c))
}

fn digit_is_four(digit: &str) -> bool {
	digit.len() == 4
}

fn digit_is_five(digit: &str, one_segments: &str, four_segments: &str) -> bool {
	digit.len() == 5
		&& one_segments.chars().any(|c| !digit.contains(c))
		&& four_segments.chars().filter(|c| digit.contains(*c)).count() == 3
}

fn digit_is_six(digit: &str, one_segments: &str) -> bool {
	digit.len() == 6 && one_segments.chars().any(|c| !digit.contains(c))
}

fn digit_is_seven(digit: &str) -> bool {
	digit.len() == 3
}

fn digit_is_eight(digit: &str) -> bool {
	digit.len() == 7
}

fn digit_is_nine(digit: &str, four_segments: &str) -> bool {
	digit.len() == 6 && four_segments.chars().all(|c| digit.contains(c))
}

fn main() {
	let segment_displays: Vec<[Vec<String>; 2]> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| {
			let mut part_iter = s.split(" | ");
			[
				part_iter.next().unwrap().split(' ').map(String::from).collect(),
				part_iter.next().unwrap().split(' ').map(String::from).collect(),
			]
		})
		.collect();

	let mut output_sum: u32 = 0;
	for output in segment_displays.iter() {
		let mut one_segments = String::new();
		let mut four_segments = String::new();
		for digit in output[0].iter() {
			if digit.len() == 2 {
				one_segments = digit.clone();
			} else if digit.len() == 4 {
				four_segments = digit.clone();
			}
		}
		let mut number: u32 = 0;
		for digit in output[1].iter() {
			number *= 10;
			number += if digit_is_zero(digit, &one_segments, &four_segments) {
				0
			} else if digit_is_one(digit) {
				1
			} else if digit_is_two(digit, &four_segments) {
				2
			} else if digit_is_three(digit, &one_segments) {
				3
			} else if digit_is_four(digit) {
				4
			} else if digit_is_five(digit, &one_segments, &four_segments) {
				5
			} else if digit_is_six(digit, &one_segments) {
				6
			} else if digit_is_seven(digit) {
				7
			} else if digit_is_eight(digit) {
				8
			} else if digit_is_nine(digit, &four_segments) {
				9
			} else {
				panic!("Unknown digit: {}", digit);
			};
		}
		output_sum += number;
	}

	println!("{}", output_sum);
}
