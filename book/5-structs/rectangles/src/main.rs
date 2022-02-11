#[derive(Debug)] // make sure Rects can implement display and debug traits

// use a struct to link the width and height of a rectangle
struct Rectangle {
	width: f64,
	height: f64,
}

// method specific to each Rectangle
impl Rectangle {
	fn area(&self) -> f64 {
		self.width * self.height
	}

	fn width(&self) -> bool {
		self.width > 0.0
	}

	// method with more params
	fn hold(&self, other: &Rectangle) -> bool {
		self.width >= other.width && self.height >= other.height
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30.4,
		height: 50.9,
	};
	let rect2 = Rectangle {
		width: 10.0,
		height: 40.0,
	};
	let rect3 = Rectangle {
		width: 60.0,
		height: 45.0,
	};

	// println!("rect1 is {:#?}", rect1);
	println!("rect1 is {:#?}", rect1); // prettier output

	// println!("the area of rect1 is {}.", area(&rect1)) // global function
	println!("the area of rect1 is {}.", rect1.area()); // method

	// use a method (return a bool)
	if rect1.width() {
		println!("the rectangle's width is {}.", rect1.width);
	}

	// methods with more params (in action)
	println!("Can rect1 hold rect2? {}", rect1.hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.hold(&rect3));
}

// global function for all Rectangles
// fn area(rect: &Rectangle) -> f64 {
// 	rect.width * rect.height
// }
