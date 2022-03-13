use std::collections::HashMap;

fn main() {
	// NOTE: create hashmap
	let mut scores = HashMap::new();

	// NOTE: insert key-value pairs into hashmap
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Red"), 50);
	// scores.insert(String::from("Blue"), 50); // values with the same key will be overwritten

	// NOTE: alternatively turn two vectors into keys and values
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let init_scores = vec![10, 50];
	let mut _spreadsheet: HashMap<_, _> =
		teams.into_iter().zip(init_scores.into_iter()).collect();

	// NOTE: accessing values in hashmap
	let team_name = String::from("Blue");
	let _score = scores.get(&team_name); // notice that team_name is a ref

	// NOTE: iteration
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	// NOTE: .or_insert() inserts the key-value pair only if the key stated has no value
	let mut exam = HashMap::new();
	exam.insert(String::from("alestor"), 30);
	exam.entry(String::from("joshua")).or_insert(40); // "joshua" doesn't have a value, insert
	exam.entry(String::from("alestor")).or_insert(45); // "alestor" has a value, don't insert
	println!("exam scores: {:?}", exam);

	// NOTE: update values based on old values in hashmap
	let text = "hello world wonderful world";
	let mut mp = HashMap::new();
	for word in text.split_whitespace() {
		let count = mp.entry(word).or_insert(0); // if the word is already counted, don't insert it
		*count += 1; // update the count
	}
	println!("{:?}", mp);
}
