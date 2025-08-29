# Slice

1. refrences to contiguos sequence of element in a collections
2. provide a way to borrow part of collection without taking ownership of the entire collection
3. can be created from arrays, vector, string and other collections implement the `Deref` trait


```
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

slice has the type &[i32] in this example
works like string slice do, by storing a refrence to the first element and a length

--------
here both [i32] and str are slice types, but directly using it will cause errors. you have to use the refrence of the slice instead: `&[i32]`  and `&str`


**❌ error**
```
fn main () {
    let arr : [i32, 3] = [1, 2, 3];

    let s1 : [i32] = arr[0..2];

    let s2 : str = "hello, world" as str;

    println!("successssssssssss");
}
```


**the right way ✅**

```
fn main () {
    let arr : [i32; 3]= [1, 2, 3];

    let s1 : &[i32] = &arr[0..2];

    let s2 : &str = "hello world";
    println!("success");
}
```

a slice refrence is a two-word object. for simplicty reasons. from now on we will use slice instead of slice refrence. the first world is a pointer to the data. and second word is the length of the slice. the word size is the same as usize, determined by the processor architecture e.g. 64 bits on a x86-64. slices can be used to borrow a section of an array, and have the type signature &[T].

```
fn main () {
    let arr: [char: 3] = ['क', 'ख', 'म']; 

    let slice = &arr[..2];

    assert!(std::mem::size_of_val(&slice) == 8);
    println!("success"); 
}

// assertion failed: std::mem::size_of_val(&slice) == 8
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// error: process didn't exit successfully: `target\debug\examples\str.exe` (exit code: 101)

```

now this is some low level shit
basically this code should run if `size_of_val` of slice ref is 8
logically it should cuz you can see that there are 2 elements and each char takes 4 bytes, issue is, it wont,it would show error

thats because the slice refrence is a two word refrence, meaning it take doubble the bits
meaning 16 should work

```


fn main () {
    let arr: [char; 3] = ['क', 'ख', 'म']; 

    let slice = &arr[..2];

    assert!(std::mem::size_of_val(&slice) == 16);
    println!("success"); 
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
//      Running `target\debug\examples\str.exe`
// success
```

