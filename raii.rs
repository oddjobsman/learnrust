// raii.rs

fn create_box() {
	let _function_box = box 3;

	// _function_box gets destroyed here, memory gets freed
}

fn main () {
	// allocate integer in the heap
	let _boxed_int = box 5;

	if true {
		// new (smaller) block scope

		// another heap allocated integer
		let _short_lived_box = box 4;

		// _short_lived_box gets destroyed here, memory gets freed
	}

	// create lots of boxes
	for _ in range(0, 1_000) {
		create_box();
	}

	// _boxed_int gets destroyed here, memory gets freed
}
