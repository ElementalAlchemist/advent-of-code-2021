use std::fs;

fn character_score(error: char) -> u64 {
	match error {
		')' => 1,
		']' => 2,
		'}' => 3,
		'>' => 4,
		_ => 0,
	}
}

fn main() {
	let chunk_lines: Vec<String> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(String::from)
		.collect();

	let mut scores: Vec<u64> = Vec::new();
	'lines: for line in chunk_lines.iter() {
		let mut char_stack: Vec<char> = Vec::new();
		for character in line.chars() {
			if character == '(' {
				char_stack.push(')');
			} else if character == '[' {
				char_stack.push(']');
			} else if character == '{' {
				char_stack.push('}');
			} else if character == '<' {
				char_stack.push('>');
			} else {
				let expected_character = char_stack.pop();
				if let Some(exp_char) = expected_character {
					if character != exp_char {
						continue 'lines;
					}
				} else {
					continue 'lines;
				}
			}
		}

		let mut line_score: u64 = 0;
		while let Some(next_char) = char_stack.pop() {
			line_score *= 5;
			line_score += character_score(next_char);
		}
		scores.push(line_score);
	}

	scores.sort_unstable();
	let score = scores[scores.len() / 2];
	println!("{}", score);
}
