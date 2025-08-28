// if u are in this file first
// go read the ownership.md it contains all the explaination
// to whatever we will be doing here so without that u wont be getting anything



// fn main() {
//     let x : String =  String::from("hello");
//     let y : String = x;

//     println!("{},{}",x,y);
// }

// here x should not print since ownership transfer is taking place
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0382]: borrow of moved value: `x`
//   --> examples\ownership.rs:10:22
//    |
// 7  |     let x : String =  String::from("hello");
//    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
// 8  |     let y : String = x;
//    |                      - value moved here
// 9  |
// 10 |     println!("{},{}",x,y);
//    |                      ^ value borrowed here after move
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//    |
// 8  |     let y : String = x.clone();
//    |                       ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `hello` (example "ownership") due to 1 previous error



//  fn main () {
//      let x : String = String::from("Hello");
//      // now we will make a deep copy
//      let y : String = x.clone();

//      println!("{},{}",x,y);
//  }

// now there a copy was made
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.68s
//      Running `target\debug\examples\ownership.exe`
// Hello,Hello

// fn main () {
//     let s1 : String = String::from("hello");
//     let s2 : String = take_ownership(s1);
//     println!("{}", s2);
// }

// fn take_ownership(s : String) -> String {
//     println!("{}", s);
//     s
// }

// ownership from s1 --> take_ownership() --> s2
// 

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
//      Running `target\debug\examples\ownership.exe`
// hello
// hello

// ========================================================================================================

// into_bytes(self)
// converts string to byte vectors
// this consumes the string, so we actually cannot reuse the old mutable

// fn main () {
//     let s = String::from("hello");
//     let bytes = s.into_bytes();

//     assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
//     println!("success");

// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
//      Running `target\debug\examples\ownership.exe`
// success


// if we dont want our variable to get consumed what we can do is we can simply 
// use as_bytes
// fn main () {
//     let s = String::from("hello");
//     let bytes = s.as_bytes();

//     assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
//     println!("success and {} can be reused", s);

// }


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
//      Running `target\debug\examples\ownership.exe`
// success and hello can be reused



// now we will see more cool stuff with ownership and 
// solve some doubts that can appear with it

// fn main (){
//     let s : String = String::from("hello");

//     print_str(s);
//     println!("{}", s);
    
// }

// fn print_str(s : String) {
//     println!("{}", s)
// }


// now this should show an error cuz the owner ship was passed to print_str function

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0382]: borrow of moved value: `s`
//    --> examples\ownership.rs:119:20
//     |
// 116 |     let s : String = String::from("hello");
//     |         - move occurs because `s` has type `String`, which 
// does not implement the `Copy` trait
// 117 |
// 118 |     print_str(s);
//     |               - value moved here
// 119 |     println!("{}", s);
//     |                    ^ value borrowed here after move        
//     |
// note: consider changing this parameter type in function `print_str` to borrow instead if owning the value isn't necessary
//    --> examples\ownership.rs:123:18
//     |
// 123 | fn print_str(s : String) {
//     |    ---------     ^^^^^^ this parameter takes ownership of the value
//     |    |
//     |    in this function
//     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in 
// Nightly builds, run with -Z macro-backtrace for more info)       
// help: consider cloning the value if the performance cost is acceptable
//     |
// 118 |     print_str(s.clone());
//     |                ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `hello` (example "ownership") due to 1 previous error


//===============================//===============================//===============================//===============================//===============================

// lets check the mutabilituy of String ddatatype

// fn main () {
//     let s : String = String::from("hello");
//     s.push_str("world");
//     println!("success!")
// }


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
//    --> examples\ownership.rs:167:5
//     |
// 167 |     s.push_str("world");
//     |     ^ cannot borrow as mutable
//     |
// help: consider changing this to be mutable
//     |
// 166 |     let mut s : String = String::from("hello");
//     |         +++

// For more information about this error, try `rustc --explain E0596`.
// error: could not compile `hello` (example "ownership") due to 1 previous error

// now lets try transfering the owner ship to a mutable variable

// fn main () {
//     let s : String = String::from("hello");
//     let mut s1 = s;
//     s1.push_str("world");
//     println!("success!")
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.74s
//      Running `target\debug\examples\ownership.exe`
// success!



//===============================================================================================


// fn main () {
//     let t : (String, String) = (String::from("hello"), String::from("world"));

//     let _s : String = t.0;
//     // access the 0th index 

//     println!("{}", _s);
//     println!("{}", t.1);
//     // no issue
//     //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
//     //      Running `target\debug\examples\ownership.exe`
//     // hello
//     // world

//     println!("{}", t.0);
//     // this would show moved
//     //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     // error[E0382]: borrow of moved value: `t.0`
//     //    --> examples\ownership.rs:221:20
//     //     |
//     // 209 |     let _s : String = t.0;
//     //     |                       --- value moved here
//     // ...
//     // 221 |     println!("{}", t.0);
//     //     |                    ^^^ value borrowed here after move
//     //     |

// }

// // now lets do it with .clone method
fn main () {
    let t: (String, String) = (String::from("hello"), String::from("world"));
    let (s1, s2) = t.clone();
    println!("{:?}, {:?}, {:?}",s1,s2,t)
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
//      Running `target\debug\examples\ownership.exe`
// "hello", "world", ("hello", "world")