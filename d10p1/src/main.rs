use std::fs;

fn character_score(error: char) -> u32 {
	match error {
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
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

	let mut score: u32 = 0;
	for line in chunk_lines.iter() {
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
						score += character_score(character);
						break;
					}
				} else {
					score += character_score(character);
					break;
				}
			}
		}
	}

	println!("{}", score);
}
