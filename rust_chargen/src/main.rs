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
//      rustfmt
//
// After that, run it to see if it works.
//      cargo run
//
// Then build it.
//      cargo build

// Imported via the Cargo.toml file
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

// Returns an 8 bit unsigned integer.
// Like Ruby, the last successful command is the return value.
fn roll() -> u8 {
    rand::thread_rng().gen_range(1..7)
}

// 2d6 for us Traveller people. Make your own 3d6 for DnD.
fn two_d6() -> u8 {
    roll() + roll()
}

// Generate gender.
fn gen_gender() -> String {
    let options = ["m", "f"];
    let mut rng = thread_rng();
    return options.choose(&mut rng).expect("Can't choose").to_string();
}

// Give a random age.
fn gen_age() -> u8 {
    rand::thread_rng().gen_range(13..72)
}

// Get a hexdigit string from a [u8;6] array.
fn gen_upp(stats: &[u8; 6]) -> String {
    let mut upp = "".to_string();
    for s in stats.iter() {
        upp += &format!("{:X}", s).to_string();
    }
    return upp;
}

// Take a gender string and return an appropriate name string.
fn gen_name(gender: &str) -> String {
    // Eventually this will hit the Names database.
    let f_name;
    if gender == "m" {
        f_name = "Fred";
    } else {
        f_name = "Wilma";
    }
    let l_name = "Flintstone";
    let name = format!("{} {}", f_name, l_name);
    return name;
}

// Build the character.
fn build_character() -> Character {
    let _gender = gen_gender();
    let _name = gen_name(&_gender);
    let _age = gen_age();
    let _stats = gen_stats();
    let _upp = gen_upp(&_stats);
    let character = Character {
        name: _name,
        gender: _gender,
        age: _age,
        stats: _stats,
        upp: _upp,
    };
    return character;
}

// Generate the stats.
fn gen_stats() -> [u8; 6] {
    let mut stat_block = [0u8; 6];
    let mut i = 0;
    while i < stat_block.len() {
        stat_block[i] = two_d6();
        i = i + 1;
    }
    return stat_block;
}

// Print the character per CT Supplement 4.
fn _print(character: &Character) {
    println!(
        "{} [{}] (age {}) [{}] ",
        character.name,
        character.gender.to_uppercase(),
        character.age,
        character.upp
    );
}

// The main function is where things happen.
fn main() {
    // 'mut' means i is mutable, otherwise it would not be.
    // When modifyin the character, put 'mut' (no quotes) between "let" and "character".
    let character = build_character();

    // Print the character.
    _print(&character);
}

// The Character struct
struct Character {
    // There will be a warning about field 'stats' never being read. Ignore it
    // since we'll eventually modify the stats.
    name: String,
    gender: String,
    age: u8,
    stats: [u8; 6],
    upp: String,
}

// Here are the test functions. The "#[test]" is the declaration.

#[test]
fn test_roll() {
    // a is a range, and the end of the range is not in the range.
    let a = 1..7;
    // Run this test 100 times to make sure it hits the edges.
    for _counter in 1..100 {
        assert!(a.contains(&roll()));
    }
}

#[test]
fn test_gender() {
    let mut a: Vec<String> = Vec::new();
    a.push("m".to_string());
    a.push("f".to_string());
    for _counter in 1..100 {
        assert!(a.contains(&gen_gender()));
    }
}

#[test]
fn test_mutable_character() {
    let mut names: Vec<String> = Vec::new();
    names.push("Fred Flintstone".to_string());
    names.push("Wilma Flintstone".to_string());
    let mut c = build_character();
    for _counter in 1..10 {
        assert!(names.contains(&c.name));
    }
}

#[test]
fn test_gen_upp() {
    let s = [7, 8, 9, 10, 11, 12];
    assert_eq!("789ABC".to_string(), gen_upp(&s));
}

#[test]
fn test_gen_name() {
    assert_eq!(gen_name(&"m"), "Fred Flintstone".to_string());
    assert_eq!(gen_name(&"f"), "Wilma Flintstone".to_string());
}
