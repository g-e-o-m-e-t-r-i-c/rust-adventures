fn main() {
	// 1. SCOPE
	let _s = "hello"; // in the scope of fn main(), s is valid
	{
		let _t = "goodbye"; // t will be valid from this point onward
	} // t is now dropped and will no longer be valid outside this scope

	// 2. THE `String` TYPE
	// The `String::from` function is only used when we don't know its size, for example in user input.
	let mut hello_string = String::from("Hello"); // `String::from` is only used when we don't know the string's size, e.g. in user input.
	hello_string.push_str(", world!"); // `String::push_str`: appends a string literal
	println!("{}", hello_string);

	// 3.1. MOVE
	let n1 = 5;
	let n2 = n1; // this code is valid!
	println!("{}, {}", n1, n2);

	// 3.2. MOVE (`String`)
	// let s1 = String::from("hello");
	// let s2 = s1; // value of s1 now moved to s2
	// println!("{}, {}!", s1, s2); // ERROR: s1 already has a new owner, s2, and cannot be borrowed.

	// 3.3. CLONE (`String`) - the way to borrow `String`s without error
	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("s1: {}, s2: {}", s1, s2);

	// 4. EXAMPLE (NOTE: includes functions below)
	let string = String::from("hi! i'm a string"); // `string` moves into scope
	takes_ownership(string); // `string`'s value moves into the function, and is no longer valid
	let x = 69; // `x` comes into scope
	makes_copy(x); // `x` goes out of scope, then `s`
}
fn takes_ownership(a_string: String) {
	// a_string comes into scope
	println!("{}", a_string);
} // Here, a_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(an_int: i32) {
	// an_int comes into scope
	println!("{}", an_int);
} // Here, an_int goes out of scope. Nothing special happens.
