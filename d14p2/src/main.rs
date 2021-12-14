use std::collections::HashMap;
use std::fs;

fn apply_rules(pair_counts: &HashMap<String, usize>, rules: &HashMap<String, char>) -> HashMap<String, usize> {
	let mut result = HashMap::new();
	for (pair, count) in pair_counts.iter() {
		if let Some(insert_element) = rules.get(pair) {
			let mut pair_chars = pair.chars();
			let new_pair = format!("{}{}", pair_chars.next().unwrap(), *insert_element);
			*result.entry(new_pair).or_default() += count;
			let new_pair = format!("{}{}", *insert_element, pair_chars.next().unwrap());
			*result.entry(new_pair).or_default() += count;
		} else {
			*result.entry(pair.clone()).or_default() += count;
		}
	}
	result
}

fn main() {
	let (template, insertion_rules) = {
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

	let mut pair_counts: HashMap<String, usize> = HashMap::new();
	for elements in template.windows(2) {
		let pair = format!("{}{}", elements[0], elements[1]);
		*pair_counts.entry(pair).or_default() += 1;
	}

	for _ in 0..40 {
		pair_counts = apply_rules(&pair_counts, &insertion_rules);
	}

	let mut element_counts: HashMap<char, usize> = HashMap::new();
	for (elements, count) in pair_counts.iter() {
		for element in elements.chars() {
			*element_counts.entry(element).or_default() += count;
		}
	}
	let most_quantity = element_counts.values().max().unwrap() / 2;
	let least_quantity = element_counts.values().min().unwrap() / 2;
	println!("{}", most_quantity - least_quantity + 1);
}
