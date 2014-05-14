// newvariables.rs

fn main() {
	// an immutable variable
	let a_variable = 1;	
	
	// a mutable variable
	let mut mutable_variable = 2;
	
	// Error: attempted to modify an immutable variable
	//a_variable += 1;
	
	// This is ok
	mutable_variable += 1;
	
	println!("a_variable is {}", a_variable);
	println!("mutable_variable is {}", mutable_variable);
}