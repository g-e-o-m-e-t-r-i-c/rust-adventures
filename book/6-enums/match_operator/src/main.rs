#[allow(dead_code)]
#[derive(Debug)] // so we can inspect the state in a minute

enum UsState {
	Alabama,
	Alaska,
	// --snip--
}

// NOTE: define a custom type `Coin` which can take multiple types of coins as value
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState), // NOTE: a variant which takes another enum type
}

fn value_in_cents(coin: Coin) -> u8 {
	// NOTE: match the variant of the coin to its respective value
	match coin {
		// NOTE: can include functions
		Coin::Penny => {
			println!("Lucky penny!");
			1 // return value
		}
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		}
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	// NOTE: if there is no value to x, return nothing, if x contains something, return that something + 1
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn main() {
	let five = Some(5);
	let _six = plus_one(five);
	let _none = plus_one(None);

	// NOTE: using _ in match

	let dice_roll = 9;
	match dice_roll {
		3 => {
			println!("fancy hat put on");
		}
		7 => {
			println!("fancy hat taken off");
		}
		// NOTE: if the value is anything else
		_ => (),
	}
}
