# Tuple

1. way to stored related pieces of information in aingle variable
2. collection of values of the diffrent types grouped togather as a single compund value (type composed of other types)
3. stored as a fixed size contigous block of memory on the stack
4. signature (T, T, T)


```
fn main  () {
    let _t0 : (u8, i16) = (0, -1);

    let _t1 : (u8, (i16, u32)) = (0, (-1, 1));
    let t : (u8, u16, i64, &str,String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("success");
}

//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target\debug\examples\tuple.exe`
// success
```
```


fn main () {
    let t : (&str, &str, &str) = ("i", "am", "shourya");
    assert_eq!(t.2, "shourya");
    println!("success");
}
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
//      Running `target\debug\examples\tuple.exe`
// success
```
too long tuples cant be printed
```
fn main () {
    let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    println!("too long tuple : {:?}",  too_long_tuple);
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
// error[E0277]: `({integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer})` doesn't implement `Debug`
//   --> examples\tuple.rs:30:40
//    |
// 30 |     println!("too long tuple : {:?}",  too_long_tuple);       
//    |                                ----    ^^^^^^^^^^^^^^ `({integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer})` cannot be formatted using `{:?}` because it doesn't implement `Debug`
//    |                                |
//    |                                required by this formatting parame
```
destructuring tuple with pattern

```
fn main () {
    let tup : (i32, f64, &str) = (1, 6.4, "hello");
    let (x, y, z) = tup;
    assert_eq!(x, 1);
    assert_eq!(y, 6.4);
    assert_eq!(z, "hello");
    println!("suuccess");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
//      Running `target\debug\examples\tuple.exe`
// suuccess
```

```

fn main () {
    let (x, y, z);

    (x, y, z) = (1, 2, 3);
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);
    println!("success")
}

// PS C:\Users\Dell\Downloads\hello> cargo run --example tuple
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
//      Running `target\debug\examples\tuple.exe`
// success
```


```

fn main () {
    let (x, y) = sum_multiply((2, 3));
    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success");
}

fn sum_multiply(nums:(i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1 )
}
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
//      Running `target\debug\examples\tuple.exe`
// Success
```