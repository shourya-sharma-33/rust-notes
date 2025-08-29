# Array

1. Fixed size collection of elements of the same data type stored as coniguous block in stack memory
2. signature of array is `[T, Length]` which indicates that the length if fixed at compile time
3. array can nether grow nor shrink, they mus retain their size

```
fn init_arr(n : i32) {
    let arr = [1 ; n];
}
```

this will cause an error, because the compiler has no idea of the exact size of the array at compile time

```
fn main () {
    let arr : [i32 ; 5] = [1, 2, 3, 4, 5];

    arrest!(arr.len() == 5);
    println!("success");
}
```

```

fn main () {
    let arr : [i32 ; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);
    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.89s
//      Running `target\debug\examples\str.exe`
// success
```

```

fn main () {
    let arr0 = [1, 2, 3];
    let arr : [_; 3] = ['a', 'b', 'c'];
    // this underscore says compiler will pick its own type
    
    // arrays are stack allocated
    // char takes 4 bytes in rust : unicode char

    assert!(std::mem::size_of_val(&arr) == 4*3);

    println!("succeess :3")
}

// warning: `hello` (example "str") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
//      Running `target\debug\examples\str.exe`
// succeess :3
```

all elements in an array can be initialized to the same value at ones

```
fn main () {
    let list : [i32 ; 100] = [1 ; 100] ; // this [1 ; 100] means 100 items and all are 1

    assert!(list[0] == 1);
    assert!(list.len() == 100);
    println!("success");
}
```

all elements in an array must be of the ssame type :3

## Indexing starts at 0

```
fn main () {
    let arr: [char; 3] = ['a', 'b', 'c'];

    let ele = arr[0];
    assert!(ele == 'a');
    println!("success");
}
```

**`.get(index)`**

```
fn main () {
    let names : [String ; 2] = [String::from("Sunfei"), "Sunface".to_string()];

    let name0 = names.get(0).unwrap();

    let _name1 = &names[1];
    println!("ssuccess");
}
```

