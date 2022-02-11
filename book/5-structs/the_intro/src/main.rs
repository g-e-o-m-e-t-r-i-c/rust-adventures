#[allow(dead_code)]

// a struct is basically a struct in c, with fields of values with specific datatype
struct User {
	active: bool,
	username: String, // NOTE: we don't use &str, because we want each instance of the struct to own its own data!
	email: String,
	hp: String,
	login_count: u64,
	posts: u64,
	karma: u64,
}

// a struct can also be defined using a tuple:
struct Color(i32, i32, i32);

fn main() {
	// create a struct
	let mut user1 = User {
		active: true,
		username: String::from("foobar20"),
		email: String::from("foo@bar.com"),
		hp: String::from("1800-420-6969"),
		login_count: 7328,
		posts: 12357,
		karma: 124890,
	};

	// modify struct values (ONLY for mutable structs - specify with `mut`)
	user1.hp = String::from("1800-725-9999");

	// create a user via function
	let user2 = create_user("foo@com.bar".to_string());

	// create a struct based on other structs' info
	let _user3 = User {
		active: true,
		username: String::from("foobar21"),
		email: user1.email,
		hp: user2.hp,
		login_count: user1.login_count - 2000,
		posts: user1.posts / 22,
		karma: 0,
	};

	let _user4 = User {
		email: String::from("foobarFOOBAR"),
		..user1 // inherit the rest of the fields' values from user1
	};

	// create a struct from a tuple;
	let _gruvy_grey = Color(40, 40, 40);
}

// function to return a struct
fn create_user(e: String) -> User {
	User {
		email: e,
		username: String::from("new_user"),
		active: true,
		login_count: 1,
		hp: String::from("no_hp_found"),
		posts: 0,
		karma: 100,
	}
}
