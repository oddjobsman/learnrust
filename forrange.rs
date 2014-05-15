// forrange.rs
fn main() {
	// n will take the value of 1, 2, ..., 100 in each iteration
	for n in range(0, 101) {
		if n % 15 == 0 {
			println!("fizzbuzz");
		} else if n % 3 == 0 {
			println!("fizz");
		} else if n % 5 == 0 {
			println!("buzz");
		} else {
			println!("{}", n);
		}
	}
}