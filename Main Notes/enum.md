# Enum

1. way of defining a type with only one of a possible st of values
2. we can only access one variant of an enum at a time
3. can hold additional information using tuples
4. espetially useful when using match statements

```
enum IpAddr {
    V4(String),
    V6(String)
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Enum for IP address. an ip address can either be of v4 format or v6 format, each variabt in the enum holds a string

```
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn main() {
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
    println!("{}", Number::One as u8);
    println!("success");
}

// warning: `hello` (example "enums") generated 3 warnings
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.26s
//      Running `target\debug\examples\enums.exe`
// 1
// success
```


```

enum Message {
    Quit,
    Move { x : i32, y : i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main () {
    let msg1 : Message = Message::Move{ x : 1, y : 2};
    let msg2 : Message = Message::Write(String::from("hello, world!"));
    println!("success");
}

// ing: `hello` (example "enums") generated 5 warnings
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
//      Running `target\debug\examples\enums.exe`
// success
// PS C:\Users\Dell\Downloads\hello> 
```

```

#[derive(Debug)]
enum Message {
    Quit,
    Move {x : i32, y : i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn main() {
    let msgs : [Message ; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message (msg : Message) {
    println!("{:?}", msg);
} 

// warning: `hello` (example "enums") generated 3 warnings
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
//      Running `target\debug\examples\enums.exe`
// Quit
// Move { x: 1, y: 3 }
// ChangeColor(255, 255, 0)
```