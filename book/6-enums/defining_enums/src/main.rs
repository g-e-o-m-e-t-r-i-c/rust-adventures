#[allow(dead_code)]

//  NOTE: an enum can enumerate all possible variants of its value. enum values can only be one of its variants

enum IpAddrKind {
	V4,
	V6,
}

// NOTE: make structs with enums as fields
struct IpAddr {
	kind: IpAddrKind,
	address: String,
}

// 4. NOTE: more concise way of defining enums with values
enum IpAddress {
	V4(String),
	V6(String),
}

// 5. NOTE: enums with different typed variants
enum IpA {
	V4(u8, u8, u8, u8),
	V6(String),
}

enum _Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

// 6. NOTE: implement functions for enums
impl _Message {
	fn _call(&self) {
		// stuff
	}
}

fn main() {
	// 1. NOTE: create instances of each of the two variants of `IpAddrKind`
	let _four = IpAddrKind::V4;
	let _six = IpAddrKind::V6;

	// 2. NOTE: functions taking enum variants as params
	// route(IpAddrKind::V4);

	// 3. NOTE: structs with enums as fields
	let _home = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	};

	let _loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};

	// 4. NOTE: more concise enums
	let _home2 = IpAddress::V4(String::from("127.0.0.1"));
	let _loopback2 = IpAddress::V6(String::from("::1"));

	// 5. NOTE: enums with different typed variants
	let _hm = IpA::V4(127, 0, 0, 1);
	let _lb = IpA::V6(String::from("::1"));
}

// NOTE: create function with enum as param
fn _route(_ip_kind: IpAddrKind) {}
