// declare.rs

fn main() {
	// variable declaration
	let declared_variable;
	
	// initialize variable
	declared_variable = 3;
	
	// Error: usage of uninitialized variable is prohibited
	let uninitialized_variable: int;
	println!("{}", uninitialized_variable);
}