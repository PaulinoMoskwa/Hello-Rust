//module (`mod`) and name of the file (print.rs)
mod print;
mod vars;
mod types;
mod strings;
mod tuples;

fn main() {

    // Hello world
    println!("Hello, world!");

    // Call the function run() from print.rs
    print::run();
    
    vars::run();

    types::run();

    strings::run();

    tuples::run();
}
