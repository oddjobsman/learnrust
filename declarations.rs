// a static item
static MONSTER_FACTOR: f64 = 57.8;

fn main() {
	// initial local variable of type f64
	let monster_size = MONSTER_FACTOR * 10.0;
	
	println!("Monster size now is {}", monster_size);
	
	// initial local variable shadowed as int
	let monster_size: int = 50;
	
	println!("Monster size now is {}", monster_size);
}