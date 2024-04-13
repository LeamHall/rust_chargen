// Here is an example of really basic Rust. You can modify this for your own needs.
//
// Get rust:
//      https://www.rust-lang.org/learn/get-started
//
// Create a new project with
//      cargo new chargen
//
// Go into the chargen directory.
//
// Add the following line under the "[dependencies]" section in Cargo.toml:
//      rand = "0.8.5"
//
// Edit src/main.rs as below.
//
// Run the tests. Since testing also compiles, this gives a lot of feedback.
//      cargo test
//
// Once the tests pass, format the code to make it Rust standard.
//      cargo fmt
//
// After that, run it to see if it works.
//      cargo run
//
// Then build it.
//      cargo build

// The main function is where things happen.
fn main() {
    // 'mut' means i is mutable, otherwise it would not be.
    // When modifying the character, put 'mut' (no quotes) between "let" and "character".
    let character = rust_chargen::build_character();

    // Print the character.
    println!("{}", character.show());
}
