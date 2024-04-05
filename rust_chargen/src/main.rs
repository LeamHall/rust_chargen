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
// use rand::prelude::SliceRandom;

// Returns an 8 bit unsigned integer.
// Like Ruby, the last successful command is the return value.
// Maybe give it a minimum roll option?
fn roll() -> i8 {
    rand::thread_rng().gen_range(1..7)
}

// 2d6 for us Traveller people. Make your own 3d6 for DnD.
// Needs a modifier.
fn two_d6() -> i8 {
    roll() + roll()
}

// Generate gender.
fn gen_gender() -> String {
    let options = vec!["m", "f"];
    let pick = rand::thread_rng().gen_range(0..2);
    return options[pick].to_string();
}

// Build the character.
fn build_character() -> Character {
    let _name = String::from("Fred");
    let _gender = gen_gender();
    let _age = 18;
    let _stats = gen_stats();
    let mut character = Character {
        name: _name,
        gender: _gender,
        age: _age,
        stats: _stats,
        upp: String::from(""),
    };
    return character;
}


// Generate the stats.
fn gen_stats() -> Vec<i8> {
    let mut stat_block: Vec<i8> = Vec::new();
    let mut i = 0;
    while i < 6 {
        // I wonder if there's a better way for a limited iteration?
        i = i + 1;
        stat_block.push(two_d6());
    };
    return stat_block;
}

// The main function is where things happen.
fn main() {
    // 'mut' means i is mutable, otherwise it would not be.
    // Starting with pseudo-code to get things in my head.
    // 
    // Build a structure (?) to hold all of the data.
    // - Name (string)
    // - Gender (string)
    // - Age (int, but converts to a string)
    // - stats (some ints)
    // - UPP (string calculated from stats)
    //
    //
    let character = build_character();

    println!("{} Age: {} Gender: {}  Stats: {:?}", 
        character.name, character.age, 
        character.gender, character.stats);
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

// The Character struct
struct Character {
    name:   String,
    gender: String,
    age:    i8,
    stats:  Vec<i8>,
    upp:    String,
}

// Here are the test functions. The "#[test]" is the declaration.
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

#[test]
fn test_gender() {
    let mut counter = 0;
    let mut a: Vec<String> = Vec::new();
    a.push("m".to_string());
    a.push("f".to_string());
    while counter < 100 {
        counter = counter + 1;
        assert!(a.contains(&gen_gender()));
    }
}

