// 1. Assigned using `let` keyword
// 2. Print to standard output by print!() or println!()
// 3. Scope of a variable defined by the block of code in which it is declared
// 4. Function is a named block of code that is Reusable
// 5. Shadowing allows a variable to be redeclared in the same scope with same name

//==================================================
//==================================================
//==================================================

// fn main() {
//     let x : i32 = 5; //this i23 is a type annotation, will be talked about later
//     let y : i32;

//     assert_eq!(x,5); // assert_eq will check if equal
//     println!("success :3");

// }

// the code ran but only gave a warning for y

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// warning: unused variable: `y`
//  --> examples\variables.rs:9:9
//   |
// 9 |     let y : i32;
//   |         ^ help: if this is intentional, prefix it with an underscore: `_y`
//   |
//   = note: `#[warn(unused_variables)]` on by default

// warning: `hello` (example "variables") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
//      Running `target\debug\examples\variables.exe`
// success :3

//==================================================


// use mut to mark a variable as mutable

// fn main () {
//     let mut x = 1;
//     x += 2;

//     assert_eq!(x,3);
//     println!("success :3")
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.76s
//      Running `target\debug\examples\variables.exe`
// success :3


//==================================================

// how to put variables in string

// fn main () {
//     let x: i32 = 10;
//     let y: i32 = 5;
//     println!("{} twos are {}", y,x);
// }

// output
// 5 twos are 10

//==================================================

// scope is a range within the program for which the item is valid

// fn main() {
//     let x : i32 = 10;
//     {
//         let y : i32 = 20;
//         println!("{} twos are {}", x,y);
//     }

    
//     println!("{} twos are {}", x,y);
// }

// this code wont compile

// error[E0425]: cannot find value `y` in this scope
//   --> examples\variables.rs:80:34
//    |
// 80 |     println!("{} twos are {}", x,y);
//    |                                  ^ help: a local variable with a similar name exists: `x`

// For more information about this error, try `rustc --explain E0425`.error: could not compile `hello` (example "variables") due to 1 previous error



// another example

// fn main() {
//     println!("{}, world",x)
// }

// fn define_x() {
//     let x : &str = "hello";
// }


// // this too wont compile
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0425]: cannot find value `x` in this scope
//   --> examples\variables.rs:98:26
//    |
// 98 |     println!("{}, world",x)
//    |                          ^ not found in this scope

// warning: unused variable: `x`
//    --> examples\variables.rs:102:9
//     |
// 102 |     let x : &str = "hello";
//     |         ^ help: if this is intentional, prefix it with an underscore: `_x`
//     |
//     = note: `#[warn(unused_variables)]` on by default

// For more information about this error, try `rustc --explain E0425`.warning: `hello` (example "variables") generated 1 warning
// error: could not compile `hello` (example "variables") due to 1 previous error; 1 warning emitted


// now a question would be how to fix it?
// lets try multiple methods till we get it right

// fn main(){

// }

// fn define_x(){
//     let x : &str = "hello";
//     println!("{}, world", x)
// }


// it would compile but show no output??
// there is ofc a warning but the code runs
// but why this issue
// well code only runs in the main file so initialise there


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// warning: function `define_x` is never used
//    --> examples\variables.rs:133:4
//     |
// 133 | fn define_x(){
//     |    ^^^^^^^^
//     |
//     = note: `#[warn(dead_code)]` on by default

// warning: `hello` (example "variables") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
//      Running `target\debug\examples\variables.exe`
// fn main(){
//     define_x();
// }

// fn define_x(){
//     let x : &str = "hello";
//     println!("{}, world", x)
// }

// now it ran
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
//      Running `target\debug\examples\variables.exe`
// hello, world

//==================================================

// Shadowing
// you can declare a new variable with the same name as a previous variabl,
// here we can say the first one is shadowed by second one

// fn main () {
//     let x : i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//         println!("we are in the inner scope and it compiling cuz assert_eq ran");
//     }

//     assert_eq!(x, 5);
//     println!("here its working cuz in outter scope the valye is same");

//     let x: i32 = 42;
//     println!("{}", x);
// }


