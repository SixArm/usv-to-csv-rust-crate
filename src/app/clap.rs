//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! We prefer clap using the `command!` macro, which runs at compile time.
//! We prefer clap using the builder pattern, which offers more capabilities.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::Arg;

pub fn clap() -> crate::app::args::Args {
    let matches = clap::command!()
    .name("usv-to-csv")
    .version("1.2.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Convert Unicode Separated Values (USV) to Comma Separated Values (CSV)")
    .arg(Arg::new("test")
        .long("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .action(clap::ArgAction::Count))
    .arg(Arg::new("unit-separator")
        .help("Set the unit separator string")
        .short('u')
        .long("unit-separator")
        .default_value(",")
        .action(clap::ArgAction::Set))
    .arg(Arg::new("record-separator")
        .help("Set the record separator string")
        .short('r')
        .long("record-separator")
        .default_value("\n")
        .action(clap::ArgAction::Set))
    .get_matches();

    crate::app::args::Args {
        test: matches.get_flag("test"),
        log_level: crate::app::log::u8_to_log_level(matches.get_count("verbose")),
        unit_separator: matches.get_one::<String>("unit-separator").expect("unit-separator is missing default value").into(),
        record_separator: matches.get_one::<String>("record-separator").expect("record-separator is missing default value").into(),
    }
}
