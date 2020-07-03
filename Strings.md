String literals are immutable.

`String` type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. 

You can create a String from a string literal using the from function, like so:
```rust
let s = String::form("hello");
```

Mutable strings:
```rust
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
```

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

1. The memory must be requested from the operating system at runtime -> `String::from`
2. We need a way of returning this memory to the operating system when we’re done with our String.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

### stack-only data: copy values

```rust
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y.

### Move reference (for heap allocs)


```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;// s1 is no longer valid

    // throws error! s1 is invalid
    println!("{}, world!", s1);// s1 use error: value borrowed here after move (was moved to s2)

}
```

because Rust also invalidates the first variable, instead of being called a shallow copy (of pointer), it’s known as a move.

Trying to use value after move results in error: `value borrowed here after move`.

### Copy trait
If a type has the `Copy` trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the `Drop` trait.

any group of simple scalar values can be Copy, and nothing that requires allocation or is some form of resource is Copy. Here are some of the types that are Copy:

1. All the integer types, such as `u32`.
2. The Boolean type, `bool`, with values true and false.
3. All the floating point types, such as `f64`.
4. The character type, `char`.
5. Tuples, if they only contain types that are also Copy. For example, `(i32, i32)` is `Copy`, but `(i32, String)` is not.

### Assignment and functions

The semantics for passing a value to a function are similar to those for assigning a value to a variable. 

Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return values, ownership and scope

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```