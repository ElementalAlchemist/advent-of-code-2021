const MIN_TARGET_Y: i32 = -142;

fn main() {
	// only works with negative Y
	let up_unit = MIN_TARGET_Y.abs() - 1;
	let up_distance = (up_unit * (up_unit + 1)) / 2;
	println!("{}", up_distance);
}
