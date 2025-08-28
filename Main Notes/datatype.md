## Type mismatch and println! formatting issue
```rust
fn main () {
    let x : i32 = 5;
    let mut y: u32 = 5;

    y = x;
    println!(y)
}
````

This code won't compile.

### Error message

```
PS C:\Users\Dell\Downloads\hello> cargo run --example datatypes
   Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
error: format argument must be a string literal
 --> examples\datatypes.rs:6:14
  |
6 |     println!(y)
  |              ^
  |
help: you might be missing a string literal to format with
  |
6 |     println!("{}", y)
  |              +++++

error[E0308]: mismatched types
 --> examples\datatypes.rs:5:9
  |
3 |     let mut y: u32 = 5;
  |                --- expected due to this type
4 |
5 |     y = x;
  |         ^ expected `u32`, found `i32`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `hello` (example "datatypes") due to 2 previous errors
```

---

## Type casting and literal suffixes

```rust
fn main () {
    let v: u16 = 38_u8 as u16;
    // we can assign a specific type to a number using suffixes like 38_u8
    // we can also use `as` to convert one type into another

    println!("success")
}
```

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.53s
Running `target\debug\examples\datatypes.exe`
success
```

---

## Getting the type of a variable

```rust
fn main () {
    let x : u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("success");
}

// don't worry, we'll discuss this later
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
```

```
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
Running `target\debug\examples\datatypes.exe`
success
```

---

## Maximum values of signed and unsigned integers

```rust
fn main () {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    // this tells us the maximum value of signed and unsigned integers for their respective bit sizes

    println!("success");
}
```

```
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
Running `target\debug\examples\datatypes.exe`
success
```

---

## Using different number systems

```rust
// in Rust you can perform mathematical operations with different number systems

fn main () {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; 
    // decimal, hexadecimal, octal, and binary respectively => 1024 + 255 + 63 + 255
    assert!(v == 1597);
    println!("Success:3")
}
```

```
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
Running `target\debug\examples\datatypes.exe`
Success:3
```

---

## Floating-point precision issues

```rust
// in real life 0.3 = 0.2 + 0.1, but in binary representation, this may fail
// due to floating-point precision limits (64-bit IEEE representation)

// fn main () {
//     assert!(0.1 + 0.2 == 0.3);
//     println!("success :3")
// }
```

```
Running `target\debug\examples\datatypes.exe`
thread 'main' panicked at examples\datatypes.rs:103:5:
assertion failed: 0.1 + 0.2 == 0.3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\examples\datatypes.exe` (exit code: 101)
```

### Corrected with explicit type casting

```rust
fn main () {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("success :3")
}
```

```
PS C:\Users\Dell\Downloads\hello> cargo run --example datatypes
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
Running `target\debug\examples\datatypes.exe`
success :3
```

---

## Printing ASCII values of characters

```rust
// there should be no doubt that characters are numbers under the hood
// here’s how to print their ASCII codes

fn main () {
    for i in 'a'..='e' {
        println!("{}", i as u16)
    }
}
```

```
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
Running `target\debug\examples\datatypes.exe`
97
98
99
100
101
```

---

## Difference between char and string

```rust
// Rust makes a distinction between char and string
// string: uses double quotes ""
// char: uses single quotes ''

fn main () {
    let c1: char = "अ"; // this is incorrect, since it's using a string instead of a char
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
```

### Error message

```
155 |     let c1: char = "अ"; 
    |             ----   ^^^ expected `char`, found `&str`
    |             expected due to this
help: if you meant to write a `char` literal, use single quotes   
155 -     let c1: char = "अ";
155 +     let c1: char = 'अ';
For more information about this error, try `rustc --explain E0308`.
error: could not compile `hello` (example "datatypes") due to 1 previous error
```

---

## Corrected char usage

```rust
fn main () {
    let c1: char = 'अ'; // now works correctly
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
```

```
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
Running `target\debug\examples\datatypes.exe`
अ
```
### String vs &str

String is hard coded in binary meaning the space for it is same always

&str is string literal and we can mutate it to store anylong string and is expandable

kinda like for u8 datatype
we can change its value from one number to another between 0 to 255 cuz space exist to hold them cuz 

will discuss more later                         

### `Box::new(5);`

stores directly on heap and x is pointing towards the heap

```
fn main () {
    let x : Box<i32> = Box::new(5);

    let mut y : Box<i32> = Box::new(1);

    *y = 4;
    assert_eq!(*x, 5);
    println!("successss :3");
}
```

there is alot to soak in in this snippet, lets break down and go topic by topic
understanding each line in detail
1. `let x : Box<i32> = Box::new(5);`
this tells that, a variable `x` is pointing towards data (in datatype i32) storing 5 in the **heap memory**

2. `let mut y : Box<i32> = Box::new(1);`
this tells that, a mutable pointer (important cuz we gonna mutate it too if we dont put mutable it will return an error) pointing towards a 1i32 in heap memoryy

3. `*y = 4;`
this says that y is now pointing towards a 4 in heap memory, this star is important cuz y = 4 and *y = 4 mean diffrent
it says
data where y is pointing is now 4
this will later be used as well

4. ` assert_eq!(*x, 5);`
we dont compare x as is, we have to Specify, that x is pointing to what value
u may interpret it as
compare the data to which x is pointing and 5

## string output

```


fn main () {
    let t: (String, String) = (String::from("hello"), String::from("world"));
    println!("{}",t);
}
// this will show an error
// because t is not string type, but a 2 string tuple

// PS C:\Users\Dell\Downloads\hello> cargo run --example datatypes
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0277]: `(String, String)` doesn't implement `std::fmt::Display`
//    --> examples\datatypes.rs:204:19
//     |
// 204 |     println!("{}",t);
//     |               --  ^ `(String, String)` cannot be formatted with the default formatter
//     |               |
//     |               required by this formatting parameter
//     |
//     = help: the trait `std::fmt::Display` is not implemented for `(String, String)`
//     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

// For more information about this error, try `rustc --explain E0277`. 
// error: could not compile `hello` (example "datatypes") due to 1 previous error


```
```

fn main () {
    let t: (String, String) = (String::from("hello"), String::from("world"));
    println!("{:?}",t);
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
//      Running `target\debug\examples\datatypes.exe`
// ("hello", "world")
```
