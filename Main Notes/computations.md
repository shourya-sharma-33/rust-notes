## Basic computations in Rust
```rust
// computations in Rust are like any other language, except
// we can also specify precision using type suffixes

// fn main () {
//     assert!(1u32 + 2u32 == 3u32);
//     println!("success");
// }
````

```
cargo run --example computations
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
Running `target\debug\examples\computations.exe`
success
```

---

## Unsigned integer underflow error

```rust
// let's see an error case

fn main () {
    println!("{}", 1u32 - 2u32);
}
```

### Error message

```
error: attempt to compute `1_u32 - 2_u32`, which would overflow
  |
18 |     println!("{}", 1u32 - 2u32);
  |                    ^^^^^^^^^^^
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
```

---

## Floating-point precision issue

```rust
fn main () {
    assert!(9.6 / 3.2 == 3.0);
    println!("success");
}
```

This won't compile as expected because the default floating-point type is `f64`,
which can produce very small precision errors during boolean comparisons.
For example, `1.0 + 2.0` may internally evaluate to `3.0000000002`.

---

## Fixing precision with `f32`

```rust
fn main () {
    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);
    println!("success");
}
```

```
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
PS C:\Users\Dell\Downloads\hello> cargo run --example computations
Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
Running `target\debug\examples\computations.exe`
success
```

```
