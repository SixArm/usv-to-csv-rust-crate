mod common; use common::*;
use std::process::Command;

#[test]
fn default_separators() {
    let input = "a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞";
    let expect = "a,b,c\nd,e,f\ng,h,i\n";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command, input);
    assert_eq!(actual, expect);
}
