# Method

method are similar to fucntions : declare with fn and have parameter and a return value, unlike function, method are defined within the context of a struct (or an enum or a trait object) and their firstparameter is always self, which represents the instance of the struct the method is being called on

1. Function that is associated with a perticular type or struct
2. takes paramters and returns a value, but defined as a member of a struct or enum
3. called using dot notation (like accessing members of a struct)

4. implemented through an `impl` block

**example methd**
```
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

imp Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

inside  the impl block for the type Rectangle we define the method area() that doest take any argument and returns the product of the product of width and height of an instance as a u32 integer

```
let rect1 = Rectangle {
    width : 30,
    height : 50
}

println!(
    "the area of the rectangle is {} square pixel", rect1.area()
);
```

here we create an instance of rectangg;e with value for width and height then w can call the method using dot notation on the instance weve created

`self` refers to the instance the method is called upon, in this case `rect1`

## Associated Functions

1. function that is assocaited with a struct or an enum, but doesnt take an instance as its first paramerter

2. calle using the name of type, not an instance of it

3. often used as constructors for a struct or enum

## associated fucntion example

```
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}
impl Rectangle {
    fn new (width : u32, height : u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}

let rect1 : Rectangle = Rectangle::new(5, 10);

println!("rectangle. {:?}, rect1");
```

**we can then call the associated funtionj by using the name of structb and the method name saperated by ::**

```
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


```

```

struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(self : &Self) {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }

}

fn main () {
    println!("success");
}

//   |            ^^^^^^^^^^^^

// warning: `hello` (example "method") generated 2 warnings
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
//      Running `target\debug\examples\method.exe`
// success
```

