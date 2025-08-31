// enum Direction {
//     East,
//     North,
//     South,
//     West
// }

// fn main () {
//     let dire : Direction = Direction::South;
//     match dire {
//         Direction::East => println!("east"),
//         Direction::South | Direction::North => {
//             println!("south or north");
//         },
//         _ => println!("east"), //underscore says rest of cases
//     };
// }

// // warning: `hello` (example "patternmatch") generated 1 warning
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
// //      Running `target\debug\examples\patternmatch.exe`
// // south or north
// // PS C:\Users\Dell\Downloads\hello> 


// fn main () {
//     let boolean : bool = true;

//     let binary : u8 = match boolean {
//         true => 1,
//         false => 0
//     };

//     assert_eq!(binary, 1);
//     println!("success");
// }


// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
// //      Running `target\debug\examples\patternmatch.exe`
// // success


// enum Message {
//     Quit,
//     Move {x : i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main () {
//     let msg : [Message; 3] = [
//         Message::Quit,
//         Message::Move{x : 1, y: 3},
//         Message::ChangeColor(255, 255, 0)
//     ];

//     for ms in msg {
//         show_message(ms)
//     }

//     println!("SUCcess");
// }

// fn show_message(msg : Message) {
//     match msg {
//         Message::Quit => println!("user quit"),
//         Message::Move{x : a, y : b} => println!("{}, {} these are the direction in x and y axis", a, b),
//         Message::ChangeColor(a, b, c) => println!("{},{},{} color has been chnaed",a,b,c ),
//         _ => println!("MEOW")
//     }
// }



// fn main () {
//     let alphabet = ['a', 'E', 'Z', '0', 'x', 'Y'];

//     for ab in alphabet {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
//     }
//     println!("sycceewsss");
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.87s
//      Running `target\debug\examples\patternmatch.exe`
// sycceewsss


// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main () {
//     let mut count = 0;
//     // vectors are like arrays but dynamic in sizw
//     let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
//     for e in v {
//         if matches!(e, MyEnum::Foo) {
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);
//     println!("success");
// }

// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
// //      Running `target\debug\examples\patternmatch.exe`
// // success


// fn main () {
//     let o : Option<i32> = Some(7);

//     if let Some(i) = o {
//         println!("this is a really long String and {:?}", i);
//         println!("success");
//     }
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.90s
//      Running `target\debug\examples\patternmatch.exe`
// this is a really long String and 7
// success


// fn main () {
//     let age: Option<i32> = Some(30);
//     if let Some(age) = age {
//         assert_eq!(age, 30);
//     }

//     match age {
//         Some(age) => println!("age is a new variable {}", age),
//         _ => {}
//     }
// }

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.65s
//      Running `target\debug\examples\patternmatch.exe`
// age is a new variable 30


// fn main () {}
// fn match_number(n : i32) {
//     match n {
//         1 => println!("one"),
//         2 | 3 | 4 | 5 => println!("match 2 -> 5"),
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match -infinite -> 0 or 11 -> +infinity");
//         }
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p: Point = Point { x: 10, y: 10 };

//     match p {
//         Point { x: x, y: 0 } => println!("on the x axis at {}", x),
//         Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("on the y axis is {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }

// enum Message {
//     Hello { id : i32 },
// }

// fn main () {
//     let msg : Message = Message::Hello { id : 5 };

//     match msg {
//         Message::Hello {
//             id : id @ 3..=7,
//         } => println!("found an id in range [3, 7]:{}", id),
//         Message::Hello { id : newid @ (10 | 11 | 12)} => {
//             println!("found an id in another range [10, 12]: {}", newid)
//         },
//         Message::Hello { id } => println!("found some other ids, {}", id),
//     }
    
// }

// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
// //      Running `target\debug\examples\patternmatch.exe`
// // found an id in range [3, 7]:5

// fn main () {
//     let num : Option<i32> = Some(4);

//     let split : i32 = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x) => assert!(x == split),
//         None => (),
//     }
//     println!("SUCCESS");
// }
// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
// //      Running `target\debug\examples\patternmatch.exe`
// // SUCCESS

// fn main () {
//     let numbers = (2, 4, 8, 16, 128, 256, 512, 1024, 2048);

//     match numbers {
//         (first, .., last) => {
//             assert_eq!(first, 2);
//             assert_eq!(last, 2048);
//         }
//     }
//     println!("SUCCESS");
// }

// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
// //      Running `target\debug\examples\patternmatch.exe`
// // SUCCESS

// fn main () {
//     let mut v: String = String::from("heello, ");
//     let r : &mut String = &mut v;

//     match r {
//         value => value.push_str(" world")
//     }
// }

// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
// //      Running `target\debug\examples\patternmatch.exe`




