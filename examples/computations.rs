// computations in rust is like anything else, except
// we can do some precision datatype stuff 

// fn main () {
//     assert!(1u32 + 2u32 == 3u32);
//     println!("success");
// }
// cargo run --example computations
// it ran with no issue
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
//      Running `target\debug\examples\computations.exe`
// success
//=============================================================

// lets see some error case

// fn main () {
//     println!("{}",1u32 - 2u32);
// }

// error will come cuz its unsigned and its overflowing

//    |
// 18 |     println!("{}",1u32 - 2u32);
//    |                   ^^^^^^^^^^^ attempt to compute `1_u32 - 2_u32`, which would overflow   |
//    = note: `#[deny(arithmetic_overflow)]` on by default


// fn main () {
//     assert!(9.6 / 3.2 == 3.0);
//     println!("success");
// }

// this wont compile cuz default float type is f64 but it goes too deep with prescision, cuz in boolean calculation 1.0 + 2.0 = 3.0000000002 which is goofy ahh
// so we can lower the prescision by defining a f32 type


// fn main () {
//     assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);
//     println!("success");
// }

// >>
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// PS C:\Users\Dell\Downloads\hello> cargo run --example computations
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
//      Running `target\debug\examples\computations.exe`
// success
// PS C:\Users\Dell\Downloads\hello> 


