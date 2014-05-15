// structs.rs

// unit struct
struct Nil;

// tuple struct
struct Pair(int, f64);

struct Point {
	x: f64,
	y: f64
}

// reusing structs as fields
struct Rectangle {
	p1: Point,
	p2: Point
}

fn main() {
	// instantiate a Point
	let point = Point { x: 3.0, y: 4.0 };

	// access fields
	println!("Point co-ordinates: ({}, {})", point.x, point.y);

	// destructuring using let
	let Point { x: my_x, y: my_y } = point;

	let _rectangle = Rectangle {
		// structs are expressions too
		p1: Point { x: my_x, y: my_y },
		p2: point
	};

	// instantiate a unit struct
	let _nil: Nil;

	// instantiate a tuple struct
	let pair = Pair(1, 0.0);

	// destructure a tuple struct
	let Pair(integer, decimal) = pair;

	println!("integer is {} and decimal is {}", integer, decimal);

}