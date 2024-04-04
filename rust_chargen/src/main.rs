// Really basic Rust. Get rust via 'rustup', then create a new project with
//      cargo new chargen
//
// Then go into the chargen directory and edit src/main.rs as below.
//
// After that, run it to see if it works.
//      cargo run
//
// Then build it.
//      cargo build


// Imported via the Cargo.toml file
use rand::Rng;

// Returns an 8 bit unsigned integer.
// Like Ruby, the last successful command is the return value.
// Maybe give it a minimum roll option?
fn roll() -> u8 {
    rand::thread_rng().gen_range(1..7)
}

// 2d6 for us Traveller people. Make your own 3d6 for DnD.
// Needs a modifier.
fn two_d6() -> u8 {
    roll() + roll()
}

// The main function is where things happen.
fn main() {
    // This is the old code. It works, but I want to make it better.
    // New code will start at //**
    // 'mut' means i is mutable, otherwise it would not be.
    let mut i = 0;
    while i < 6 {
        i = i + 1;
        // The ! means print is a macro. Still learning about those.
        print!("{} ", two_d6());
    }
    // This is to add a newline after the results. If you're doing one line,
    // you can get a free newline with println!.
    //     println!("My cool character.")
    print!("\n")

    // ** This is the new code. Starting with pseudo-code to get things
    // in my head.
    // 
    // Build a structure (?) to hold all of the data.
    // - Name (string)
    // - Gender (string)
    // - Age (int, but converts to a string)
    // - UPP (6 ints that get turned into a hexidecimal string character)
    //
    // Assign a gender (M,F)
    //
    // Get a name from the database. 
    //
    // Assign the name to a string variable. 
    //
    // Generate a random age, say 14-65?
    //
    // Build the empty collection of six characters.
    //
    // Get the roll (above)
    //
    // Ensure the roll is within 2-15.
    //
    // Convert the roll to a Hexidecimal string character.
    //
    // Add the roll to the collection.
    //
    // Print the collection.
}

// Here's a test function. The "#[test]" is the declaration.
// It is called with:
//      cargo test
#[test]
fn test_roll() {
    let mut counter = 0;
    // a is a range, and the end of the range is not in the range.
    let a = 1..7;
    // Run this test 100 times. 
    while counter < 100 {
        counter = counter + 1;
        assert!(a.contains(&roll()));
    }

}


