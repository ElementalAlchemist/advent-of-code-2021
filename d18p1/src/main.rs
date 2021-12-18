use std::fmt::Display;
use std::fs;

#[derive(Clone, Debug)]
enum NumberPart {
	Number(u32),
	Pair(Box<SnailfishNumber>),
}

impl NumberPart {
	fn magnitude(&self) -> u32 {
		match &self {
			Self::Number(num) => *num,
			Self::Pair(num) => num.magnitude(),
		}
	}
}

impl Display for NumberPart {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Number(val) => write!(f, "{}", *val),
			Self::Pair(val) => write!(f, "{}", **val),
		}
	}
}

#[derive(Default)]
struct ExplodeResult {
	exploded: bool,
	add_left: Option<u32>,
	add_right: Option<u32>,
}

#[derive(Clone, Debug)]
struct SnailfishNumber {
	x: NumberPart,
	y: NumberPart,
}

impl SnailfishNumber {
	fn new(number_str: &str) -> Self {
		let mut depth: u32 = 0;
		let mut start_point: usize = 0;

		let mut x: Option<NumberPart> = None;
		let mut y: Option<NumberPart> = None;

		for (index, character) in number_str.chars().enumerate() {
			if character == '[' {
				if depth == 0 {
					start_point = index + 1;
				}
				depth += 1;
			} else if character == ']' {
				depth -= 1;
				if depth == 0 {
					if x.is_none() {
						x = Some(NumberPart::Pair(Box::new(Self::new(&number_str[start_point..index]))));
					} else {
						y = Some(NumberPart::Pair(Box::new(Self::new(&number_str[start_point..index]))));
					}
				}
			} else if character != ',' && depth == 0 {
				if x.is_none() {
					x = Some(NumberPart::Number((character as u8 - 48) as u32));
				} else {
					y = Some(NumberPart::Number((character as u8 - 48) as u32));
				}
			}
		}

		let x = x.unwrap();
		let y = y.unwrap();

		Self { x, y }
	}

	fn reduce(&mut self) {
		loop {
			let explode_result = self.explode(1);
			if explode_result.exploded {
				println!("X -> {}", self);
				continue;
			}
			let did_split = self.split();
			if did_split {
				println!("= -> {}", self);
				continue;
			}
			break;
		}
	}

	fn explode(&mut self, depth: u32) -> ExplodeResult {
		if depth >= 4 {
			let mut explode_result = ExplodeResult::default();
			self.x = if let NumberPart::Pair(number) = &self.x {
				let add_left = if let NumberPart::Number(x) = number.x {
					Some(x)
				} else {
					panic!("Got too deep (exploding number {})", number);
				};
				let add_y = if let NumberPart::Number(y) = number.y {
					y
				} else {
					panic!("Got too deep (exploding number {})", number);
				};
				match &mut self.y {
					NumberPart::Number(y) => *y += add_y,
					NumberPart::Pair(y_number) => y_number.add_regular_leftmost(add_y),
				}
				explode_result.exploded = true;
				explode_result.add_left = add_left;
				NumberPart::Number(0)
			} else {
				self.x.clone()
			};
			if explode_result.exploded {
				return explode_result;
			}

			self.y = if let NumberPart::Pair(number) = &self.y {
				let add_right = if let NumberPart::Number(y) = number.y {
					Some(y)
				} else {
					panic!("Got too deep (exploding number {})", number);
				};
				let add_x = if let NumberPart::Number(x) = number.x {
					x
				} else {
					panic!("Got too deep (exploding number {})", number);
				};
				match &mut self.x {
					NumberPart::Number(x) => *x += add_x,
					NumberPart::Pair(x_number) => x_number.add_regular_rightmost(add_x),
				}
				explode_result.exploded = true;
				explode_result.add_right = add_right;
				NumberPart::Number(0)
			} else {
				self.y.clone()
			};
			if explode_result.exploded {
				return explode_result;
			}
		}
		if let NumberPart::Pair(number) = &mut self.x {
			let mut explode_result = number.explode(depth + 1);
			if explode_result.exploded {
				if let Some(add_y) = explode_result.add_right.take() {
					match &mut self.y {
						NumberPart::Number(y) => *y += add_y,
						NumberPart::Pair(y_number) => y_number.add_regular_leftmost(add_y),
					}
				}
				return explode_result;
			}
		}
		if let NumberPart::Pair(number) = &mut self.y {
			let mut explode_result = number.explode(depth + 1);
			if explode_result.exploded {
				if let Some(add_x) = explode_result.add_left.take() {
					match &mut self.x {
						NumberPart::Number(x) => *x += add_x,
						NumberPart::Pair(x_number) => x_number.add_regular_rightmost(add_x),
					}
				}
				return explode_result;
			}
		}

		ExplodeResult {
			exploded: false,
			add_left: None,
			add_right: None,
		}
	}

	fn add_regular_rightmost(&mut self, value: u32) {
		match &mut self.y {
			NumberPart::Number(val) => *val += value,
			NumberPart::Pair(number) => number.add_regular_rightmost(value),
		}
	}

	fn add_regular_leftmost(&mut self, value: u32) {
		match &mut self.x {
			NumberPart::Number(val) => *val += value,
			NumberPart::Pair(number) => number.add_regular_leftmost(value),
		}
	}

	fn split(&mut self) -> bool {
		let mut did_split = false;
		self.x = match &mut self.x {
			NumberPart::Number(x) => {
				if *x >= 10 {
					did_split = true;
					let left = *x / 2;
					let right = left + (*x % 2);
					let left = NumberPart::Number(left);
					let right = NumberPart::Number(right);
					let number = SnailfishNumber { x: left, y: right };
					NumberPart::Pair(Box::new(number))
				} else {
					NumberPart::Number(*x)
				}
			}
			NumberPart::Pair(x_number) => {
				did_split = x_number.split();
				NumberPart::Pair(x_number.clone())
			}
		};
		if did_split {
			return true;
		}
		self.y = match &mut self.y {
			NumberPart::Number(y) => {
				if *y >= 10 {
					did_split = true;
					let left = *y / 2;
					let right = left + (*y % 2);
					let left = NumberPart::Number(left);
					let right = NumberPart::Number(right);
					let number = SnailfishNumber { x: left, y: right };
					NumberPart::Pair(Box::new(number))
				} else {
					NumberPart::Number(*y)
				}
			}
			NumberPart::Pair(y_number) => {
				did_split = y_number.split();
				NumberPart::Pair(y_number.clone())
			}
		};
		did_split
	}

	fn add(&self, other: &Self) -> Self {
		let mut new_number = SnailfishNumber {
			x: NumberPart::Pair(Box::new(self.clone())),
			y: NumberPart::Pair(Box::new(other.clone())),
		};
		new_number.reduce();
		new_number
	}

	fn magnitude(&self) -> u32 {
		let left = 3 * self.x.magnitude();
		let right = 2 * self.y.magnitude();
		left + right
	}
}

impl Display for SnailfishNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{},{}]", self.x, self.y)
	}
}

fn main() {
	let number_string_list: Vec<String> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.strip_prefix('[').unwrap().strip_suffix(']').unwrap())
		.map(String::from)
		.collect();

	let mut numbers: Vec<SnailfishNumber> = Vec::new();
	for number_str in number_string_list {
		numbers.push(SnailfishNumber::new(&number_str));
	}

	let mut number = numbers.remove(0);
	for next_number in numbers.iter() {
		number = number.add(next_number);
		println!("{}", number);
	}
	println!("Magnitude: {}", number.magnitude());
}
