#![allow(dead_code)]
use std::mem;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
struct PointDu3D {
    x: f64,
    y: f64,
    z: f64,
}
enum Enumi2T {
    X, Y
}
enum Enumi3T {
    X, Y, Z
}
enum Enumi2T1M {
    X, Y, Z(f64, f64),
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() ->Point {
    Point { x: 0.0, y: 0.0 }
}
fn boxed_origin() ->Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    let f32_test: f32 = 0.12;
    let f64_test: f64 = 0.24;
    let i32_test: i32 = 2;
    let u32_test: u32 = 4;
    let pointdu = PointDu3D { x: 0.0, y: 0.0, z: 0.0 };

    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });
    let boxed_point: Box<Point> = Box::new(origin());
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());


    println!("(f32, f64, i32, u32) occupies ({}, {}, {}, {}) bytes",
        mem::size_of_val(&f32_test), mem::size_of_val(&f64_test),
        mem::size_of_val(&i32_test), mem::size_of_val(&u32_test));
    println!("Point occupies {} bytes on the stack",
        mem::size_of_val(&point));
    println!("3DPoint occupies {} bytes on the stack",
        mem::size_of_val(&pointdu));
    println!("Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle));
    println!("Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point));

}