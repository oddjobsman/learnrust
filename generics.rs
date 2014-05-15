// generics.rs

// generic struct
struct Pair<T> {
	first: T,
	second: T
}

// reimplementation of a 2-element Tuple
struct Tuple2<T, U>(T, U);

fn main() {
	// explicitly specialized Pair
	let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'c' };

	// implicitly specialized Pair
	let pair_of_ints = Pair { first: 1, second: 2 };

	// explicitly specialized Tuple2
	let tuple: Tuple2<char, int> = Tuple2('R', 2);

	// explicitly specialized swap call
	let swapped_pair_of_chars: Pair<char> = swap::<char>(pair_of_chars);

	// implicitly specialized swap call
	let swapped_pair_of_ints = swap(pair_of_ints);
}

// a generic function
fn swap<T>(pair: Pair<T>) -> Pair<T> {
	let Pair { first: first, second: second } = pair;

	Pair { first: second, second: first }
}