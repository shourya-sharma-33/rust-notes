## OwnerShip

1. Rust's ownership sysetem is unique and sets it apart from other programming language

2. set of rules that govern memory management

3. rules are enforces at compile time

4. if any of the rules are violated, the program wont compile

### Three Rules of OwnerShip

1. Each value in rust has on owner
2. There can only be one owner at a time
3. when the owner goes out of the scope, the value will be dropped

**owner:** the owner of the value is the varible or data structure that holds it and is responsible for allocating and freeing the memory used to store that data

### Scope 

Rangee within a program for which an itm is valid
1. **Global Scope** : Accessible throut the entire programme
2. **Local Scope** : 
accesible only within perticular functipn or block of code
not assesible outside of that function or block 

```
{
    let name = "shourya";

    // we can only do stuff within this block
    // start and end is curly braces
}
```

### Memory

1. Component in a computer to store data and instructions for the processor to execute
2. Random Access Memory (RAM) is volatile, when power turned off the contents are lost.
3. two types of regions in RAM used by programm at runtime: **Stack Memory** and **Heap Memory**

## Stack Memory 
1. Last in, first out
2. All Data stored on the stack must have known, fixed size (like integers, floats, char, bool, etc...)
3. Pushing to the stack is faster than allocating on the heap, because the location for new data is always at the top of the stack

4. types of unknown size will get allocated to the heap and a pointer to the value is pushed to the stack, because a pointer is fixed size (usize)

```
fn main () {
    let x = 42;
    let y = 10;
    leet z = add_number(x, y);
    println!("the result is {}",z);
}
fn add_number(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}
```

the stack memory will store this function like thiss

```
_______________________

add_numbers() function
a, b, c
_______________________

main() function
x, y, z
_______________________

```

## Heap Memory

1. Data of no known, fixed size belongs on the heap
2. allocating data of thee heap will return a pointer (an address to location where data has data has been allocated)
3. allocating on the heap is slower than pushing to stack
4. accessing data on the heap is also slower,as it has to be accessed using a pointer which points to an adress

basically there will be an adress assigned

## The String Type

1. All type covered so far were fixed size
2. Strinf is mutable
3. String size can change at run time
4. String Stored on the stack with a pointer to the heap
5. value of string us stored on the heap

```let s1 = String::from("hello");```

s1 --> This will contain the metadata and adress to the data

| name  | value |
|--------|-----------|
| ptr|**address**|
|len|5|
|capacity|5|

size of s1 will be 24 bytes 3 * 8 bytes (usize)


|index|value|
|--------|-----------|
|0|h|
|1|e|
|2|l|
|3|l|
|4|o|

1. ptr : Pointer to data stored on the heap
2. len : data size in bytes
3. capacity : total amount of memory recieved from the allocator

 ## Copy vs Move

 1. Scalar values with fixed sizes (all types we covered at the beginning) will automatically get copied in the stack, copying here is cheap
 2. dynamically sized data wont't get copied, but moved, copying would be too expesive :3


 ```
 let x = 5;
 let y = x;
 ```

 here, the integer value of variable x will get copied into y and both variable are usable, because i32 value has been copied -> i32 is fixed size

 ```
 let s1 = String::from("hello");
 let s2 = s1;
 ```

 as s1 is just a pointer to data on the heap just the pointer will get copied into s2,not the wholee data on heap

 ```
 let s1 = String::from("hello");
 let s2 = s1;
 ```

 s1 and s2 point to the same location in heap memory

 this would violate the second rule which says that there can only be ONE owner at a time

`s1` 
| name  | value |
|--------|-----------|
| ptr|**address**|
|len|5|
|capacity|5|

`s2` 
| name  | value |
|--------|-----------|
| ptr|**address**|
|len|5|
|capacity|5|

address ----> this adress is pointing to the adress below


|index|value|
|--------|-----------|
|0|h|
|1|e|
|2|l|
|3|l|
|4|o|

this should not happen :3


so variable s1 will be dropped and cannot be used after assigning it to s2, to avoid dangling pointer


## Deep Copy

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```


------------

# Ownership

now its time we understand how ownership exactly works

we will look at a code and see what happens in it

```
1  | fn main () {
2  |     let s = String::from("hello");
3  |     takes_ownership(s);
4  | 
5  |     let x = 5;
6  |     makes_copy(x);
7  | }
8  | 
9  | fn takes_ownership(some_string : String) {
10 |     println!("{}", some_string);
11 | } 
12 | 
13 | fn makes_copy(some_integer: i32) {
14 |     println!("{}", same_integer);
15 | }

```
 **now we will understand each code line by line**

 ```
2  |     let s = String::from("hello");
3  |     takes_ownership(s);
 ```

 `String::from("hello")` is a heap Datatype
 here `s` is the owner of `String::from("hello")`
 and this ownership can be passed down
 here it is passed down to takes_ownership

 while u can see that both take_ownership and make_copy are same function
 they both are treating data diffrent ways
 `s` cant be reused after ownership transfered

 here since `x` is a stack datatype, here the ownership doesnt pass, just the copy is made

```
fn main () {
    let s1 = gives_ownership();
    // gives_ownership moves its return
    // value into s1
    // {go look at the gives ownership function}

    let s2 = String::from("hello");
    // s2 comes into scope

    let s3 = takes_and_gives_back(s2);
    // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into x3
} 

