use std::collections::HashMap;
use std::fs;

fn apply_rules(template: &[char], rules: &HashMap<String, char>) -> Vec<char> {
	let mut result = Vec::new();
	for values in template.windows(2) {
		let test = format!("{}{}", values[0], values[1]);
		result.push(values[0]);
		if let Some(insert_element) = rules.get(&test) {
			result.push(*insert_element);
		}
	}
	result.push(*template.last().unwrap());
	result
}

fn main() {
	let (mut template, insertion_rules) = {
		let input: Vec<String> = fs::read_to_string("input.txt")
			.unwrap()
			.lines()
			.filter(|s| !s.is_empty())
			.map(String::from)
			.collect();
		let mut template: Vec<char> = Vec::new();
		let mut insertion_rules: HashMap<String, char> = HashMap::new();
		for value in input {
			if value.contains(" -> ") {
				let mut value_iter = value.split(" -> ");
				let pair = value_iter.next().unwrap();
				let element = value_iter.next().unwrap();
				insertion_rules.insert(String::from(pair), element.chars().next().unwrap());
			} else {
				template = value.chars().collect();
			}
		}
		(template, insertion_rules)
	};

	for _ in 0..10 {
		template = apply_rules(&template, &insertion_rules);
	}

	let mut element_counts: HashMap<char, usize> = HashMap::new();
	for element in template.iter() {
		*element_counts.entry(*element).or_default() += 1;
	}
	let most_quantity = element_counts.values().max().unwrap();
	let least_quantity = element_counts.values().min().unwrap();
	println!("{}", *most_quantity - *least_quantity);
}
