fn main() {
	let hi = "hi";				// immutable local variable
	let mut count = 0;			// mutable local variable

	while count < 10 {
		println!("count is {}", count);
		count += 1;
	}
}