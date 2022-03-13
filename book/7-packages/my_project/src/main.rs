// import std struct
use std::collections::HashMap;
// use std::collections::*; // bring all public items defined in std::collections into scope
// use std::fmt::Result as FmtResult;
// use std::io::Result as IoResult; // use aliases for the things we're importing
// use std::io::{self, Write}; // equivalent to importing std::io and std::io::Write
// use std::{cmp::Ordering, io}; // combine multiple imports into 1 line

// fmt and io both have Result, rust wouldn't know which one we meant if we imported eg. std::fmt::Result
// fn function1() -> FmtResult {
// 	// --snip--
// }

// fn function2() -> IoResult<()> {
// 	// --snip--
// }

fn main() {
	let mut map = HashMap::new();
	map.insert(1, 2);
}
