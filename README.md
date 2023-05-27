# Hello Rust 🦀
[![](https://img.shields.io/badge/Linux-Ubuntu%2022.04-blue)]()
[![](https://img.shields.io/badge/rustup-1.26.0-blue)]()
[![](https://img.shields.io/badge/rustc-1.69.0-blue)]()
[![](https://img.shields.io/badge/cargo-1.69.0-blue)]()

<p align="center">
<img src='./media/rustacean-banner.png' width='1000'/>
</p>


## Install Rust
If Linux, install [Rust](https://www.rust-lang.org/tools/install) by running in terminal:
```terminal
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Restart the terminal and check that the command:
```terminal
rustup --version
```
produces an output like:
```terminal
rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.69.0 (84c898d65 2023-04-16)`
```

Check rust compiler version:
```terminal
rustc --version
```

Check rust package manager version:
```terminal
cargo --version
```

Finally, move to VSCode and install the rust extension (now called `rust-analyzer`).


## "Hello world!" with Rust
Start a VSCode terminal and create a new file with the command:
```terminal
touch hello.rs
```

Then, the code for the simplest "Hello world!" in Rust is the following:
```rust
fn main() {
    println!("Hello world!");
}
```

To compile it and run it, go back to the terminal and execute:
```terminal
rustc hello.rs
```

This will create an executable. <br>
Then, run the application from the command line:
```terminal
./hello
```

## Hello ✨ Cargo ✨
Although it works, the above procedure is not the standard one used to handle projects in Rust.<br>
What is done instead, is to use Cargo.

Initialize the project in Cargo:
```terminal
cargo new hello_cargo
```
This will initialize a cargo project in a newly created folder called `hello_cargo`.

If, instead, we want to initialize the cargo project exactly where we are, it is enought to execute:
```terminal
cargo init
``` 

We move now to the newly created folder:
```terminal
cd hello_cargo/
```

It is possible to compile the `src/main.rs` with the command:
```terminal
cargo run 
```

The executable will be automatically created in a `target` folder (ignored by git).<br>
Notice, if we just want to build without run it, then we can type:
```terminal
cargo build
```

## Deepening (ongoing)
In the folder `src` we can now create a file for every topic we want.

- `print.rs` 
- `vars.rs`
- `types.rs`
- `strings.rs`
- `tuples.rs`