fn main() {
	let item = "salad";
	
	let price;
	if item == "salad" {
		price = 3.50;
	} else if item == "muffin" {
		price = 2.25;
	} else {
		price = 2.00;
	}
	
	println!("The price is: {}", price);
}