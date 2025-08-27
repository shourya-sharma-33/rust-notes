
## Variable Basics
1. Assigned using `let` keyword  
2. Print to standard output by `print!()` or `println!()`  
3. Scope of a variable is defined by the block of code in which it is declared  
4. Function is a named block of code that is reusable  
5. Shadowing allows a variable to be redeclared in the same scope with the same name

```rust
fn main() {
    let x: i32; // this i32 is a type annotation, will be talked about later
    let y: i32;

    assert_eq!(x, 5); // assert_eq will check if equal
    println!("success :3");
}
````

This code won't compile.

### Error Message

```txt
warning: unused variable: `y`
 --> examples\variables.rs:9:9
  |
9 |     let y : i32;
  |         ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` on by default

error: binding declared here but left uninitialized
 --> examples\variables.rs:8:5
  |
8 | let x : i32;
  |     - binding declared here but left uninitialized
...
11 | assert_eq!(x,5);
   | ^^^^^^^^^^^^^^^ `x` used here but it isn't initialized
```

---

## Variable Initialization

```rust
fn main() {
    let x: i32 = 5; // this i32 is a type annotation, will be talked about later
    let y: i32;

    assert_eq!(x, 5); // assert_eq will check if equal
    println!("success :3");
}
```

The code ran but only gave a warning for `y`.

### Warning Message

```txt
warning: unused variable: `y`
 --> examples\variables.rs:9:9
  |
9 |     let y : i32;
  |         ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `hello` (example "variables") generated 1 warning
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
Running `target\debug\examples\variables.exe`
success :3
```

---

## Mutable Variables

```rust
fn main() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("success :3")
}
```

### Output

```txt
success :3
```

---

## Basic Printing

```rust
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    println!("{} twos are {}", y, x);
}
```

### Output

```txt
5 twos are 10
```

---

## Scope Example

Scope is a range within the program for which the item is valid.

```rust
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 20;
        println!("{} twos are {}", x, y);
    }

    println!("{} twos are {}", x, y);
}
```

This code won't compile.

### Error Message

```txt
error[E0425]: cannot find value `y` in this scope
 --> examples\variables.rs:80:34
  |
80 |     println!("{} twos are {}", x, y);
   |                                  ^ help: a local variable with a similar name exists: `x`

For more information about this error, try `rustc --explain E0425`.
error: could not compile `hello` (example "variables") due to 1 previous error
```

---

## Undefined Variable Example

```rust
fn main() {
    println!("{}, world", x)
}

fn define_x() {
    let x: &str = "hello";
}
```

This code won't compile.

### Error Message

```txt
error[E0425]: cannot find value `x` in this scope
 --> examples\variables.rs:98:26
  |
98 |     println!("{}, world", x)
   |                          ^ not found in this scope

warning: unused variable: `x`
 --> examples\variables.rs:102:9
  |
102 |     let x : &str = "hello";
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default
```

---

## Functions and Main Scope

```rust
fn main() {}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}
```

It compiles but shows **no output**.
Functions must be called inside `main()` to produce output.

### Warning Message

```txt
warning: function `define_x` is never used
 --> examples\variables.rs:133:4
  |
133 | fn define_x() {
  |    ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default
```

---

## Calling Functions

```rust
fn main() {
    define_x();
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x)
}
```

### Output

```txt
hello, world
```

---

## Shadowing

Shadowing lets you declare a new variable with the **same name** as a previous variable.

```rust
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
        println!("we are in the inner scope and it compiling cuz assert_eq ran");
    }

    assert_eq!(x, 5);
    println!("here its working cuz in outter scope the valye is same");

    let x: i32 = 42;
    println!("{}", x);
}
```

### Output

```txt
we are in the inner scope and it compiling cuz assert_eq ran
here its working cuz in outter scope the valye is same
42
```

---

## Shadowing + Rebinding

```rust
fn main() {
    let mut x: i32 = 1;
    // by default every variable is immutable
    x = 7;
    // reassigning x
    let mut x = x; // redeclaring variable — shadowing + rebinding

    let y: i32 = 4;
    let y: &str = "mrrp meow :3333";
    println!("{} success :3", y);
}
```

The code works but shows multiple warnings.

### Warnings

```txt
warning: value assigned to `x` is never read
warning: unused variable: `x`
warning: unused variable: `y`
warning: variable does not need to be mutable
```

---

## Allowing Unused Variables

```rust
#[allow(unused_variables)]
fn main() {
    let x = 1;
}
```

This compiles with **no warnings**.

---

## Destructuring Variables

```rust
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("successss {},{}", x, y);
}
```

### Output

```txt
successss 3,2
```

---

## More on Destructuring

```rust
fn main() {
    let (x, y);

    (x, ..) = (3, 4); // first value assigned, rest ignored
    [.., y] = [3, 2];

    assert_eq!([x, y], [3, 2]);
    println!("success");
}
```

### Output

```txt
success
```

```
