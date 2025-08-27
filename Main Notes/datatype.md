```
fn main () {
    let x : i32 = 5;
    let mut y: u32 = 5;

    y = x;
    println!(y)
}

// // this wont compile and should be obvious why
// PS C:\Users\Dell\Downloads\hello> cargo run --example datatypes
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error: format argument must be a string literal
//  --> examples\datatypes.rs:6:14
//   |
// 6 |     println!(y)
//   |              ^
//   |
// help: you might be missing a string literal to format with
//   |
// 6 |     println!("{}", y)
//   |              +++++

// error[E0308]: mismatched types
//  --> examples\datatypes.rs:5:9
//   |
// 3 |     let mut y: u32 = 5;
//   |                --- expected due to this type
// 4 |
// 5 |     y = x;
//   |         ^ expected `u32`, found `i32`

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `hello` (example "datatypes") due to 2 previous errors

```

```


fn main () {
    let v: u16 = 38_u8 as u16;
    // now there are 2 things in this part of code
    // first we can asign type for a number like this so 38 in 8 bit is 38_u8
    // and second we can use `as` to have it converted into the type we wanr


    println!("success")
}

//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.53s
//      Running `target\debug\examples\datatypes.exe`
// success
```

```

fn main () {
    let x : u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("success");
}

// dont be scared we will discuss later
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

//  Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
//      Running `target\debug\examples\datatypes.exe`
// success
```

```

fn main () {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    // this tells us the maximum value of signed and unsigned interger type for that bit

    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
//      Running `target\debug\examples\datatypes.exe`
// success
```

```

// in rust u can perform mathematical operations with diffrent number systems

fn main () {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //these are numbers in decimal, hexadecimal, octal and binary systems basically 1024 + 255 + 63 + 255
    assert!(v == 1597);
    println!("Success:3")
}
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target\debug\examples\datatypes.exe`
// Success:3
```

```

// irl 0.3 = 0.2 + 0.1 is true, buttt
// in binary, it depends, cuz in a 64 bit, that is datatype for float, the precision would return false

// fn main () {
//     assert!(0.1 + 0.2 == 0.3);
//     println!("success :3")
// }

// Running `target\debug\examples\datatypes.exe`

// thread 'main' panicked at examples\datatypes.rs:103:5:
// assertion failed: 0.1 + 0.2 == 0.3
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// error: process didn't exit successfully: `target\debug\examples\datatypes.exe` (exit code: 101)
```
```
fn main () {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("success :3")
}

// error: could not compile `hello` (example "datatypes") due to 3 previous errors
// PS C:\Users\Dell\Downloads\hello> cargo run --example datatypes    
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
//      Running `target\debug\examples\datatypes.exe`
// success :3
```

```

// there should be no doubt in the fact that charaters are numbers under the hood, what if we wanna print the ASCII code

 fn main () {
     for i in 'a'..='e' {
         println!("{}", i as u16)
     }
 }

//   Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
//      Running `target\debug\examples\datatypes.exe`
// 97
// 98
// 99
// 100
// 101
```
```
// rust makes a distinction between char and string
// string by "" double quotes
// andd char type by single quote


 fn main () {
     let c1: char = "अ"; // this is a string cuzza double string
     print_char(c1);
 }

 fn print_char(c: char) {
     println!("{}", c);
 }

// this is the issue
//     |
// 155 |     let c1: char = "अ"; // this is a string cuzza double ...
//     |             ----   ^^^ expected `char`, found `&str`        
//     |             |
//     |             expected due to this
//     |
// help: if you meant to write a `char` literal, use single quotes   
//     |
// 155 -     let c1: char = "अ"; // this is a string cuzza double string
// 155 +     let c1: char = 'अ'; // this is a string cuzza double string
//     |

// For more information about this error, try `rustc --explain E0308`.error: could not compile `hello` (example "datatypes") due to 1 previous 



```
```

fn main () {
    let c1: char = 'अ'; // now should run
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
//      Running `target\debug\examples\datatypes.exe`
// अ

```