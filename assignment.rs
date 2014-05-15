// assignment.rs
use std::owned::Box;

fn main() {
    // stack allocated integer
    let x = 5;

    // copy `x` to `y`, there are no resources to move
    let y = x;

    // both values can be used, because they are independant
    println!("x is {} and y is {}", x, y);

    // `a` is pointer to a heap allocated integer
    let a = box 5;

    // copy `a` into `b`, now both point to the same heap allocated object
    // *but*, now `b` *owns* the heap allocated object
    // `b` is now in charge of freeing memory in the heap
    let b = a;

    // move back `b` into `a`
    let a = b;

    // Error: `a` can no longer access the data
    // because the resource has been *moved*
    println!("{} can not be used", a);

    // a goes out of scope
    // b goes out of scope, and the memory is freed
}
