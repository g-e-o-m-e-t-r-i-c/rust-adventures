fn main() {
	// NOTE: create a vector to store i32s
	let _v1: Vec<i32> = Vec::new();

	// NOTE: create a vector with initialised values
	let _v2 = vec![1, 2, 3];

	// NOTE: pushing values (to the back) in a vector
	let mut v3 = Vec::new();
	v3.push(5);
	v3.push(6);
	v3.push(7);
	v3.push(8);

	{
		let _v = vec![1, 2, 3, 4];
	} // <- v goes out of scope and is freed here

	// NOTE: reading elements of vectors
	let v = vec![1, 2, 3, 4, 5]; // create the vector

	let third: &i32 = &v[2]; // reference the third element (v[2])
	println!("The third element is {}", third);

	// alternatively, get the element using the .get(index) method
	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}

	// NOTE: having a reference while trying to push an element
	// let mut w = vec![1, 2, 3, 4, 5];
	// let first = &w[0]; // immutable borrow here - needs to be dropped first
	// w.push(6); // will fail here - mutable borrow here
	// println!("The first element is: {}", first);

	// NOTE: iteration over an IMMUTABLE vector
	let w = vec![100, 32, 57];
	for i in &w {
		println!("{}", i);
	}

	// NOTE: iteration over a MUTABLE vector
	let mut x = vec![100, 32, 57];
	for i in &mut x {
		*i += 50;
		println!("{}", i);
	}

	// NOTE: vectors can only store values of the same type, but we can change that using enums!
	// different variants of the same enum type
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	// vector thinks values have the same type, but internally they don't
	let _row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];
}
