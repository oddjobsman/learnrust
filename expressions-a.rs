fn main() {
	let item = "muffin";

	let price = 
		if item == "salad" {
			3.50
		} else if item == "muffin" {
			2.50
		} else {
			2.00
		};

	println!("The price is {}", price);
}