
fn main(){
    println!("hi");
    let foo = 5;// let declares immutable variables like functional langs
    // foo = 6; //err : re-assignment of immutable variable 

    // syntax for declaration of local variables
    // let name[: type] [= value];

    let kk;
    kk = 6;

    let mut jj = 2;
    jj = 11;// ok because jj is mut-able

    // statements in rust end in semicolon, and expressions don't

    // conditionals in rust
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
        i += 1;// no ++ or -- allowed in rust
    }
      //for loop only done with iterators
    for j in 0..10 {
        println!("aha");
    }

    // pattern matching and expressions in rust
}
