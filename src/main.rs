// Following tutorial at https://fasterthanli.me/articles/a-half-hour-to-learn-rust

fn main() {
    // `let` intoduces a varibale binding
    let _x; // decare 'x'
    _x = 42; // assign 42 to 'x'
    let x = 42; //can also be written in one line
    println!("The value of x is '{}'", x);

    // You can specify variable types with the `:` symbol
    let x: i32;
    x = 42;
    println!("The value of x is '{}'", x);

    // the `_` is a special name, it basically means to throw away something
    //  this does *nothing* because 42 is a contant
    fn get_thing() -> i32 {
        let x = 42;
        x
    }
    // this calls  `get_thing` but throws away its result
    let _ = get_thing();

    // names with _x are regular names, it's just that the compiler wont yell at us, or warn about variable not being used
    // separate binding with same name can be introduced, you can shadow a variable binding
    let x = 13;
    let x = x + 3;
    // using 'x' ater that line only refers to the second 'x',
    // the first 'x' no longer exists

    println!("The value of x aftershadow binding is : '{}'", x);

    // rust tuple, which you can think of as fixed-length collections of values of different types
    let pair = ('a', 17);
    pair.0; // this is 'a'
    pair.1; // this is 17
    println!(
        "Tuple has value: '{}' at position 0, and : '{}' at position 1",
        pair.0, pair.1
    );

    // if we want to annotate the type of 'pair' tuple we can write:
    let _pair: (char, i32) = ('a', 17);

    // tupel can be deconstructed when doing an assignment, useful when a function returns a tuple
    let (_some_char, _some_int) = ('a', 17);

    // of cours, when deconstruction a tuple, '_' can be used to throw away part of it
    let slice = [1, 31, 32, 4, 0];
    let middle = 2;
    let (left, right) = slice.split_at(middle);
    println!(
        "Using split_at on array [1,31,32,4,0] at index '{}' returns result of '{:?}' and '{:?}'",
        middle, left, right
    );

    // semi colon marks the end of a statement
    let x = 3;
    let y = 5;
    let z = y + x;
    println!("Value of z is '{}'", z);

    // statements can span multiple lines before `;` semicolon
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("{}", x);

    // heres a function that returns a 32-bit signed integer, arrow indicates return type
    fn fair_dice_roll() -> i32 {
        4
    }
    println!("{}", fair_dice_roll());

    // A pair of brackets declaires a block, which has it's own scope
    fn scope() {
        let x = "out";
        {
            // this is a different 'x'
            let x = "in";
            println!("{}", x);
        }
        println!("{}", x);
    }
    println!("{:?}", scope());

    // blocks are also expressions, which mean they evaluare to.. a value
    // this:
    let _x = 42;
    // is equivalent to this:
    let _x = { 42 };

    // inside a block, there can be multiple statements:
    let x = {
        let y = 1;
        let z = 2;
        y + z // this is a *tail* - what the whole block will evaluate to
    };
    println!("Inside block computation {}", x);

    // if statements are also expressions:
    fn roll_dice_roll(feeling_lucky: bool) -> i32 {
        if feeling_lucky {
            6
        } else {
            4
        }
    }
    println!("Are you feeling lucky? {}", roll_dice_roll(true));

    // A `match` is also an expression:
    fn fair_roll_dice(feeling_lucky: bool) -> i32 {
        match feeling_lucky {
            true => 6,
            false => 4,
        }
    }
    println!("I am feeling super lucky today {}", fair_roll_dice(false));

    // dots are used to access fields of a value:
    let a = (10, 20);
    a.0; // this is 10

    #[derive(Debug)]
    struct User {
        nickname: String,
    }

    fn get_some_struct() -> User {
        let user;
        user = User {
            nickname: "fasterthanlime".to_string(),
        };

        user
    }
    let amos = get_some_struct();
    println!("The nickname from the user is {:#?}", amos.nickname); // this is 'fasterthanlime'

    let nick = "fasterthanlime";
    let l = nick.len(); // this is 14
    println!("the length of nickname is {}", l);

    // th double colon ::, is similar but u ut operates on namespaces
    // in this exampl, std is a crate (~ library), cmp is a module (~ a source file), and min is a function
    let least = std::cmp::min(3, 8); // this is 3
    println!("Min value in (3, 8) is {}", least);

    // use derivertives can be used to 'bring in scope' name from other namespace:
    use std::cmp::min;
    let _least = min(7, 1); // this is 1

    // within use directives, curly brackets have another meaning: they're 'globs'. If we want both min and max,
    // we can do any of these:
    // this works:
    // use std::cmp::max;
    // use std::cmp::min;

    // this also works:
    // use std::cmp::{max, min};

    // this also works!
    // use std::{cmp::max, cmp::min};

    // A wild cad (*) let you import every symbol from a namespace:
    // this brings 'min' and 'max' in scope, and many other things
    use std::cmp::*;
    let _max = max(7, 1); // this is 7

    // types are namespaces too, and methos can be called as regular functions:
    let _x = "amos".len(); // this returns 4
    let _x = str::len("amos"); // this returns 4

    // 'str' is a primitive type, but many non-primitive types are also in scope by default
    // 'Vec' is a regular struct, not a primitive type
    let _v: Vec<i32> = Vec::new();

    // this is exactly the same code, but with the *full* path to 'Vec'
    let _v: Vec<i32> = std::vec::Vec::new();

    // this works be cause rust inserts this at the begning of every module:
    use std::prelude::v1::*;
    // which in turn re-exports a lot of symbols, like 'Vec', 'String', 'Option', and 'Result'

    // Structs are declared with the struct keyword
    #[derive(Debug)]
    struct Vec2 {
        x: f64, // 64-bit floating point, aka 'double precision'
        y: f64,
    }
    // structs can be initialized using struct literals:
    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { x: 2.0, y: 4.0 };
    // the order does not matter, only the names do

    // this is a shortcut for initializing the rest of the fields from another struct
    let v3 = Vec2 { x: 14.0, ..v2 };
    println!("Check out v3 sruct {:?}", v3);

    // this is called "struct update syntax", can only happen in last position, and cannot be followed by a comma
    // not that the rest of the fields can mean all the fields
    let v4 = Vec2 { ..v1 };
    println!("Check out v4 sruct {:?}", v4);

    // structs are like tuples, can be deconstructed,
    // just like this is a valid 'let' patterns:
    let (_left, _right) = slice.split_at(middle);
    // so is this:
    let Vec2 { x, .. } = v4;
    println!("Deconstructe value x = {} form struct {:?}", x, v4);
}
