# String vs &str

1. a `String` is a heap-allocated string type that owns its contents and is mutable
2. A &str is an immutable sequence of UTF-8 bytes in memory and doesnt own the underlying data and is immutable
3. think of &str as a view on a sequence of characters (Stored as UTF-8 bytes) in memory
4. use &str if u just want a view of a string
5. &str is more lightweight and efficient that String
6. use String if u needa own that data and be able to mutate it

# String Literal

1. A string literal is a sequece of characters enclosed in double quotes
2. fixed sizze, compile time  known sequence of UTF-8 bytes
3. the type is &static str which indicates the data is stored in static storage, meaning its valid throughout the entire lifetime of the programee
4. the data is hardcoded into the executable and stored in read only memory, meaning thy are immutable


# example string slice

```
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

the string slice `world` points to a sequrnce of characters stored on the heap


s -------------------->[h,e,l,l,o, ,w,o,r,l,d]
                               ^
                               |
world --------------------------

they both are pointing to this hello world stored on heap

--------

❌

here 'Hello, World' is hardcoded in memory, we cannot use str type like that
```
fn main () {
    let s : str = "Hello, World";

    println!("successsss:3");
}

```

✅

```
fn main () {
    let s : &str = "Hello, World";

    println!("successsss:3");
}

```

--------

```

fn main () {

let mut s : String = String::from("");
s.push_str("uwu hewwo meow mrrp");
s.push(':');
s.push('3');

println!("{}", s);
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
//      Running `target\debug\examples\str.exe`
// uwu hewwo meow mrrp:3
```


```

fn main () {
    let mut s : String = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);

}


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
//      Running `target\debug\examples\str.exe`
// hello, world!
```

```

fn main () {
    let s : String = String::from("i lob dogs");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "i lob cats");
    println!("scuccess :3")
    //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.16s
    //      Running `target\debug\examples\str.exe`
    // scuccess :3
}
```

-----------

**more stringb method can be found under string module
**

you can only concat a `String` with `&str` and `String`'s ownership can be moved to another variable

```
fn main () {
    let s1 : String = String::from("hello,");
    let s2 : String = String::from("world!");
    let s3 = s1 + s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
```


```

// there is a really good question

fn main () {
    let s: &str = "hello, world";
    greetings(s)
}

fn greetings(s : String) {
    println!("{}", s)
}

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

```

```

// how to fix the code above??

fn main () {
    let s : &str = "hello, world";
    greetings(s.to_string())
}
fn greetings(s : String) {
println!("{}", s)
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.02s
//      Running `target\debug\examples\str.exe`
// hello, world

```

## &str to String

we can use `String::from` or `to_string` to convert a `&str` to `String`

```
fn main () {
    let s = "hello, world".to_string();
    let s1 : &str = s;
    println!("success");
}

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
```

```

fn main () {
    let s : String = "hello, world".to_string();
    let s1 : &str = &s;
    println!("success");
}

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
```

## Escape String

we can directly insert ascii characers in string using backslash

```
fn main () {
    let byte_escape = "I am writing Ru\x73\x74!";
    println!("what are you doing\x3F (\\x3F double back slash to literally print backslash x3F) {}", byte_escape);
}
```

we can use `r""` to have all inner stuff literally print

```
fn main () {
    println!(r"meow /x3F");
}
```

## String Index

you cant use inddex to access a char in a string but you can use slice `&s1[start..end]`

```
fn main () {
    let s1 : String = String::from("meowmeowmeowmeow");
    let h : &str = &s1[0..1];
    println!("{}", h);
    
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
//      Running `target\debug\examples\str.exe`
// m
```
