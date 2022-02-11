fn main() {
	// get a range of a string [start, end] (inclusive)
	let s = String::from("hello world"); // `String`
	let t = "bye earth"; // NOTE: string literals (`str`) == slices

	let _ello_w = &s[1..6];
	let _hello = &s[..5]; // can drop the first index if it's 0
	let _world = &s[6..]; // can drop the last index if it's the end

	println!("the first word of {:?} is {:?}", s, first_word(&s)); // `String` types
	println!("the first word of {:?} is {:?}", t, first_word(&t)); // string literals
}

/* FIX: return the first word found in a string
NOTE: we can also take string slices (`str`) as params
*/
// fn first_word(s: &String) -> &str { // to accommodate string slices, use...
fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		// if the iterator finds a space, then a word is found, return the word
		if item == b' ' {
			return &s[..i];
		}
	}
	&s[..] // otherwise, return the entire string (there are no spaces, hence the string is a word)
}
