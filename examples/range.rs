// for ranges we have to understand some concepts
// everything is number in the end of the day 
// so are abc's, so what we should know is
// when iterating from a to z, z will be excluded, because
// z was never an alphabet like human language, it was a ASCII value

// now we will make a code to iterate over number from -3 to 2, like sum

// fn main() {
//     let mut sum : i32 = 0; //-5 will be the answr

//     for i in -3..2 { 
//         sum += i;
//     }

//     assert!(sum == -5);
//     // code ran because sum came to be -5
//     // also range excludes the last one
//     // except if we put ..=<last> like in example below


//     for j in 'a'..='z' {
//         println!("{}", j);
//     }

// }


use std::ops::{Range, RangeInclusive};
// really no one uses it but its us importing from standard library the ops modyle and these functions

    
// fn main () {
//     assert_eq!((1..5), Range{start:1, end: 5});
//     assert_eq!((1..=5), RangeInclusive::new(1,5));
//     println!("successsss :3");
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
//      Running `target\debug\examples\range.exe`
// // successsss :3


