fn main() {
	let a = 1;			// inferred as signed int
	let b = 10i;		// explicitly set as signed int
	let c = 100u;		// explicitly set as unsigned int
	let d = 1000i32; 	// explictly set as signed 32-bit int
	
	println!("a is {}", a);
	println!("b is {}", b);
	println!("c is {}", c);
	println!("d is {}", d);
}