// it ran
// PS C:\Users\Dell\Downloads\hello> cargo run --example variables    
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
//      Running `target\debug\examples\variables.exe`
// we are in the inner scope and it compiling cuz assert_eq ran
// here its working cuz in outter scope the valye is same
// 42




// now lets run a peice of code and learn somethings

// fn main() {
//     let mut x : i32 = 1;
//     // by default every variable is mutable
//     x = 7;
//     // now we have reassigned the valye of x
//     let mut x = x;
//     // here this like is practically useless but here we are redeclaring the x variable
//     // here we are shadowing and rebinding

//     let y: i32 = 4;
//     // shadowing
//     let y : &str = "mrrp meow :3333";
//     println!("{} success :3", y);

// }

// the code obviosly worked but read the warning

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// warning: value assigned to `x` is never read
//    --> examples\variables.rs:209:13
//     |
// 209 |     let mut x : i32 = 1;
//     |             ^
//     |
//     = help: maybe it is overwritten before being read?
//     = note: `#[warn(unused_assignments)]` on by default

// warning: unused variable: `x`
//    --> examples\variables.rs:213:13
//     |
// 213 |     let mut x = x;
//     |             ^ help: if this is intentional, prefix it with an underscore: `_x`
//     |
//     = note: `#[warn(unused_variables)]` on by default

// warning: unused variable: `y`
//    --> examples\variables.rs:217:9
//     |
// 217 |     let y: i32 = 4;
//     |         ^ help: if this is intentional, prefix it with an underscore: `_y`

// warning: unused variable: `y`
//    --> examples\variables.rs:219:9
//     |
// 219 |     let y : &str = "mrrp meow :3333";
//     |         ^ help: if this is intentional, prefix it with an underscore: `_y`

// warning: variable does not need to be mutable
//    --> examples\variables.rs:213:9
//     |
// 213 |     let mut x = x;
//     |         ----^
//     |         |
//     |
//     = note: `#[warn(unused_mut)]` on by default

// warning: `hello` (example "variables") generated 5 warnings (run `cargo fix --example "variables"` to apply 1 suggestion)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.23s
//      Running `target\debug\examples\variables.exe`
// success :3
// PS C:\Users\Dell\Downloads\hello>  cargo run --example variables   
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// warning: value assigned to `x` is never read
//    --> examples\variables.rs:209:13
//     |
// 209 |     let mut x : i32 = 1;
//     |             ^
//     |
//     = help: maybe it is overwritten before being read?
//     = note: `#[warn(unused_assignments)]` on by default

// warning: unused variable: `x`
//    --> examples\variables.rs:213:13
//     |
// 213 |     let mut x = x;
//     |             ^ help: if this is intentional, prefix it with an underscore: `_x`
//     |
//     = note: `#[warn(unused_variables)]` on by default

// warning: unused variable: `y`
//    --> examples\variables.rs:217:9
//     |
// 217 |     let y: i32 = 4;
//     |         ^ help: if this is intentional, prefix it with an underscore: `_y`

// warning: variable does not need to be mutable
//    --> examples\variables.rs:213:9
//     |
// 213 |     let mut x = x;
//     |         ----^
//     |         |
//     |         help: remove this `mut`
//     |
//     = note: `#[warn(unused_mut)]` on by default

// warning: `hello` (example "variables") generated 4 warnings (run `cargo fix --example "variables"` to apply 1 suggestion)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
//      Running `target\debug\examples\variables.exe`
// mrrp meow :3333 success :3

//==================================================


// unused variables

// #[allow(unused_variables)]
// fn main() {
//     let x = 1;
// }

// it ran with no warning

//============================================================
// destructuring

// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x,3);
//     assert_eq!(y,2);
//     println!("successss {},{}",x,y);
// }

// >>
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
//      Running `target\debug\examples\destructuring.exe`
// successss 3,2

//============================================================

// more on destructuring

fn main(){
    let (x,y);

    (x,..) = (3,4);
    // this syntax says that first value is assigned as first value of tuple else we dont care about
    [..,y] = [3,2];

    assert_eq!([x,y], [3,2]);
    println!("success")
}

// it ran
// PS C:\Users\Dell\Downloads\hello> cargo run --example variables    
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
//      Running `target\debug\examples\variables.exe`
// success