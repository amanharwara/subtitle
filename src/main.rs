use std::env;
extern crate reqwest;
use clap::{App, Arg};
use subtitle::{error::Result, run};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("file")
                .index(1)
                .value_name("FILE(S)")
                .about("Get subtitles for a file or multiple files.")
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
        .arg(
            Arg::new("dont_use_lang_fallback")
                .long("dont-use-lang-fallback")
                .short('d')
                .value_name("VALUE")
                .takes_value(false)
                .about(
                    "When used, app will stop if subtitles aren't found for the selected language.",
                ),
        )
        // Commented out subcommand because there's no documentation on how to use it.
        /* .subcommand(
            App::new("config")
                .about("Modify the configuration")
                .subcommand(
                    App::new("set")
                        .about("Set a value in the config.")
                        .arg(
                            Arg::new("key")
                                .index(1)
                                .value_name("KEY")
                                .takes_value(true)
                                .about("The key in the config to change."),
                        )
                        .arg(
                            Arg::new("value")
                                .index(2)
                                .value_name("VALUE")
                                .takes_value(true)
                                .about("The value to set for the given key."),
                        ),
                )
                .subcommand(
                    App::new("file").about("Set a new file as the config.").arg(
                        Arg::new("conf_file")
                            .index(1)
                            .value_name("FILE")
                            .takes_value(true)
                            .about("The file to set as the new config."),
                    ),
                )
                .subcommand(App::new("list").about("List the complete configuration")),
        ) */
        .get_matches();

    run(matches)
}
