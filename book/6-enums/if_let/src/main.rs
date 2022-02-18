fn main() {
	// NOTE: original way
	let config_max = Some(3u8);
	match config_max {
		Some(max) => println!("The maximum is configured to be {}", max),
		_ => (),
	}

	// FIXED: if only want to execute code if the value is a `Some` variant
	if let Some(max) = config_max {
		// FIXED: shorter way
		println!("The maximum is configured to be {}", max);
	}
}
