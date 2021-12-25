use std::fs;
use std::num::ParseIntError;
use std::str::{FromStr, Split};

fn parse_two_arguments(arg_iter: &mut Split<char>) -> (char, VarNum) {
	let first = arg_iter.next().unwrap().chars().next().unwrap();
	let second = {
		let next_part = arg_iter.next().unwrap();
		let val: Result<i64, ParseIntError> = next_part.parse();
		if let Ok(v) = val {
			VarNum::Number(v)
		} else {
			VarNum::Variable(Variable::from_char(next_part.chars().next().unwrap()))
		}
	};
	(first, second)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Variable {
	W,
	X,
	Y,
	Z,
}

impl Variable {
	fn from_char(var_char: char) -> Self {
		match var_char {
			'w' => Self::W,
			'x' => Self::X,
			'y' => Self::Y,
			'z' => Self::Z,
			_ => panic!("Invalid variable {}", var_char),
		}
	}

	fn var_ref<'a, T>(&self, w: &'a T, x: &'a T, y: &'a T, z: &'a T) -> &'a T {
		match self {
			Self::W => w,
			Self::X => x,
			Self::Y => y,
			Self::Z => z,
		}
	}

	fn var_mut_ref<'a, T>(&self, w: &'a mut T, x: &'a mut T, y: &'a mut T, z: &'a mut T) -> &'a mut T {
		match self {
			Self::W => w,
			Self::X => x,
			Self::Y => y,
			Self::Z => z,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum VarNum {
	Variable(Variable),
	Number(i64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
	Input,
	Add(Variable, VarNum),
	Multiply(Variable, VarNum),
	Divide(Variable, VarNum),
	Modulo(Variable, VarNum),
	Equal(Variable, VarNum),
}

impl Instruction {
	fn execute(&self, w: &mut i64, x: &mut i64, y: &mut i64, z: &mut i64) {
		let (var, result) = match self {
			Self::Input => unreachable!(), // This should never have input passed to it
			Self::Add(lhs, rhs) => {
				let lhs_ref = lhs.var_ref(w, x, y, z);
				(
					lhs,
					match rhs {
						VarNum::Variable(rhs_var) => {
							let rhs_ref = rhs_var.var_ref(w, x, y, z);
							*lhs_ref + *rhs_ref
						}
						VarNum::Number(rhs_num) => *lhs_ref + *rhs_num,
					},
				)
			}
			Self::Multiply(lhs, rhs) => {
				let lhs_ref = lhs.var_ref(w, x, y, z);
				(
					lhs,
					match rhs {
						VarNum::Variable(rhs_var) => {
							let rhs_ref = rhs_var.var_ref(w, x, y, z);
							*lhs_ref * *rhs_ref
						}
						VarNum::Number(rhs_num) => *lhs_ref * *rhs_num,
					},
				)
			}
			Self::Divide(lhs, rhs) => {
				let lhs_ref = lhs.var_ref(w, x, y, z);
				(
					lhs,
					match rhs {
						VarNum::Variable(rhs_var) => {
							let rhs_ref = rhs_var.var_ref(w, x, y, z);
							*lhs_ref / *rhs_ref
						}
						VarNum::Number(rhs_num) => *lhs_ref / *rhs_num,
					},
				)
			}
			Self::Modulo(lhs, rhs) => {
				let lhs_ref = lhs.var_ref(w, x, y, z);
				(
					lhs,
					match rhs {
						VarNum::Variable(rhs_var) => {
							let rhs_ref = rhs_var.var_ref(w, x, y, z);
							*lhs_ref % *rhs_ref
						}
						VarNum::Number(rhs_num) => *lhs_ref % *rhs_num,
					},
				)
			}
			Self::Equal(lhs, rhs) => {
				let lhs_ref = lhs.var_ref(w, x, y, z);
				let rhs_val = match rhs {
					VarNum::Variable(rhs_var) => *rhs_var.var_ref(w, x, y, z),
					VarNum::Number(rhs_num) => *rhs_num,
				};
				(lhs, if *lhs_ref == rhs_val { 1 } else { 0 })
			}
		};

		*var.var_mut_ref(w, x, y, z) = result;
	}
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(' ');
		let instruction = parts.next().unwrap();
		Ok(match instruction {
			"inp" => Self::Input,
			"add" => {
				let (lhs, rhs) = parse_two_arguments(&mut parts);
				let lhs = Variable::from_char(lhs);
				Self::Add(lhs, rhs)
			}
			"mul" => {
				let (lhs, rhs) = parse_two_arguments(&mut parts);
				let lhs = Variable::from_char(lhs);
				Self::Multiply(lhs, rhs)
			}
			"div" => {
				let (lhs, rhs) = parse_two_arguments(&mut parts);
				let lhs = Variable::from_char(lhs);
				Self::Divide(lhs, rhs)
			}
			"mod" => {
				let (lhs, rhs) = parse_two_arguments(&mut parts);
				let lhs = Variable::from_char(lhs);
				Self::Modulo(lhs, rhs)
			}
			"eql" => {
				let (lhs, rhs) = parse_two_arguments(&mut parts);
				let lhs = Variable::from_char(lhs);
				Self::Equal(lhs, rhs)
			}
			_ => panic!("Invalid instruction {}", instruction),
		})
	}
}

#[derive(Debug)]
struct StepManipulation {
	pop_stack: bool,
	add_x: i64,
	add_y: i64,
}

fn main() {
	let instructions: Vec<Instruction> = fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().unwrap())
		.collect();

	let mut instructions_per_input = Vec::new();
	let mut input_instructions = Vec::new();
	for instruction in instructions.iter() {
		if *instruction == Instruction::Input {
			if !input_instructions.is_empty() {
				instructions_per_input.push(input_instructions);
				input_instructions = Vec::new();
			}
		} else {
			input_instructions.push(instruction.clone());
		}
	}
	if !input_instructions.is_empty() {
		instructions_per_input.push(input_instructions);
	}

	let mut manipulations_per_input: Vec<StepManipulation> = Vec::new();
	for input_instructions in instructions_per_input.iter() {
		let mut pop_stack = false;
		let mut add_x = 0;
		let mut add_y = 0;
		for instruction in input_instructions.iter() {
			match instruction {
				Instruction::Divide(Variable::Z, VarNum::Number(26)) => pop_stack = true,
				Instruction::Add(Variable::X, VarNum::Number(add_num)) => add_x = *add_num,
				Instruction::Add(Variable::Y, VarNum::Number(add_num)) => add_y = *add_num,
				_ => (),
			}
		}
		manipulations_per_input.push(StepManipulation {
			pop_stack,
			add_x,
			add_y,
		});
	}

	println!("{:?}", manipulations_per_input);

	let mut z = Vec::new();
	let mut inputs = vec![0; manipulations_per_input.len()];
	for (step_num, step) in manipulations_per_input.iter().enumerate() {
		if step.pop_stack {
			let (y, initial_step) = z.pop().unwrap();
			let x = step.add_x;
			let offset = y + x;
			println!("Offset: {}", offset);
			let mut initial_input = 1;
			let mut current_input = initial_input + offset;
			if current_input < 1 {
				let fix_offset = current_input - 1;
				current_input -= fix_offset;
				initial_input -= fix_offset;
			}
			inputs[step_num] = current_input;
			inputs[initial_step] = initial_input;
		} else {
			z.push((step.add_y, step_num));
		}
		println!("{:?}", z);
	}
	println!("{:?}", inputs);

	// Verification step
	let mut w = 0;
	let mut x = 0;
	let mut y = 0;
	let mut z = 0;
	for (input_num, input_instructions) in instructions_per_input.iter().enumerate() {
		w = inputs[input_num];
		for instruction in input_instructions.iter() {
			instruction.execute(&mut w, &mut x, &mut y, &mut z);
		}
	}
	println!("w: {}; x: {}; y: {}; z: {}", w, x, y, z);
}
