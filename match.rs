// match.rs
fn main() {
	let number = 23;

	match number {
		1 => println!("One!"),
		// match several values
		2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
		// match a range
		13..19 => println!("A teen"),
		// bind rest of the values to x
		// and use a 'guard' to pick odd numbers
		x if x % 2 != 0 => println!("An odd one"),
		// the rest of the cases
		x => println!("{} ain't special", x),
	}

	let pair = (2, 3);

	// match can be used to destructure a tuple
	match pair {
		(x, y) if x == y => println!("These are twins"),
		(x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
		// _ means dont bind the value to a variable
		(x, _) if x % 2 != 0 => println!("The first is odd"),
		// _ can be used to match the rest of the cases
		_ => println!("No co-relation"),
	}

	// match is also an expression
	let _big_number = match number {
		0 => 9000,
		// block are also valid branches
		x if x < 10 => {
			let y = x * x;
			let z = x * x * x;
			x + y + z
		},
		x => x
	};
}