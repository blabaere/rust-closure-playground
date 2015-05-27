use std::ops::{Add, Sub, Mul, Div};

fn evaluate<T>(values: &[T;9], constants: &[T;4]) -> T where
	T : Add<Output = T>,
	T : Sub<Output = T>,
	T : Mul<Output = T>,
	T : Div<Output = T>,
	T : Copy {

	values[0] + 
	(constants[0] * values[1] / values[2]) + 
	values[3] + 
	(constants[1] * values[4]) - 
	values[5] - 
	constants[2] + 
	(values[6] * values[7] / values[8]) - 
	constants[3]

}

const FLOAT_CONSTANTS: [f64;4]  = [13f64, 12f64, 11f64, 10f64];
const INT_CONSTANTS: [isize;4]  = [13, 12, 11, 10];

fn swap<T : Copy>(values: &mut [T], i: usize, j: usize) {
	let temp = values[i];

	values[i] = values[j];
	values[j] = temp;
}

fn permutation<T>(values: &mut [T;9], constants: &[T;4], from: usize, to: usize, expected: T) -> usize where
	T : Add<Output = T>,
	T : Sub<Output = T>,
	T : Mul<Output = T>,
	T : Div<Output = T>,
	T : PartialEq,
	T : Copy {
	if from == to {
		if evaluate_and_check(values, constants, expected) {
			1
		} else {
			0
		}
	} else {
		let mut count: usize = 0;
		for j in from..to {
			swap(values, from, j);
			count += permutation(values, constants, from + 1, to, expected);
			swap(values, from, j);
		} 

		count
	}
}

fn permute_and_check_slice<T, P>(values: &mut [T;9], from: usize, to: usize, predicate: &P) -> usize where 
	T : Copy,
	P : Fn(&[T;9]) -> bool {

	if from == to {
		if predicate(values) { 1 } else { 0 }
	} else {
		let mut count: usize = 0;
		for j in from..to {

			swap(values, from, j);
			count += permute_and_check_slice(values, from + 1, to, predicate);
			swap(values, from, j);
		} 

		count
	}
}

fn permute_and_check<T, P>(values: &mut [T;9], predicate: &P) -> usize where 
	T : Copy,
	P : Fn(&[T;9]) -> bool {

	let count = values.len();
	permute_and_check_slice(values, 0, count, predicate)
}

fn evaluate_and_check<T>(values: &[T;9], constants: &[T;4], expected: T) -> bool where
	T : Add<Output = T>,
	T : Sub<Output = T>,
	T : Mul<Output = T>,
	T : Div<Output = T>,
	T : PartialEq,
	T : Copy {

	evaluate(values, constants) == expected
}

fn find_int_solutions() -> usize {
	let mut int_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];

	permute_and_check(&mut int_values, &|xs| { evaluate(xs, &INT_CONSTANTS) == 66 })
}

fn find_float_solutions() -> usize {
	let mut int_values = [1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64, 9f64];

	permute_and_check(&mut int_values, &|xs| { evaluate(xs, &FLOAT_CONSTANTS) == 66f64 })
}

fn without_closure() {
	let mut float_values = [1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64, 9f64];
	let mut int_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
	let mut result_count = permutation(&mut float_values, &FLOAT_CONSTANTS, 0, 9, 66f64);

	println!("Float result count: {:?}", result_count);
	result_count = permutation(&mut int_values, &INT_CONSTANTS, 0, 9, 66);
	println!("Int result count: {:?}", result_count);
}

fn with_closure() {
	let mut result_count = find_float_solutions();

	println!("Float result count: {:?}", result_count);
	result_count = find_int_solutions();
	println!("Int result count: {:?}", result_count);
}

fn main() {
	without_closure();
	with_closure();
}
