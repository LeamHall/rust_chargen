// Here are the test functions. The "#[test]" is the declaration.
//
//

use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rust_chargen").unwrap();
    cmd.assert().success();
}

#[test]
fn prints_some_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust_chargen")?;
    let output = cmd.output()?;

    let output_str = String::from_utf8(output.stdout)?;

    assert!(output_str.contains(' '), "Output does not contain a space.");

    let words: Vec<&str> = output_str.split_whitespace().collect();
    let long_words: Vec<&str> = words.into_iter().filter(|word| word.len() > 3).collect();

    assert!(
        long_words.len() >= 2,
        "Output does not contain at least two words longer than 2 characters."
    );

    Ok(())
}
