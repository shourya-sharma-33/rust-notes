```
// we will first see the given definition and then we will decode this definitin and see with examples

// struct

// struct is kinda like its own datatype, with multiple data points, it may contain data of heap instead of stack
// we can define a datatype and it will store value the way we have made the scema

// partial move
// within the destrructuring of a single variable, both by-move and by-refrence pattern binding can be used at the same time.
// doing this well result in a partial move of the variable, which means that parts of the variable
// will be moved while other parts stay, in such a case, thee parrent variable cannot be used afterwardws
// as a whole, however the parts that are only refrenced (and not moved) can still be used


fn main () {
    #[derive(Debug)]
    struct Person {
        name : String, // a string (not &str)
        age : Box<u8>, // stored in heap memory
        salary_in_lpa : u8 // this is stored in stack
    }

    let shourya : Person = Person {
        name : String::from("Shourya Sharma"),
        age : Box::new(20),
        salary_in_lpa : 100
    };

    let Person {name, ref age, salary_in_lpa} = shourya;

    // now we will check diffrent stuff and comment out the outcome to understand whats up
    //===========(test1)==================================
    // println!("{},{},{}, are name age and salary respectively", name, age, salary_in_lpa)

    //        Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
    //      Running `target\debug\examples\struct.exe`
    // Shourya Sharma,20,100, are name age and salary respectively

    //==========(test2)=============================
    // now sincee we are doing three diffrent things with the data
    // one we are refrencing and one we are changing ownership and one which isnt that type of data
    // now lets see after declaring the data we will wonder if we access the data
    //==============(testing with changing the ownership)==================
    
    // println!("{}", shourya.name)

    // an error came
    //     error[E0382]: borrow of moved value: `shourya.name`
    //   --> examples\struct.rs:45:20
    //    |
    // 29 |     let Person {name, ref age, salary_in_lpa} = shourya;
    //    |                 ---- value moved here
    // ...
    // 45 |     println!("{}", shourya.name)
    //    |                    ^^^^^^^^^^^^ value borrowed here after move
    //    |
    


    //=====================(lets test for the moved one)=====================

    // println!("{}", shourya.age)
    
    
    // // age is refrencing
    //       --> examples\struct.rs:29:32
    //    |
    // 29 |     let Person {name, ref age, salary_in_lpa} = shourya;
    //    |                                ^^^^^^^^^^^^^ help: try ignoring the field: `salary_in_lpa: _`

    // warning: `hello` (example "struct") generated 2 warnings
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
    //      Running `target\debug\examples\struct.exe`
    // 20

    //============================(now the salary one should work fine)
    // println!("{}", shourya.salary_in_lpa)   
    
    
    // }   warning: `hello` (example "struct") generated 3 warnings
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
    //      Running `target\debug\examples\struct.exe`
    // 100
}


```