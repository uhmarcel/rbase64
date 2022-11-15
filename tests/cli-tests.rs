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
        .stderr(predicate::str::contains("Unable to decode non-base64 character '!'"));
}

#[test]
fn given_input_arg_expect_encoded_string_in_stdout() {
    let expected = fs::read_to_string("./tests/resources/utf8.base64").unwrap();

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
    let expected = fs::read_to_string("./tests/resources/utf8.base64").unwrap();

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
        .arg("./tests/resources/utf8.base64")
        .arg("-d")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn given_stdin_and_output_arg_expect_encoded_string_in_file() {
    let filepath = format!("./tests/tmp/{}.txt", Utc::now().to_rfc3339());
    fs::create_dir("./tests/tmp").ok();

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

    fs::remove_dir_all("./tests/tmp").unwrap();
}
