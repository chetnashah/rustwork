### What happens on assignment?
One of two things happen:
1. if value to the right of assingment implements copy trait then copy
2. else value ownership is moved to the variable on the left side of assignment.

```rust
    let x = 1;
    let y = x;// copy because i32 implements Copy trait, x is a valid value after this assingment
    println!("{} {}", x, y);
    
    let j = String::from("abc");// String does not implement Copy trait
    let k = j;// ownership moved to k, j is no longer valid
```

3 rules:

1. Every value has an owner.
2. only one owner of a value.
3. Value gets dropped if its owner goes out of scope.

Rust uses the term copy only if stack data is copied.

If heap data is also copied, the term clone is used (like deep copy)

Moving of values:
happens on 
1. reassignment
2. function calls

### copy trait

as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

All the integer types, such as `u32`.
The Boolean type, `bool`, with values true and false.
All the floating point types, such as `f64`.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

### References and borrowing
Most of the time, we'd like to access data without taking ownership over it. To accomplish this, Rust uses a borrowing mechanism. Instead of passing objects by value (T), objects can be passed by reference (&T).
Basically pass by reference is borrowing.
To prevent moving of ownership to function on the function call, we use references (Also known as borrowing).
By borrowing, the values are valid after borrowing is over/finished (e.g. function call in eg. below)

**Note** - If a value is not being passed by reference to a function there is a move of ownership going on.
e.g.
```rust
let s1 = String::from("abc");
do_stuff(&s1);
println!("{}", s1); // fine, ok
// do_stuff borrows reference to the value
// reference gets moved to function
// reference is dropped at end of function
// so borrowing ends at the end of function execution
fn do_stuff(s: &String) {
    // do stuf
}
```

**References rule** - You can have either one mutable reference to a value, or any number of immutable refences to the value (think thread safety).

References default to immutable even if the value being referenced is mutable.
But if we use a mutable reference to a mutable value, then we can use the reference to change the value as well.
Mutable data can be mutably borrowed using `&mut T`. This is called a mutable reference and gives read/write access to the borrower. In contrast, `&T` borrows the data via an immutable reference, and the borrower can read the data but not modify it
Notice the syntax `& mut`
e.g.
```rust
let mut s1 = String::from("abc");
do_stuff(&mut s1);

fn do_stuff(s: &mut String){
    *s = String::from("xyz");
}
```