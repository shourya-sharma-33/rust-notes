## Statements and Expressions (SNE)

```rust
// sne means Statements and Expressions

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Now the Expression below is NOT followed by a semicolon
        // This means the value of this expression will be assigned to `y`
        // If I put a semicolon here, it would return nothing, and `y` would store nothing.
        x_cube + x_squared + x // NO SEMICOLON
    };

    println!("x is {}, y is {}", x, y);
}
````

### Output

```txt
x is 5, y is 155
```

```
