use std::fs;

fn main() {
	let segment_displays: Vec<Vec<String>> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| {
			s.split(" | ")
				.nth(1)
				.unwrap()
				.split(' ')
				.map(String::from)
				.collect()
		})
		.collect();
	let mut unique_count: u32 = 0;
	for display_output in segment_displays.iter() {
		for display in display_output.iter() {
			match display.len() {
				2 | 3 | 4 | 7 => unique_count += 1,
				_ => (),
			}
		}
	}
	println!("{}", unique_count);
}
