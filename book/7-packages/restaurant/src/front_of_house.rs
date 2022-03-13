#[allow(dead_code)]
pub mod hosting {
	// FIX: add `pub` keyword to make `hosting`, as well as `add_to_waitlist` public
	pub fn add_to_waitlist() {}

	fn seat_at_table() {}
}

mod serving {
	fn take_order() {}

	fn serve_order() {}

	fn take_payment() {}
}
