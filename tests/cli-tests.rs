use assert_cmd::Command;
use chrono::Utc;
use predicates::prelude::*;
use std::fs;

#[test]
fn given_stdin_no_args_expect_encoded_string_in_stdout() {
    Command::cargo_bin("rbase64")
        .unwrap()
        .write_stdin("%^&*()_<>,.?;[]{} 1234567890 abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .assert()
        .success()
        .stdout("JV4mKigpXzw+LC4/O1tde30gMTIzNDU2Nzg5MCBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiBBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWg==");
}

#[test]
fn given_stdin_and_decode_arg_expect_decoded_string_in_stdout() {
    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("--decode")
        .write_stdin("JV4mKigpXzw+LC4/O1tde30gMTIzNDU2Nzg5MCBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiBBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWg==")
        .assert()
        .success()
        .stdout("%^&*()_<>,.?;[]{} 1234567890 abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ");
}

#[test]
fn given_invalid_stdin_and_decode_arg_expect_error() {
    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("--decode")
        .write_stdin("!Not a base64 string")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Unable to decode non-base64 character '!'",
        ));
}

#[test]
fn given_text_file_input_arg_expect_encoded_string_in_stdout() {
    let expected = fs::read_to_string("./tests/resources/utf8.b64").unwrap();

    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("--input")
        .arg("./tests/resources/utf8.txt")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn given_positional_arg_expect_encoded_string_in_stdout() {
    let expected = fs::read_to_string("./tests/resources/utf8.b64").unwrap();

    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("./tests/resources/utf8.txt")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn given_input_arg_and_decode_arg_expect_decoded_string_in_stdout() {
    let expected = fs::read_to_string("./tests/resources/utf8.txt").unwrap();

    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("-i")
        .arg("./tests/resources/utf8.b64")
        .arg("-d")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn given_stdin_and_output_arg_expect_encoded_string_in_file() {
    let filepath = format!("./tests/t1-{}", Utc::now().to_rfc3339());

    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("--output")
        .arg(&filepath)
        .write_stdin("client:secret")
        .assert()
        .success()
        .stdout("");

    let result = fs::read_to_string(&filepath).unwrap();
    assert_eq!(result, "Y2xpZW50OnNlY3JldA==");

    fs::remove_file(&filepath).unwrap();
}

#[test]
fn given_binary_file_input_arg_expect_encoded_string_in_stdout() {
    let expected = fs::read_to_string("./tests/resources/bytes.b64").unwrap();

    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("--input")
        .arg("./tests/resources/bytes.bin")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn given_non_utf8_encoded_binary_and_output_arg_expect_decoded_binary_file() {
    let filepath = format!("./tests/t2-{}.bin", Utc::now().to_rfc3339());

    Command::cargo_bin("rbase64")
        .unwrap()
        .arg("--input")
        .arg("./tests/resources/bytes.b64")
        .arg("--output")
        .arg(&filepath)
        .arg("--decode")
        .assert()
        .success()
        .stdout("");

    let result = fs::read(&filepath).unwrap();
    let expected = fs::read("./tests/resources/bytes.bin").unwrap();

    assert_eq!(result.len(), expected.len());
    assert_eq!(result, expected);

    fs::remove_file(&filepath).unwrap();
}
