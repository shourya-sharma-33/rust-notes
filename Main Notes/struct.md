# Struct

1. compound type allowing to group togather value of diffrent types into a named data structure
2. similar to tuple but each value has a name so values can be accessed through this name
3. have to be insatnciated with data, think of it like the struct is the template for the instanced you create from it

## template
it aint right analogy but i see it as the schema

```
struct User {
    active: bool,
    username : String,
    email: String,
    sign_in_count: u64
}
```
## creating a user
```
fn main () {
    let user1 = User {
        active: 1,
        username : String::from("someusername123"),
        email : String::from("sgouryasharma@suagdsd"),
        sign_in_count: 1,
    };
}
```

## accessing and mutating

```
let mut user1 = User {
        active: 1,
        username : String::from("someusername123"),
        email : String::from("sgouryasharma@suagdsd"),
        sign_in_count: 1,
    };
```
now we can access and mutate single field of struct

```
user1.email = String::from("adgyisd");
```
## Fucntion returning structs

```
fn build_user (email : String, username: String) -> User {
    // here we are defining is default values
    User {
        active : true,
        username : username,
        email : email,
        sign_in_count : 1
    }
}
```
**function can instantiate and return struct**

now instead of username : username, we can have username only

## Struct Update Syntax

```
fn main () {
    let user1 = User{
        active : true,
        username : String::from("ashaduihasas123"),
        email: String::from("agsuadad.csd"),
        sign_in_count : 1
        };
}
```

```
let user2 = User {
    active : user1.active, // we take user1.active
    username : user1.username,
    email: String::from("asgaius.com"),
    sign_in_count : user1.sign_in_count
}
```
instead of using syntax above, if we wanna copy data from user1 to user2 except email
```
let user2 = User {
    email : String::from("adusdsd.sdhguysd"),
    ..user1
};
```

## Tuple Struct

1. Like normal struct but using tuple like syntax for defining their field
2. basically a name tuple
3. instantiated by parathesis instead of curly braces
4. accessed through point notation

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main () {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}
```
## Unit like struct

1. Struct without any field
2. used when working with traits (will be covered)
3. doest store any data

---------

```
struct Person {
    name : String,
    age : u8,
    hobby : String
}

fn main () {
    let age : u8 = 30;
    let p : Person = Person {
        name : String::from("shourya");
        age,
        hobby : String::from("being gay and homosexyal and liking men");
    };

    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
//      Running `target\debug\examples\tuple.exe`
// Success

```

## Unit Struct

unit struct dont have any field, it can be useful when you need to implement and on some type but dont have any data that you wanna store in the type itself

```

struct Unit;
trait SomeTrait {
    // some behavior defined here
}

impl SomeTrait for Unit { }
fn main () {
    let u = Unit;
    do_something_with_unit(u);
    println!("asasasas");
}

//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
//      Running `target\debug\examples\tuple.exe`
// Success
```
```

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main () {
    let v : Point = Point(0, 127, 255);
    check_color(v);
    println!("succss");
}
fn check_color(p: Point) {
    let (x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}    

// Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
//      Running `target\debug\examples\tuple.exe`
// Success


```

```
struct Person () {
    name : String,
    age : u8
}

fn main () {
    let age : u8 = 18;
    let mut p : Person = Person {
        name : String::from("shourya");
        age,
    };

    p.age = 30;

    p.name = String::from("sharma");
    println!("success");
}

//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
//      Running `target\debug\examples\tuple.exe`
// Success

```



```

struct Person {
    name : String,
    age : u8,
}
fn main() {
    println!("success");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
       age,
       name 
    }
}

// // we simply built a function that makes an struct
```

```

struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64
}

fn main () {
    let u1 : User = User {
        email : String::from("smaiasjai.ajias"),
        username : String::from("sasas"),
        active : true,
        sign_in_count : 1,
    };

    let u2 : User = set_email(u1);
    println!("sucess");
}

fn set_email(u : User) -> User {
    User  {
        email : String::from("contact@h.dg"),
        ..u
    }
}

// 1.68s
//      Running `target\debug\examples\struct.exe`
// sucess
```



```

#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}

fn main () {
    let scale : u32 = 2;
    let rect1 : Rectangle = Rectangle {
        width : 30 * scale,
        height : 50 
    };

    dbg!(&rect1);
    println!("{}", rect1);
}