// here, s3 goes out of scope and is dropped, s2 was moved, so nothing
// happens, s1 goes out of scope and is dropped

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// gives_ownership will move its return value into the function that calls it
// some_string comes into scope
// some_string is return and moves out to the calling function

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
// a_string comes into scope
// a_string is returned and moves out to the calling function

```

## Preventing Issues

1. Ownership prevents memory safety issues

1. dangling pointers
2. double free (trying to free memory that has already been freed)
3. memory leaks (not freeing memory that should have been freed)


```
// if u are in this file first
// go read the ownership.md it contains all the explaination
// to whatever we will be doing here so without that u wont be getting anything


fn main() {
    let x : String =  String::from("hello");
    let y : String = x;

    println!("{},{}",x,y);
}

// here x should not print since ownership transfer is taking place
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0382]: borrow of moved value: `x`
//   --> examples\ownership.rs:10:22
//    |
// 7  |     let x : String =  String::from("hello");
//    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
// 8  |     let y : String = x;
//    |                      - value moved here
// 9  |
// 10 |     println!("{},{}",x,y);
//    |                      ^ value borrowed here after move
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//    |
// 8  |     let y : String = x.clone();
//    |                       ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `hello` (example "ownership") due to 1 previous error

```

```

 fn main () {
     let x : String = String::from("Hello");
     // now we will make a deep copy
     let y : String = x.clone();

     println!("{},{}",x,y);
 }

// now there a copy was made
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.68s
//      Running `target\debug\examples\ownership.exe`
// Hello,Hello

```

```

fn main () {
    let s1 : String = String::from("hello");
    let s2 : String = take_ownership(s1);
    println!("{}", s2);
}

fn take_ownership(s : String) -> String {
    println!("{}", s);
    s
}

// ownership from s1 --> take_ownership() --> s2
// 

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
//      Running `target\debug\examples\ownership.exe`
// hello
// hello
```


```
// into_bytes(self)
// converts string to byte vectors
// this consumes the string, so we actually cannot reuse the old mutable

fn main () {
    let s = String::from("hello");
    let bytes = s.into_bytes();

    assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
    println!("success");

}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
//      Running `target\debug\examples\ownership.exe`
// success

```

```

// if we dont want our variable to get consumed what we can do is we can simply 
// use as_bytes
fn main () {
    let s = String::from("hello");
    let bytes = s.as_bytes();

    assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
    println!("success and {} can be reused", s);

}


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
//      Running `target\debug\examples\ownership.exe`
// success and hello can be reused
```



```

// now we will see more cool stuff with ownership and 
// solve some doubts that can appear with it

fn main (){
    let s : String = String::from("hello");

    print_str(s);
    println!("{}", s);
    
}

fn print_str(s : String) {
    println!("{}", s)
}


// now this should show an error cuz the owner ship was passed to print_str function

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0382]: borrow of moved value: `s`
//    --> examples\ownership.rs:119:20
//     |
// 116 |     let s : String = String::from("hello");
//     |         - move occurs because `s` has type `String`, which 
// does not implement the `Copy` trait
// 117 |
// 118 |     print_str(s);
//     |               - value moved here
// 119 |     println!("{}", s);
//     |                    ^ value borrowed here after move        
//     |
// note: consider changing this parameter type in function `print_str` to borrow instead if owning the value isn't necessary
//    --> examples\ownership.rs:123:18
//     |
// 123 | fn print_str(s : String) {
//     |    ---------     ^^^^^^ this parameter takes ownership of the value
//     |    |
//     |    in this function
//     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in 
// Nightly builds, run with -Z macro-backtrace for more info)       
// help: consider cloning the value if the performance cost is acceptable
//     |
// 118 |     print_str(s.clone());
//     |                ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `hello` (example "ownership") due to 1 previous error




```
```

// lets check the mutabilituy of String ddatatype

// fn main () {
//     let s : String = String::from("hello");
//     s.push_str("world");
//     println!("success!")
// }


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
//    --> examples\ownership.rs:167:5
//     |
// 167 |     s.push_str("world");
//     |     ^ cannot borrow as mutable
//     |
// help: consider changing this to be mutable
//     |
// 166 |     let mut s : String = String::from("hello");
//     |         +++

// For more information about this error, try `rustc --explain E0596`.
// error: could not compile `hello` (example "ownership") due to 1 previous error

// now lets try transfering the owner ship to a mutable variable

fn main () {
    let s : String = String::from("hello");
    let mut s1 = s;
    s1.push_str("world");
    println!("success!")
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.74s
//      Running `target\debug\examples\ownership.exe`
// success!

```
```


fn main () {
    let t : (String, String) = (String::from("hello"), String::from("world"));

    let _s : String = t.0;
    // access the 0th index 

    println!("{}", _s);
    println!("{}", t.1);
    // no issue
    //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
    //      Running `target\debug\examples\ownership.exe`
    // hello
    // world

    println!("{}", t.0);
    // this would show moved
    //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
    // error[E0382]: borrow of moved value: `t.0`
    //    --> examples\ownership.rs:221:20
    //     |
    // 209 |     let _s : String = t.0;
    //     |                       --- value moved here
    // ...
    // 221 |     println!("{}", t.0);
    //     |                    ^^^ value borrowed here after move
    //     |

}

// now lets do it with .clone method

```

```

// // now lets do it with .clone method



fn main () {
    let t: (String, String) = (String::from("hello"), String::from("world"));
    let (s1, s2) = t.clone();
    println!("{:?}, {:?}, {:?}",s1,s2,t)
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
//      Running `target\debug\examples\ownership.exe`
// "hello", "world", ("hello", "world")
```