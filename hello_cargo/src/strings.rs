// INFO
// There are 2 types of string:
//  - primitive str: unmutable fixed-length string somewhere in the memory
//  - string: growable, heap-allocated data structure 

pub fn run() {

    // String of type 1 (ungrowable)
    let hello = "Hello"; 

    // String of type 2
    let hello_2 = String::from("Hello");

    println!("{}, {}", hello, hello_2);

    // Get length 
    println!("Length: {}", hello.len());
    println!("Length: {}", hello_2.len());

    // It is possible to modify the type 2 string, but we need to define them mutable
    let mut hello_3 = String::from("Hello");

    // Push a char
    hello_3.push('w');
    println!("{}", hello_3);

    // Push a string 
    hello_3.push_str("ooooooo");
    println!("{}", hello_3);

    // Check if a string is empty
    println!("{}", hello.is_empty());

    // Check if contains a substring
    println!("Contains ooo? {}", hello_3.contains("ooo"));

    // Replace
    println!("Old string: {} New string: {}", hello_3, hello_3.replace("ooooo", "eeeee"));

    // Loop through string by whitespace
    for word in String::from("Hello World !").split_whitespace() {
        println!("{}", word);
    }

    // Assertion testing
    assert_eq!(hello, hello_2);
    assert_eq!(hello, hello_3);
    /* 
        The second assert will generate:
        
            thread 'main' panicked at 'assertion failed: `(left == right)`
              left: `"Hello"`,
             right: `"Hellowooooooo"`', src/strings.rs:47:5
            note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

     */
    
}
