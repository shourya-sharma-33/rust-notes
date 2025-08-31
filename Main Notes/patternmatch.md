# pattern match

1. powerful contruct that allows to compare a value against a set of patterns, then execute diffrent code based on which pattern matches
2. patterns can be made up of literal values, variable names, wildcards, etc
3. in match, all possible cases must be handled, enforced by the complier

## Example Match

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny -> 1,
        Coin::Nickel -> 5,
        Coin::Dime -> 10,
        Coin::Quarter -> 25
    }
}
```

here, we have an enum `coin` which holds diffrent denominations of US coins

`value_in_cents()` takes as argument a `Coin` which can only hold one field of enum and checks which field in the enum has been matched then returns the right amount of u8

## if let statements and better approach

```
enum Direction {
    East,
    North,
    South,
    West
}

fn main () {
    let dire : Direction = Direction::South;
    match dire {
        Direction::East => println!("east"),
        Direction::South | Direction::North => {
            println!("south or north");
        },
        _ => println!("east"), //underscore says rest of cases
    };
}

// warning: `hello` (example "patternmatch") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
//      Running `target\debug\examples\patternmatch.exe`
// south or north
// PS C:\Users\Dell\Downloads\hello> 


```

**match is an expression, so we can use it as assignments**

```

fn main () {
    let boolean : bool = true;

    let binary : u8 = match boolean {
        true => 1,
        false => 0
    };

    assert_eq!(binary, 1);
    println!("success");
}


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
//      Running `target\debug\examples\patternmatch.exe`
// success
```

```


enum Message {
    Quit,
    Move {x : i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main () {
    let msg : [Message; 3] = [
        Message::Quit,
        Message::Move{x : 1, y: 3},
        Message::ChangeColor(255, 255, 0)
    ];

    for ms in msg {
        show_message(ms)
    }

    println!("SUCcess");
}

fn show_message(msg : Message) {
    match msg {
        Message::Quit => println!("user quit"),
        Message::Move{x : a, y : b} => println!("{}, {} these are the direction in x and y axis", a, b),
        Message::ChangeColor(a, b, c) => println!("{},{},{} color has been chnaed",a,b,c ),
        _ => println!("MEOW")
    }
}
```

```


fn main () {
    let alphabet = ['a', 'E', 'Z', '0', 'x', 'Y'];

    for ab in alphabet {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
    }
    println!("sycceewsss");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.87s
//      Running `target\debug\examples\patternmatch.exe`
// sycceewsss
```

```

enum MyEnum {
    Foo,
    Bar
}

fn main () {
    let mut count = 0;
    // vectors are like arrays but dynamic in sizw
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    assert_eq!(count, 2);
    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
//      Running `target\debug\examples\patternmatch.exe`
// success
```

## if let statements

for some cases, when matching enums `enums`, `match` is too heavy, we can use `if let` instead

```

fn main () {
    let o : Option<i32> = Some(7);

    if let Some(i) = o {
        println!("this is a really long String and {:?}", i);
        println!("success");
    }
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.90s
//      Running `target\debug\examples\patternmatch.exe`
// this is a really long String and 7
// success
```

```


fn main () {
    let age: Option<i32> = Some(30);
    if let Some(age) = age {
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("age is a new variable {}", age),
        _ => {}
    }
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.65s
//      Running `target\debug\examples\patternmatch.exe`
// age is a new variable 30
```

```

fn main () {}
fn match_number(n : i32) {
    match n {
        1 => println!("one"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinity");
        }
    }
}
```

```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p: Point = Point { x: 10, y: 10 };

    match p {
        Point { x: x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("on the y axis is {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


```

**what does this @ mean in this syntax ?**

y @ pattern means that if y matches the pattern then store the exact value in variable y


## `@` operators


```

enum Message {
    Hello { id : i32 },
}

fn main () {
    let msg : Message = Message::Hello { id : 5 };

    match msg {
        Message::Hello {
            id : id @ 3..=7,
        } => println!("found an id in range [3, 7]:{}", id),
        Message::Hello { id : newid @ (10 | 11 | 12)} => {
            println!("found an id in another range [10, 12]: {}", newid)
        },
        Message::Hello { id } => println!("found some other ids, {}", id),
    }
    
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
//      Running `target\debug\examples\patternmatch.exe`
// found an id in range [3, 7]:5
```

```

fn main () {
    let num : Option<i32> = Some(4);

    let split : i32 = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x == split),
        None => (),
    }
    println!("SUCCESS");
}
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
//      Running `target\debug\examples\patternmatch.exe`
// SUCCESS
```

```

fn main () {
    let numbers = (2, 4, 8, 16, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
    println!("SUCCESS");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
//      Running `target\debug\examples\patternmatch.exe`
// SUCCESS

```

```

fn main () {
    let mut v: String = String::from("heello, ");
    let r : &mut String = &mut v;

    match r {
        value => value.push_str(" world")
    }
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
//      Running `target\debug\examples\patternmatch.exe`

```


```
struct Rectangle {
    width : u32,
    height : u32
}

impl Rectangle {
    fn area(self) -> u32 {
        self.width  * self.height
    }
}

fn main (){
    let rect1: Rectangle = Rectangle {width : 30, height : 50};
    assert_eq!(rect1.area(), 1500);
    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.33s
//      Running `target\debug\examples\method.exe`
// success
```
