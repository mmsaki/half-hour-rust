#![allow(dead_code, unused)]

#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

fn main() {
    let _v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { x: 2.0, y: 4.0 };

    let v3 = Vec2 { x: 14.0, ..v2 };
    println!("'Struct update syntax': {:?}", v3);
}
