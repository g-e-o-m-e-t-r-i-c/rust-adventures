use rand::Rng;

fn main() {
	let config_max = Some(3u8);

	// NOTE: original way
	match config_max {
		Some(max) => println!("The maximum is configured to be {}", max),
		_ => (),
	}

	// FIXED: if only want to execute code if the value is a `Some` variant
	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	}

	// more than 1 match case (using the coin example)
	// let mut count = 0;
	// match coin {
	// 	Coin::Quarter(state) => println!("State quarter from {:?}!", state),
	// 	_ => count += 1,
	// }

	// let mut count = 0;
	// if let Coin::Quarter(state) = coin {
	// 	println!("State quarter from {:?}!", state);
	// } else {
	// 	count += 1;
	// }
}
