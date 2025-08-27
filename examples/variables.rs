// 1. Assigned using `let` keyword
// 2. Print to standard output by print!() or println!()
// 3. Scope of a variable defined by the block of code in which it is declared
// 4. Function is a named block of code that is Reusable
// 5. Shadowing allows a variable to be redeclared in the same scope with same name

fn main() {
    let x : i32; //this i23 is a type annotation, will be talked about later
    let y : i32;

    assert_eq!(x,5); // assert_eq will check if equal
    println!("success :3");

    // now what will happen

    // error message

    // since y isnt used there will be a warning but no error
    //     warning: unused variable: `y`
    //  --> examples\variables.rs:9:9
    //   |
    // 9 |     let y : i32;
    //   |         ^ help: if this is intentional, prefix it with an underscore: `_y`
    //   |
    //   = note: `#[warn(unused_variables)]` on by default

    // basically if we dont initialise x it will return an error
    // the case for this will be if we ran the code with let: x : i32 without any initialisation

    // error message

    //     8  |     let x : i32; //this i23 is a type annotation, will be talked about later
    //    |         - binding declared here but left uninitialized
    // ...
    // 11 |     assert_eq!(x,5); // assert_eq will check if equal
    //    |     ^^^^^^^^^^^^^^^ `x` used here but it isn't initialized
    //    |
    // 


}