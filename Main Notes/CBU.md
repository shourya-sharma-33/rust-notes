```
// cbu means character boolean and units

// char
// in rust the character type is big enough to hold every single unicode value


use std::mem::size_of_val;

fn main () {
    let c1: char = 'a'; //4bytes
    let c2: char = 'अ'; //4bytes
    println!("{} is a, {} is अ", size_of_val(&c1), size_of_val(&c2))
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
//      Running `target\debug\examples\cbu.exe`
// 4 is a, 4 is अ

```

```

fn main () {
    let f: bool = false;
    let t: bool = true && false;

    assert_eq!(t, f);
    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target\debug\examples\cbu.exe`
// success
```

```

fn main () {
    let _v : () = (); // this is called a unit type

    let v : (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_returning_unit());
    println!("success :3");
}

fn implicitly_returning_unit() {
    println!("compiler will return unit type if no return is mentioned");
}

// code is running just some warning :3
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// warning: unused variable: `v`
//   --> examples\cbu.rs:40:9
//    |
// 40 |     let v : (i32, i32) = (2, 3);
//    |         ^ help: if this is intentional, prefix it with an undePS C:\Users\Dell\Downloads\hello> cargo run --example cbu
// warning: unused variable: `v`
//   --> examples\cbu.rs:40:9
//    |
// 40 |     let v : (i32, i32) = (2, 3);
//    |         ^ help: if this is intentional, prefix it with an underscore: `_v`
//    |
//    = note: `#[warn(unused_variables)]` on by default

// warning: `hello` (example "cbu") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
//      Running `target\debug\examples\cbu.exe`
// compiler will return unit type if no return is mentioned
// success :3


// this tells us withoyt anything returning, a unit type is explicityly returned regardless
```

```

// what is size of a union type
use std::mem::size_of_val;

fn main() {
    let unit : () = ();
    assert!(size_of_val(&unit) == 0);
    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
//      Running `target\debug\examples\cbu.exe`
// success
```