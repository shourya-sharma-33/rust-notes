```
// sne means Statements and Expressions

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x*x;
        let x_cube = x_squared*x;

        // now theis Expression below wont be written in a semicolon
        // this means this is the value that will be asigned to the value of y
        // if i would put ; in front of it, it will return nothing, saving nothing in value of y
        x_cube + x_squared + x //NO SEMICOLON
    };

    println!("x is {}, y is {}",x,y);
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target\debug\examples\sne.exe`
// x is 5, y is 155
```