//! # usv-to-csv
//!
//! Convert [Unicode Separated Values (USV)](https://github.com/sixarm/usv) to
//! Comma Separated Values (CSV).
//!
//! Syntax:
//!
//! ```sh
//! stdin | usv-to-csv [options] | stdout
//! ```
//!
//! Example:
//!
//! ```sh
//! cat example.usv | usv-to-csv
//! ```
//!
//! Example with output to a file:
//!
//! ```sh
//! cat example.usv | usv-to-csv > example.csv
//! ```
//!
//! Example with custom separators:
//!
//! ```sh
//! cat example.usv | usv-to-csv --unit-separator ";" --record-separator "*"
//! ```
//!
//! ## Options
//!
//! * -u, --unit-separator <unit-separator> : Set the unit separator string [default: ","]
//!
//! * -r, --record-separator <record-separator> : Set the record separator string [default: "\n"]
//!
//! * -h, --help : Print help
//!
//! * -V, --version : Print version
//!
//! * -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …
//!
//! * --test : Print test output for debugging, verifying, tracing, and the like. Example: --test
//!
//! ## Install
//!
//! Install:
//!
//! ```sh
//! cargo install usv-to-csv
//! ```
//!
//! Link:
//! [https://crates.io/crates/usv-to-csv](https://crates.io/crates/usv-to-csv)
//!
//! ## Example
//!
//! Suppose example.usv contains:
//!
//! ```usv
//! a␟b␟c␟␞
//! d␟e␟f␟␞
//! g␟h␟i␟␞
//! ```
//!
//! Run:
//!
//! ```sh
//! cat example.usv | usv-to-csv
//! ```
//!
//! Output:
//!
//! ```csv
//! a,b,c
//! d,e,f
//! g,h,i
//! ```
//!
//! Run:
//!
//! ```sh
//! cat example.usv | usv-to-csv --unit-separator ";" --record-separator "*"
//! ```
//!
//! Output:
//!
//! ```csv
//! a;b;c*d;e;f*g;h;i
//!
//! ```
//! ## FAQ
//!
//! ### When to use this command?
//!
//! Use this command when you want to convert from USV to CSV.
//!
//! A typical use case is when you have USV data, such as a collection of units
//! and
//! records, and you want to convert it to CSV data, such as for a spreadsheet
//! import.
//!
//! Our real-world use case is converting a bunch of USV document-oriented data
//! from a variety of programs, including a CMS, to USV so we're better-able to
//! import the data into Excel.
//!
//! ### Is there a similar command to convert from CSV to USV?
//!
//! Yes: [csv-to-usv](https://crates.io/crates/csv-to-usv).
//!
//! ### Why use USV instead of CSV?
//!
//! See the documentation for [USV](https://github.com/sixarm/usv).
//!
//! ### Is USV aiming to become a standard?
//!
//! Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
//! [link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).
//!
//! ## Help wanted
//!
//! Constructive feedback welcome. Pull requests and feature requests welcome.
//!
//! ## Tracking
//!
//! * Package: usv-to-csv-rust-crate
//! * Version: 1.2.0
//! * Created: 2024-03-09-13:33:20Z
//! * Updated: 2024-03-13T12:06:01Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

//// log
#[macro_use]
extern crate log;
extern crate env_logger;

use usv_to_csv::usv_to_csv_with_separators;
use std::io::{Read, stdin};

pub mod app {
    pub mod args;
    pub mod clap;
    pub mod log;
}

fn main() -> std::io::Result<()> {
    let args: crate::app::args::Args = crate::app::clap::clap();
    if args.test { println!("{:?}", args); }
    let mut stdin = stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    println!("{}", usv_to_csv_with_separators(&s, args.unit_separator, args.record_separator));
    Ok(())
}