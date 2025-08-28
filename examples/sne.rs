// sne means Statements and Expressions

// fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x*x;
//         let x_cube = x_squared*x;

//         // now theis Expression below wont be written in a semicolon
//         // this means this is the value that will be asigned to the value of y
//         // if i would put ; in front of it, it will return nothing, saving nothing in value of y
//         x_cube + x_squared + x //NO SEMICOLON
//     };

//     println!("x is {}, y is {}",x,y);
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target\debug\examples\sne.exe`
// x is 5, y is 155


// fn main () {
//     let v = {
//         let mut x : i32 = 1;
//         x += 2
//     };
//     assert_eq!(v, 3);
//     println!("successsss:3")
// }

// reason it will show an error cuz of the datatype mismatch, we have to specify what datatype we are talking about

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0308]: mismatched types
//   --> examples\sne.rs:30:19
//    |
// 30 |     assert_eq!(v, 3);
//    |                   ^ expected `()`, found integer

// For more information about this error, try `rustc --explain E0308`.error: could not compile `hello` (example "sne") due to 1 previous 
// error


// fn main () {
//     let v : i32 = {
//         let mut x : i32 = 1;
//         x += 2
//     };
//     assert_eq!(v, 3i32);
//     println!("successsss:3")
// }

// you cannot return the variable like that too

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0308]: mismatched types
//   --> examples\sne.rs:50:9
//    |
// 50 |         x += 2
//    |         ^^^^^^ expected `i32`, found `()`

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `hello` (example "sne") due to 1 previous 
// error

// fn main () {
//     let v : i32 = {
//         let mut x : i32 = 1;
//         x = x + 2;
//         x
//     };
//     assert_eq!(v, 3i32);
//     println!("successsss:3")
// }

// only this syntax run
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.60s
//      Running `target\debug\examples\sne.exe`
// successsss:3


// fn main () {
//     let s: i32 = sum(1 ,2);
//     assert_eq!(s, 3);
//     println!("success :3");
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.42s
//      Running `target\debug\examples\sne.exe`
// success :3

//==================================================================================


// Diverging function in rust

// Diverging function never returns to the caller
// so they may be used in places where a value of any type is expected

// fn main () {
//     println!("success :3");
// }

// fn get_option(tp : u8) -> Option<i32> {
// // tp : u8 means datatype is u8
// // and returns option<i32> will discuss later
//     match tp {
// // match is kinda like swich in javascript
// // if tp is yes then it that block of code will run
// // else the _ one will
//         1 => {

//         }
//         _ => {

//         }
//     };

//     never_return_fn();
// }

// fn never_return_fn() -> ! {
//     panic!();
// }
// panic wont return anything




//====================//====================//====================


// fn main () {
//     let b = false;

//     let _v = match b{
//         true => 1,
//         false => {
//             println!("success");
//             panic!("we have no value for false but we can panic")
//         }
//     };

//     println!("excercise failed if printing out this line :3")
// }

//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
//      Running `target\debug\examples\sne.exe`
// success

// thread 'main' panicked at examples\sne.rs:149:13:
// we have no value for false but we can panic
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// error: process didn't exit successfully: `target\debug\examples\sne.exe` (exit code: 101)


// lets run the same code with boolean true

fn main () {
    let b = true;

    let _v = match b{
        true => 1,
        false => {
            println!("success");
            panic!("we have no value for false but we can panic")
        }
    };

    println!("we skipped panic")
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.62s
//      Running `target\debug\examples\sne.exe`
// we skipped panic

//============================================================================================

