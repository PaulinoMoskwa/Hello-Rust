// Define public function run()
pub fn run() {

    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting - Print with placeholders
    // The format of the placeholder is the same for whatever type 
    println!("Number: {} \nString: {}", 1, "example_of_string");

    // Positional arguments
    // The following are equivalent, the 2nd is more efficient
    println!("This {} is {}. What is this {} name?", "cat", "cool", "cat");
    println!("This {0} is {1}. What is this {0} name?", "cat", "cool");

    // Named arguments
    println!("{name} likes to play {activity}", name="John", activity="baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10, 10, 10);
    // Output > Binary: 1010 Hex: a Octo: 12

    // Placeholder for debug trait
    // We can put multiple values
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

}
