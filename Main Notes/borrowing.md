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

```