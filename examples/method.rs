// struct Rectangle {
//     width : u32,
//     height : u32
// }

// impl Rectangle {
//     fn area(self) -> u32 {
//         self.width  * self.height
//     }
// }

// fn main (){
//     let rect1: Rectangle = Rectangle {width : 30, height : 50};
//     assert_eq!(rect1.area(), 1500);
//     println!("success");
// }

// //    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.33s
// //      Running `target\debug\examples\method.exe`
// // success


// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(self : &Self) {
//         println!("the current state is {}", self.color);
//     }

//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }

// }

// fn main () {
//     println!("success");
// }

// //   |            ^^^^^^^^^^^^

// // warning: `hello` (example "method") generated 2 warnings
// //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
// //      Running `target\debug\examples\method.exe`
// // success