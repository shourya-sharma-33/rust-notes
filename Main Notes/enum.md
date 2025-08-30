# Enum

1. way of defining a type with only one of a possible st of values
2. we can only access one variant of an enum at a time
3. can hold additional information using tuples
4. espetially useful when using match statements

```
enum IpAddr {
    V4(String),
    V6(String)
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Enum for IP address. an ip address can either be of v4 format or v6 format, each variabt in the enum holds a string