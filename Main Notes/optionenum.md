# the option enum

1. Option is an enum that represents a value that may or may not be present
2. known in other languages as null, refering to the absence of a value
3. used to handle cases where a function or methof fail to return a value

```
enum Option<T> {
    None,
    Some(T)
}

fn main () {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // if none return none
        Some(i) => Some(i + 1),
        // if some with a value return a one added
    }
}
```

plus_one() expecrts an argument of type Option so we have to wrap an i32 inside some()

x get matched and if there is a some() value it gets incremented by one

basically

`Option<i32>` says either there is some value of i32 type or nothing