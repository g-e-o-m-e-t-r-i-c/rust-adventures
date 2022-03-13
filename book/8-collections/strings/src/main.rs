fn main() {
	// NOTE: create a mutable string
	let mut _s = String::new();

	let data = "initial contents";
	let _t = data.to_string(); // NOTE: loads the string with initial data
	let _t = "initial contents".to_string();

	let _u = String::from("init"); // NOTE: equivalent to .to_string() above

	// NOTE: strings are utf-8, so we can put anything into them
	let _hello = String::from("السلام عليكم");
	let _hello = String::from("Dobrý den");
	let _hello = String::from("Hello");
	let _hello = String::from("שָׁלוֹם");
	let _hello = String::from("नमस्ते");
	let _hello = String::from("こんにちは");
	let _hello = String::from("안녕하세요");
	let _hello = String::from("你好");
	let _hello = String::from("Olá");
	let _hello = String::from("Здравствуйте");
	let _hello = String::from("Hola");

	// NOTE: appending to a string with .push_str
	let mut a = String::from("foo");
	a.push_str("bar"); // a is now "foobar"

	// NOTE: .push method only appends 1 char
	let mut b = String::from("lo");
	b.push('l'); // a single char is always surrounded by single quotes

	// NOTE: concat
	let x = String::from("Hello, ");
	let y = String::from("world!");
	let _z = x + &y; // WARN: s1 has been moved here and can no longer be used

	// NOTE: concat also works with literals!
	let tic = String::from("tic");
	let tac = String::from("tac");
	let toe = String::from("toe");
	// let _tic_tac_toe = tic + "-" + &tac + "-" + &toe; // gets unwieldy when too many things to concat

	// NOTE: format! macro
	let _tic_tac_format = format!("{}-{}-{}", tic, tac, toe);

	// NOTE: indexing & slicing
	// ERROR: indexing doesn't work
	// let hi = String::from("hello");
	// let h = &hi[0]; // std::String cannot be indexed by integer

	// FIX: use slicing
	let hi = String::from("hello");
	let h = &hi[..1]; // includes index 0, excludes index 1
	println!("the first character of '{}' is '{}'", hi, h);

	// NOTE: iteration
	for c in "नमस्ते".chars() {
		println!("{}", c); // prints the literal chars
	}

	for b in "नमस्ते".bytes() {
		println!("{}", b); // prints out the integer bytes making up the string
	}
}
