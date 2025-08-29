
// fn main () {
//     let s : str = "Hello, World";

//     println!("successsss:3");
// }

// there are some errors

// =============================================

// fn main () {

// let mut s : String = String::from("");
// s.push_str("uwu hewwo meow mrrp");
// s.push(':');
// s.push('3');

// println!("{}", s);
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
//      Running `target\debug\examples\str.exe`
// uwu hewwo meow mrrp:3

//========================================================================

// fn main () {
//     let mut s : String = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";

//     println!("{}", s);

// }


// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
// //      Running `target\debug\examples\str.exe`
// // hello, world!

//==================================================================================================================


// fn main () {
//     let s : String = String::from("i lob dogs");
//     let s1 = s.replace("dogs", "cats");
//     assert_eq!(s1, "i lob cats");
//     println!("scuccess :3")
//     //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.16s
//     //      Running `target\debug\examples\str.exe`
//     // scuccess :3
// }

//===================================================

// there is a really good question

// fn main () {
//     let s: &str = "hello, world";
//     greetings(s)
// }

// fn greetings(s : String) {
//     println!("{}", s)
// }

// PS C:\Users\Dell\Downloads\hello> cargo run --example str
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0308]: mismatched types
//   --> examples\str.rs:65:15   
//    |
// 65 |     greetings(s)
//    |     --------- ^- help: try using a conversion method: `.to_string()`
//    |     |         |
//    |     |         expected `String`, found `&str`
//    |     arguments to this function are incorrect
//    |
// note: function defined here
//   --> examples\str.rs:68:4
//    |
// 68 | fn greetings(s : String) {
//    |    ^^^^^^^^^ ----------

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `hello` (example "str") due to 1 previous error


// how to fix the code above??

// fn main () {
//     let s : &str = "hello, world";
//     greetings(s.to_string())
// }
// fn greetings(s : String) {
// println!("{}", s)
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.02s
//      Running `target\debug\examples\str.exe`
// hello, world

// ===================================================================================


// fn main () {
//     let s = "hello, world".to_string();
//     let s1 : &str = s;
//     println!("success");
// }
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0308]: mismatched types
//    --> examples\str.rs:113:21
//     |
// 113 |     let s1 : &str = s;
//     |              ----   ^ expected `&str`, found `String`
//     |              |
//     |              expected due to this
//     |
// help: consider borrowing here
//     |
// 113 |     let s1 : &str = &s;
//     |                     +

// For more information about this error, try `rustc --explain E0308`. 
// error: could not compile `hello` (example "str") due to 1 previous error

// fn main () {
//     let s : String = "hello, world".to_string();
//     let s1 : &str = &s;
//     println!("success");
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// warning: unused variable: `s1`
//    --> examples\str.rs:135:9
//     |
// 135 |     let s1 : &str = &s;
//     |         ^^ help: if this is intentional, prefix it with an underscore: `_s1`
//     |
//     = note: `#[warn(unused_variables)]` on by default

// warning: `hello` (example "str") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
//      Running `target\debug\examples\str.exe`
// success


// fn main () {
//     let s1 : String = String::from("meowmeowmeowmeow");
//     let h : &str = &s1[0..1];
//     println!("{}", h);
    
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
//      Running `target\debug\examples\str.exe`
// m


// fn main () {
//     let arr0 = [1, 2, 3];
//     let arr : [_; 3] = ['a', 'b', 'c'];
//     // this underscore says compiler will pick its own type
    
//     // arrays are stack allocated
//     // char takes 4 bytes in rust : unicode char

//     assert!(std::mem::size_of_val(&arr) == 4*3);

//     println!("succeess :3")
// }

// warning: `hello` (example "str") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
//      Running `target\debug\examples\str.exe`
// succeess :3

// refer to the md for this

// fn main () {
//     let arr: [char; 3] = ['क', 'ख', 'म']; 

//     let slice = &arr[..2];

//     assert!(std::mem::size_of_val(&slice) == 8);
//     println!("success"); 
// }

// assertion failed: std::mem::size_of_val(&slice) == 8
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// error: process didn't exit successfully: `target\debug\examples\str.exe` (exit code: 101)

// now lets try with 16


fn main () {
    let arr: [char; 3] = ['क', 'ख', 'म']; 

    let slice = &arr[..2];

    assert!(std::mem::size_of_val(&slice) == 16);
    println!("success"); 
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
//      Running `target\debug\examples\str.exe`
// success