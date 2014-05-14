// nested.rs
fn main() {
	'outer: loop {
		println!("Entered outer loop");
		
		'inner: loop {
			println!("Entered inner loop");
			
			// this would break the inner loop
			//break;
			
			// this breaks the outer loop
			break 'outer;
		}
		
		println!("This point will never reach");
	}
	
	println!("Exited the outer loop");
}