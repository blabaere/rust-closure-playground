use std::ops::{Add, Sub, Mul, Div};

fn evaluate<T : Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy>(values: &[T;9], constants: &[T;4]) -> T {
	values[0] + 
	((constants[0] * values[1]) / values[2]) + 
	values[3] + 
	(constants[1] * values[4]) - 
	values[5] - 
	constants[2] + 
	((values[6] * values[7]) / values[8]) - 
	constants[3]
}

const FLOAT_CONSTANTS: [f64;4]  = [13f64, 12f64, 11f64, 10f64];
//const INT_CONSTANTS: [isize;4]  = [13, 12, 11, 10];

/*fn evaluate(
	x1: f64,
	x2: f64,
	x3: f64,
	x4: f64,
	x5: f64,
	x6: f64,
	x7: f64,
	x8: f64,
	x9: f64) -> f64 {
	x1 + 
	(13f64 * x2 / x3) + 
	x4 + 
	(12f64 * x5) - 
	x6 - 
	11f64 + 
	(x7 * x8 / x9) - 
	10f64
}*/
// 13 12 11 10
//<T : Add + Sub + Mul + Div>
fn swap<T : Copy>(values: &mut [T;9], i: usize, j: usize) {
	let temp = values[i];

	values[i] = values[j];
	values[j] = temp;
}

fn permutation(values: &mut [f64;9], from: usize, to: usize) -> usize {
	if from == to {
		return check_result(values);
	}

	let mut count: usize = 0;
	for j in from..to {
		swap(values, from, j);
		count += permutation(values, from + 1, to);
		swap(values, from, j);
	} 

	return count;
}

fn check_result(values: &mut [f64;9]) -> usize {
	let result = evaluate(values, &FLOAT_CONSTANTS);
	if result == 66f64 {
		println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
			values[0],
			values[1],
			values[2],
			values[3],
			values[4],
			values[5],
			values[6],
			values[7],
			values[8]);
		return 1;
	}
	return 0;
}

fn main() {
	let mut float_values = [1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64, 9f64];
	let mut int_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
	let result_count = permutation(&mut float_values, 0, 9);

	println!("Result count: {:?}", result_count);
}