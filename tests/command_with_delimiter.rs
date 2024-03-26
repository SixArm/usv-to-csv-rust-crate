mod common; use common::*;
use std::process::Command;

const EXAMPLE_INPUT: &str = "a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞";
const EXAMPLE_OUTPUT: &str = "a;b;c\nd;e;f\ng;h;i\n";

#[test]
fn command_with_delimiter_with_short_arg() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("-d").arg(";"), EXAMPLE_INPUT);
    assert_eq!(actual, EXAMPLE_OUTPUT);
}

#[test]
fn command_with_delimiter_with_long_arg() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--delimiter").arg(";"), EXAMPLE_INPUT);
    assert_eq!(actual, EXAMPLE_OUTPUT);
}
