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

```
