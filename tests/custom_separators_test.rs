mod common; use common::*;
use std::process::Command;

#[test]
fn custom_separators_with_short_args() {
    let input = "a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞";
    let expect = "a;b;c!d;e;f!g;h;i\n";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("-u").arg(";").arg("-r").arg("!"), input);
    assert_eq!(actual, expect);
}

#[test]
fn custom_separators_with_long_args() {
    let input = "a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞";
    let expect = "a;b;c!d;e;f!g;h;i\n";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--unit-separator").arg(";").arg("--record-separator").arg("!"), input);
    assert_eq!(actual, expect);
}
