fn main() {
	// 1. REFERENCING
	let mut s1 = String::from("hello");
	let len = calculate_length(&s1);
	println!("The length of '{}' is {}.", s1, len);

	// 2. MODIFYING BORROWED VALUES
	change(&mut s1);
	println!("modified string: '{}'", s1);

	// 3. BORROWING MUTABLES
	// let r1 = &mut s1; // first borrow
	// let r2 = &mut s1; // ERROR: cannot borrow mut value more than once
	// println!("{}, {}", r1, r2); // first borrow here, second borrow also here

	{
		// FIXED
		let _r1 = &mut s1;
	} // r1 goes out of scope here, so we can make a new reference with no problems.
	let _r2 = &mut s1;

	// converse
	let mut _s = String::from("ah yes");
	// let t1 = &mut s; // ERROR (mutable borrow)
	let _t2 = &_s; // no problem: immutable borrow
	let _t3 = &_s; // no problem: immutable borrow

	// 4. DANGLING REFERENCES
	let _ref_to_nothing = dangle();
}

// function takes the reference to a variable of type `String` as a param, rather than the variable itself
fn calculate_length(s: &String) -> usize {
	// s is a reference to a String
	s.len()
} // s goes out of scope, but because it doesn't have ownership of what it refers to, nothing happens.

// ERROR: function below doesn't work, cannot modify a borrowed variable (&)
// fn change(string: &String) { // incorrect
fn change(string: &mut String) {
	// string type should be directly used, instead of borrowing
	string.push_str(", ok?");
}

// ERROR: dangling reference
// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// }

fn dangle() -> String {
	let s = String::from("hi");
	// &s // DANGLE! data cannot go out of scope before the reference does
	s // FIX: return variable directly
} // s goes out of scope HERE, dropped; memory goes away
