// INFO
// Primitive types of Rust
//  - integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
//  - floats: f32, f64
//  - boolean: true, false
//  - characters (char)
//  - tuples
//  - arrays

pub fn run() {

    // Rust has to know the types of all th variables at compile time, however,
    // the compiler can infer what type we want to use based on the value and how we use it
    
    // Default is `i32`
    let x = 2;

    // Default is `f64`
    let y = 2.5;

    // Add explicit type
    let z: i64 = 41;

    // Find max for specific type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    // Boolean from an epressions
    let is_greater = x > z;
    println!("{} is greater than {}? {}", x, z, is_greater);

    // Char
    // A char has single quotes
    let a1 = 'a';
    println!("{}", a1);

    // An emoji is considered to be a single char
    let a2 = '\u{1F600}';
    println!("{}", a2);

}
