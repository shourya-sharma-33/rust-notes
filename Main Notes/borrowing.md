# Borrowing

1. Way of temporarily access data without taking ownership of it
2. when borrowing you're taking a refrence (pointer) to the data, not the data itself
3. prevention of dangling pointers and dataraces
4. data can be borrowed immutable and mutably
5. there are certain rules when borrowing which we have to comply with, otherwise the program wont compile

## Rules of references

1. at any given time, yes can have either one mutable refrence or any number of immutable refrences
2. refrences must always be valid

```
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the lenght of '{}' is {}", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

`&` means `refrencing to` so here thats why we can reuse `s1` because we are refrencing to the value
when a refrenced variable is passed in a function
it too has to be put in the datatype
like in fn calculate_length we added s: &String

s

|name|value|
|--------|-----------|
|ptr|**address**|

s1

|name|value|
|--------|-----------|
|ptr|**address**|
|len|5|
|capacity|5|



|index|value|
|--------|-----------|
|0|h|
|1|e|
|2|l|
|3|l|
|4|0|

**now lets talk about mutables**

mutable refrence can only be a single one

```
fn main () {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```


## Correct Borrowing Methods (❌✅)

❌

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{},{}", r1, r2);
```

this would violate the first rule of borrowing, which says that we can only have ONE mutable refrence to the same data at a time


✅

```
let mut s = String::from("hello");

{
    let r1 = &mut s;
}

let r2 = &mut s;

```

here both refrence are in diffrent scope so its good :3


------

❌

```

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // Issue

println!("{},{},{}", r1, r2, r3);

```

✅

```

let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
print("{}, {}", r1, r2);
//now we just have to make sure after declaration r3 refrence, r1 and r2 arent used

let r3 = &mut s; //no problem
println!("{}", r3);
```

-----

❌

```
fn main () {
    let refrence_to_nothing = dangle();
}
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```
here we are returning a refrence, and that is out of scope, so we are refrencing to something out of scope so bad practice


-------------

```
fn main () {
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, *y);
    println!("sccess");
}

// PS C:\Users\Dell\Downloads\hello> cargo run --example borrowing
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
//      Running `target\debug\examples\borrowing.exe`
// sccess
// PS C:\Users\Dell\Downloads\hello> 


```

```

// how to know memory adress

fn main () {
    let x : i32 = 5;
    let p : &i32 = &x;

    println!("the memory adress of x is {:p}",p);
}

// sccess
// PS C:\Users\Dell\Downloads\hello> cargo run --example borrowing
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
//      Running `target\debug\examples\borrowing.exe`
// the memory adress of x is 0x329a9ffa8c
```

----------

## `ref` and `&` are same

```

fn main () {
    let c : char = 'अ';
    let r1 : &char = &c;
    let ref r2 = c;

    // both are same
    // r1 where we use &
    // r2 where we use ref

    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));
    // we check if memory address is same
    println!("success :3");   
}
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
```
 ## we cant pass an mutable refrence to something immutable


❌ an error will return 
```
fn main () {
    let s : String = String::from("hello, ");
    borrow_object(&mut s);
    println!("Success:3");
}
fn borrow_object(s : &mut String) {

}
```
✅ here we are intialising s as mutable 
```
fn main () {
    let mut s : String = String::from("hello, ");
    borrow_object(&mut s);
    println!("Success:3");
}
fn borrow_object(s : &mut String) {

}
```
## we can borrow an immutable object as mutable

```
fn main () {
    let mut s : String = String::from("hello, ");
    borrow_object(&s);
    s.push_str("world");
    println!("success");
}
fn borrow_object(s : &String) {}
```
