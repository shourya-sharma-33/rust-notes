// refer to the image notes where i have explained it allow


fn main () {
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1<<5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

// PS C:\Users\Dell\Downloads\hello> cargo run --example bitwiseoperations
//    Compiling hello v0.1.0 (C:\Users\Dell\Downloads\hello)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
//      Running `target\debug\examples\bitwiseoperations.exe`
// 0011 AND 0101 is 0001
// 0011 OR 0101 is 0111 
// 0011 XOR 0101 is 0110
// 1<<5 is 32
// 0x80 >> 2 is 0x20    