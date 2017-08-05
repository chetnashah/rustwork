
fn main() {
    println!("hi");
    // variable binding let expressions
    // lhs of let expression is a pattern
    let foo = 5; // let declares immutable variables like functional langs
    // foo = 6; //err : re-assignment of immutable variable

    let (_, two) = (1, 2);
    println!("two = {}", two);

    // syntax for declaration of local variables
    // let name[: type] [= value];
    // rust has type inference , types come after a colon, add wherever possible
    let p: i32 = 5;

    let kk;
    kk = 6;

    let mut jj = 2;
    jj = 11; // ok because jj is mut-able

    // array is fixed size immutablel list of same type
    let arr = [1, 2, 3];
    // array access notation
    println!(" second element is {}", arr[1]);

    // tuples are named records with number as their label
    let tuple = (95, 96, 99);


    // conditionals in rust
    // all branches of if expression should return value of same type
    // and the returned value is the value of the if expression
    if foo == 5 {
        println!("its five!");
    } else if foo == 6 {
        println!("its a six!");
    } else {
        println!("neither 5 nor 6");
    }

    // looping in rust
    // looping with while
    let mut i = 0;
    while i < 10 {
        println!("hi there looper!");
        i += 1; // no ++ or -- allowed in rust
    }
    //for loop only done with iterators
    for j in 0..10 {
        println!("aha");
    }

    // statements in rust end in semicolon, and expressions don't
    // pattern matching and expressions in rust
    // match significantly more powerful than switch

    // all blocks i.e. { //code }
    // a block is an expressions with return value as that of last line
    let blockvalue = {
        println!("I'm in a block"); // any number of statements can be here
        2000 // last line is the return value of block expression
    };

    println!("block value = {}", blockvalue);

    // below match returns unit, since println is a statement
    // all cases should return same type
    match kk == 6 {
        true => println!("KK is 6"), // use commas or put rhs in curly braces
        false => println!("KK not 6"),
    }

    // match everything with _
    let y = 20;
    match y {
        0 => {}
        4 => {
            println!("it is 4");
        }
        _ => {
            println!("match captured via _ ");
        }
    }

    // pattern match with guards
    match (20, 21) {
        (x, y) if x > y => {
            println!("decreasing!");
        }
        (x, y) if x < y => {
            println!("Increasing!");
        }
        _ => {
            println!("equal");
        }
    }

    // a successful pattern match will will
    // enrich enviroment with shadow binding if pattern has a variable already declared.

    // destructuring pattern match with struct constructors,
    // we have already destructured a tuple above
    // here's a struct (like a record)
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 100, y: 100 };
    match origin {
        Point { x, y } => {
            println!(" x is {} and y is {}", x, y);
        }
    }

    // destructuring only on fields you want
    match origin {
        Point { y, .. } => {
            println!(" y cord is {}", y);
        }
    }

    let pt = Point { x: 2, y: 3 };
    // using match as a expression instead of statement
    // but make sure all arms rhs have same return type
    let pointSum = match pt {
        Point { x, y } => x + y,
    };

    println!(" pointSum is {}", pointSum);

    // matching ranges with ...
    let h = 'u';
    match h {
        'a'...'j' => println!(" early charachter "),
        'k'...'z' => println!(" late charachter "),
        _ => println!("anything else!"),
    }

    // aliasing bound var name during pattern match
    match origin {
        Point { x: x1, y: y1 } => { println!(" x1, y1 = {},{}", x1, y1); }
    }

}
