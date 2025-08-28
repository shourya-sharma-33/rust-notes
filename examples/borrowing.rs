// fn main () {
//     let x: i32 = 5;
//     let y: &i32 = &x;

//     assert_eq!(5, *y);
//     println!("sccess");
// }

// PS C:\Users\Dell\Downloads\hello> cargo run --example borrowing
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
//      Running `target\debug\examples\borrowing.exe`
// sccess
// PS C:\Users\Dell\Downloads\hello> 


//============================================

// how to know memory adress

// fn main () {
//     let x : i32 = 5;
//     let p : &i32 = &x;

//     println!("the memory adress of x is {:p}",p);
// }

// sccess
// PS C:\Users\Dell\Downloads\hello> cargo run --example borrowing
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
//      Running `target\debug\examples\borrowing.exe`
// the memory adress of x is 0x329a9ffa8c

//=======================================================================================

