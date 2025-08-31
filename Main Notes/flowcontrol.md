# Flow Control

1. Normal flow of a progrramme : top to bottom, line by line
2. concept that refers to ability to control the order in which statemenrs or instructions are executed in a programm
3. allows to specify which instruction should be executed under which conditions and in what order

## Conditionals
1. if/else
2. match


## Loops
1. for/which/loop
2. continue/break

```
fn main () {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
//      Running `target\debug\examples\flowcontrol.exe`
// 5 is positive
```


```

fn main () {
    let n : i32 = 5;
    let big_n : i32 =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2.0 as i32
        };
    println!("{} -> {}", n, big_n);
}


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
//      Running `target\debug\examples\flowcontrol.exe`
// , and is a small number, increase ten fold
// 5 -> 50
```

```

fn main () {
    let names : [String ; 2] = [String::from("shourya"), String::from("sharma")];
    for name in &names {
        println!("{}", name);
    }
    println!("{:?}", names);

    let numbers : [i32; 3] = [1, 2, 3];
    for n in numbers {
        println!("{}", n);
    }

    println!("{:?}", numbers);
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.98s
//      Running `target\debug\examples\flowcontrol.exe`
// shourya
// sharma
// ["shourya", "sharma"]
// 1
// 2
// 3
// [1, 2, 3]
```

```

fn main () {
    let a : [i32; 4] = [4, 3, 2, 1];

    for (i, v) in a.iter().enumerate() {
        println!("the {}the element is {}", i+1, v);
    }
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.81s
//      Running `target\debug\examples\flowcontrol.exe`
// the 1the element is 4
// the 2the element is 3
// the 3the element is 2
// the 4the element is 1
```

```

fn main () {
    let mut n : i32 = 1;
    while n < 10 {
        if n % 15 == 0 {
            println!("shourya");
        } else if n % 3 == 0 {
            println!("sharma");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

// 1
// 2
// sharma
// 4
// buzz
// sharma
// 7
// 8
// sharma
```

```

fn main () {
    let mut n : i32 = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n , 66);
    println!("success")
}
// warning: `hello` (example "flowcontrol") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
//      Running `target\debug\examples\flowcontrol.exe`
// success
```

## continue to skip the rest of code in block


```

fn main () {
    let mut n = 0;
    for i in 0..=10 {
        if n != 6 {
            n += 1;
            println!("{}", n);
            continue;
        }
        break;
    }
    assert_eq!(n, 6);
    println!("success");
}

// warning: `hello` (example "flowcontrol") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
//      Running `target\debug\examples\flowcontrol.exe`
// 1
// 2
// 3
// 4
// 5
// 6
// success
```

## loop is an expression, we can use break and contine
so we will have a value outside and then run a loop with a condition and a break statement somewherre

```

fn main () {
    let mut count : u32 = 0u32;

    println!("let count until infinitly");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
            // iske neeche wala sab skip
        }
        println!("{}", count);

        if count == 5 {
            println!("ok thhat nuff");
            break;
        }

    }
    assert_eq!(count, 5);
    println!("success");
}

//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
//      Running `target\debug\examples\flowcontrol.exe`
// let count until infinitly
// 1
// 2
// three
// 4
// 5
// ok thhat nuff
// success

```

```

fn main () {
    let mut counter : i32 = 0;

    let result : i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
    println!("success");
}


//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target\debug\examples\flowcontrol.exe`
// success
```

```

fn main () {
    let mut count = 0;

    'outer : loop {
        'inner : loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }
        count += 5;

        'inner2 : loop {
            if count >= 30 {
                break 'outer;
            }   
            continue 'outer;
        }
    }
    assert_eq!(count == 30);
    println!("success");
}
```