## Ranges in Rust

### Iterating Over Ranges
```rust
// For ranges, we need to understand a few concepts.
// Everything is a number in the end — even characters like a, b, c.
// When iterating from 'a' to 'z', remember that 'z' is also represented as an ASCII value.
// In Rust, ranges are exclusive of the upper bound by default unless specified otherwise.

fn main() {
    let mut sum: i32 = 0; // Final answer will be -5

    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);
    // Code ran because the sum came out to be -5.
    // By default, ranges exclude the last value.
    // To include the last value, we can use '..=' instead.

    for j in 'a'..='z' {
        println!("{}", j);
    }
}
````

---

### Using `Range` and `RangeInclusive` from the Standard Library

```rust
use std::ops::{Range, RangeInclusive};
// We're importing the Range types from the standard library.
// Although rarely used directly, this shows how Rust internally handles ranges.

fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("successsss :3");
}
```

### Output

```txt
successsss :3
```

```
