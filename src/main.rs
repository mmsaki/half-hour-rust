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
}
