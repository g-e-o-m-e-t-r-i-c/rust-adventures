use std::fmt;

/* TASKS
1. NOTE: Add a function `rect_area` which calculates the area of a `Rectangle`
2. NOTE: Add a function `square` which takes a `Point` and a `f32` as arguments,
and returns a `Rectangle` with its top left corner on the point, and a width and height corresponding to the `f32`.
*/

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
	name: String,
	age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
	x: f32,
	y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
	// A rectangle can be specified by where the top left and bottom right
	// corners are in space.
	top_left: Point,
	bottom_right: Point,
}

// FIXED: 1
impl Rectangle {
	fn area(&self) -> f32 {
		let width = &self.top_left.x - &self.bottom_right.x;
		let height = &self.top_left.y - &self.bottom_right.y;
		width * height
	}
}

impl fmt::Display for Rectangle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}x{}",
			&self.bottom_right.x - &self.top_left.x,
			&self.top_left.y - &self.bottom_right.y
		)
	}
}

// FIXED: 2
fn square(pt: Point, wh: f32) -> Rectangle {
	let br = Point {
		x: pt.x + wh,
		y: pt.y + wh,
	};
	Rectangle {
		top_left: pt,
		bottom_right: br,
	}
}

fn main() {
	// Create struct with field init shorthand
	let name = String::from("Peter");
	let age = 27;
	let peter = Person { name, age };

	// Print debug struct
	println!("{:?}", peter);

	// Instantiate a `Point`
	let point: Point = Point { x: 10.3, y: 0.4 };

	// Access the fields of the point
	println!("point coordinates: ({}, {})", point.x, point.y);

	// Make a new point by using struct update syntax to use the fields of our
	// other one
	let bottom_right = Point { x: 5.2, ..point };

	// `bottom_right.y` will be the same as `point.y` because we used that field
	// from `point`
	println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

	// Destructure the point using a `let` binding
	let Point {
		x: left_edge,
		y: top_edge,
	} = point;

	let _rectangle = Rectangle {
		// struct instantiation is an expression too
		top_left: Point {
			x: left_edge,
			y: top_edge,
		},
		bottom_right,
	};

	let rect1 = Rectangle {
		top_left: Point { x: 3.0, y: 5.0 },
		bottom_right: Point { x: 6.0, y: 10.0 },
	};

	// Instantiate a unit struct
	let _unit = Unit;

	// Instantiate a tuple struct
	let pair = Pair(1, 0.1);

	// Access the fields of a tuple struct
	println!("pair contains {:?} and {:?}", pair.0, pair.1);

	// Destructure a tuple struct
	let Pair(integer, decimal) = pair;

	println!("pair contains {:?} and {:?}", integer, decimal);

	// area of a rectangle
	println!("area of rect: {}", rect1.area());

	// determine a square
	println!("square: {}", square(Point { x: 8.0, y: 9.0 }, 5.0));
}
