// library crate which provides functionalities of a restaurant
// `cargo new --lib <lib_name>`

/*
	The fix_incorrect_order function is in the back_of_house module,
	so we can use super to go to the parent module of back_of_house,
	which in this case is crate, the root.
	From there, we look for serve_order and find it.

	We think the back_of_house module and the serve_order function are likely to stay in the
	same relationship to each other and get moved together should we decide to
	reorganize the crate’s module tree.

	Therefore, we used super so we’ll have fewer places to update code in the future
	if this code gets moved to a different module.
*/

#[allow(dead_code)]
fn serve_order() {}

#[allow(dead_code)]
mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order(); // similar to beginning a path with ../
	}

	fn cook_order() {}

	// huzzah a public breakfast struct
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast), // toast is public (we can use meal.toast)
				seasonal_fruit: String::from("peaches"), // seasonal_fruit isn't
			}
		}
	}

	// all variants are public
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

// make path shorter
// use self::front_of_house::hosting; // relative path

mod front_of_house; // if using external file for import
pub use crate::front_of_house::hosting; // absolute path -- import from external file

// use crate::front_of_house::hosting::add_to_waitlist; // make the whole thing easier to write
pub fn eat() {
	// absolute path
	// crate::front_of_house::hosting::add_to_waitlist(); // ERROR: hosting is a private function

	// relative path
	// front_of_house::hosting::add_to_waitlist();

	// using the path in a scope
	hosting::add_to_waitlist();

	// import a single function
	// add_to_waitlist();

	// order a breakfast in the summer with Rye toast
	let mut meal = back_of_house::Breakfast::summer("Rye");
	// change our mind about what bread we'd like
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);

	// public variants of enum
	let _order1 = back_of_house::Appetizer::Soup;
	let _order2 = back_of_house::Appetizer::Salad;
}
