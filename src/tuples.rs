#![allow(dead_code, unused)]

#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

fn main() {
    syntax_update();
    struct_like_tuple();
    selective_deconstruct();
}

fn syntax_update() {
    let v2 = Vec2 { x: 2.0, y: 4.0 };
    let v3 = Vec2 { x: 14.0, ..v2 };
    println!("`Struct update syntax`, {:?}", v3);
}

fn struct_like_tuple() {
    let v = Vec2 { x: 1.0, y: 3.0 };
    let Vec2 { x, y } = v;
    println!("`Desconstruct struct`, x = {:?}, y = {:?}", x, y);
}

fn selective_deconstruct() {
    let v = Vec2 { x: 1.0, y: 3.0 };
    let Vec2 { x, .. } = v;
    println!("`Select value`, x = {:?}", x);
}
