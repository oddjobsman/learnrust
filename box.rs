// box.rs
use std::owned::Box;
use std::mem::size_of_val;

struct Point {
    x: f64,
    y: f64
}

struct Rectangle {
    p1: Point,
    p2: Point
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // heap allocate this Point, return a pointer to it
    box Point { x: 0.0, y: 0.0 }
}

fn main() {
    // all type annotations are superfluous
    // stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    // heap allocated Rectangle
    let boxed_rectangle: Box<Rectangle> = box Rectangle {
        p1: origin(),
        p2: origin()
    };

    // function output can be boxed
    let boxed_point: Box<Point> = box origin();

    // double indirection
    let _box_in_a_box:  Box<Box<Point>> = box boxed_origin();

    println!("Point occupies {} bytes in the stack", size_of_val(&point));
    // the '&' means pass-by-reference, known as borrowing
    println!("Rectangle occupies {} bytes in the stack", size_of_val(&rectangle));

    // box size = pointer size
    println!("Boxed point occupies {} bytes in the stack", size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack", size_of_val(&boxed_rectangle));

    // copy data of the boxed_point onto the stack
    let unboxed_point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack", size_of_val(&unboxed_point));

    // unboxing via deconstruction
    let box another_unboxed_point = boxed_point;
    println!("Another unboxed point occupies {} bytes in the stack", 
        size_of_val(&another_unboxed_point));
}
