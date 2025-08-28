# String vs &str

1. a `String` is a heap-allocated string type that owns its contents and is mutable
2. A &str is an immutable sequence of UTF-8 bytes in memory and doesnt own the underlying data and is immutable
3. think of &str as a view on a sequence of characters (Stored as UTF-8 bytes) in memory
4. use &str if u just want a view of a string
5. &str is more lightweight and efficient that String
6. use String if u needa own that data and be able to mutate it

# String Literal

1. A string literal is a sequece of characters enclosed in double quotes
2. fixed sizze, compile time  known sequence of UTF-8 bytes
3. the type is &static str which indicates the data is stored in static storage, meaning its valid throughout the entire lifetime of the programee
4. the data is hardcoded into the executable and stored in read only memory, meaning thy are immutable


# example string slice

```
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

the string slice `world` points to a sequrnce of characters stored on the heap


s -------------------->[h,e,l,l,o, ,w,o,r,l,d]
                               ^
                               |
world --------------------------

they both are pointing to this hello world stored on heap