// we cant print struct like that, either use Debug 
// or use {:?}
```

```

#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}

fn main () {
    let scale : u32 = 2;
    let rect1 : Rectangle = Rectangle {
        width : 30 * scale,
        height : 50 
    };

    dbg!(&rect1);
    println!("{:?}", rect1);
}

// s
//      Running `target\debug\examples\struct.exe`
// [examples\struct.rs:260:5] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }
// Rectangle { width: 60, height: 50 }
```

```

#[derive(Debug)]
struct File {
    name : String,
    data : String
}

fn main () {
    let f : File = File {
        name : String::from("readme.md"),
        data : "Rust By Practice".to_string()
    };

    let _name : String = f.name;

    println!("{}, {}, {:?}", _name, f.data, f);

}

//     |
// 285 |     let _name : String = f.name;
//     |                          ------ value partially moved here   
// 286 |
// 287 |     println!("{}, {}, {:?}", _name, f.data, f);
//     |                                             ^ value borrowed 
// here after partial move
//     |
```





```
// we will first see the given definition and then we will decode this definitin and see with examples

// struct

// struct is kinda like its own datatype, with multiple data points, it may contain data of heap instead of stack
// we can define a datatype and it will store value the way we have made the scema

// partial move
// within the destrructuring of a single variable, both by-move and by-refrence pattern binding can be used at the same time.
// doing this well result in a partial move of the variable, which means that parts of the variable
// will be moved while other parts stay, in such a case, thee parrent variable cannot be used afterwardws
// as a whole, however the parts that are only refrenced (and not moved) can still be used


fn main () {
    #[derive(Debug)]
    struct Person {
        name : String, // a string (not &str)
        age : Box<u8>, // stored in heap memory
        salary_in_lpa : u8 // this is stored in stack
    }

    let shourya : Person = Person {
        name : String::from("Shourya Sharma"),
        age : Box::new(20),
        salary_in_lpa : 100
    };

    let Person {name, ref age, salary_in_lpa} = shourya;

    // now we will check diffrent stuff and comment out the outcome to understand whats up
    //===========(test1)==================================
    // println!("{},{},{}, are name age and salary respectively", name, age, salary_in_lpa)

    //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
    //      Running `target\debug\examples\struct.exe`
    // Shourya Sharma,20,100, are name age and salary respectively

    //==========(test2)=============================
    // now sincee we are doing three diffrent things with the data
    // one we are refrencing and one we are changing ownership and one which isnt that type of data
    // now lets see after declaring the data we will wonder if we access the data
    //==============(testing with changing the ownership)==================
    
    // println!("{}", shourya.name)

    // an error came
    //     error[E0382]: borrow of moved value: `shourya.name`
    //   --> examples\struct.rs:45:20
    //    |
    // 29 |     let Person {name, ref age, salary_in_lpa} = shourya;
    //    |                 ---- value moved here
    // ...
    // 45 |     println!("{}", shourya.name)
    //    |                    ^^^^^^^^^^^^ value borrowed here after move
    //    |
    


    //=====================(lets test for the moved one)=====================

    // println!("{}", shourya.age)
    
    
    // // age is refrencing
    //       --> examples\struct.rs:29:32
    //    |
    // 29 |     let Person {name, ref age, salary_in_lpa} = shourya;
    //    |                                ^^^^^^^^^^^^^ help: try ignoring the field: `salary_in_lpa: _`

    // warning: `hello` (example "struct") generated 2 warnings
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
    //      Running `target\debug\examples\struct.exe`
    // 20

    //============================(now the salary one should work fine)
    // println!("{}", shourya.salary_in_lpa)   
    
    
    // }   warning: `hello` (example "struct") generated 3 warnings
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
    //      Running `target\debug\examples\struct.exe`
    // 100
}


```

```

#[derive(Debug)]
struct File {
    name : String,
    data : String
}

fn main () {
    let f : File = File {
        name : String::from("readme.md"),
        data : "Rust By Practice".to_string()
    };

    let _name : String = f.name.clone();

    println!("{}, {}, {:?}", _name, f.data, f);
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
//      Running `target\debug\examples\struct.exe`
// readme.md, Rust By Practice, File { name: "readme.md", data: "Rust 
// By Practice" }
```