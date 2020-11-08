use std::env;
extern crate reqwest;
use clap::{App, Arg};
use subtitle::run;
fn main() -> Result<(), reqwest::Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("file")
                .index(1)
                .value_name("FILE")
                .about("Get subtitles using a file.")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::new("api")
                .long("api")
                .value_name("API_NAME")
                .about("The API to use for downloading subtitles.")
                .takes_value(true)
                .possible_values(&["subdb", "opensubtitles"])
                .default_value("subdb"),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .short('l')
                .value_name("LANGUAGE")
                .about("The preferred language to download the subtitles in.")
                .takes_value(true)
                .default_value("en"),
        )
        .get_matches();

    run(matches)
}
