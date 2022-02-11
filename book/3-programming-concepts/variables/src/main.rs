fn main() {
	// 1. MUTABILITY

	// let x: i32 = 5; // immutable
	// x = 6; // error - cannot change the value of immutable variable

	let mut x = 7; // mutable
	println!("x (original): {}", x);
	x = 69; // no problem - can change value of mutable
	println!("x (new): {}", x);

	// 2. CONSTANTS
	// 2a) cannot use 'mut' with consts
	// 2b) must define type (see u32 below - unsigned 32-bit int)
	const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;
	println!("The number of seconds in a day is: {}", SECONDS_IN_A_DAY);

	// 3. SHADOWING (MUTABLE VARIABLES)
	// changing the value of immut. var. by redefinition
	let y = 5;
	println!("y (original): {}", y);

	let y = y + 1; // now x = 6, redefined using let
	println!("y (redefined): {}", y);

	{
		let y = y * 2; // y = 12 in the inner scope, will not be affected in the outer scope
		println!("y (inner scope): {}", y);
	}

	println!("y (out of inner scope): {}", y);

	// changing the variable AND its type (only works for immutable variables)
	let word = "word";
	println!("word (original): {}", word);
	let word = word.len(); // change word's original value of "word" to its length (4)
	println!("word (redefinition, changed type): {}", word);

	// what happens when you try to change a MUTABLE variable's type (redefinition)?
	// let mut word = "word";
	// word = word.len();
}
