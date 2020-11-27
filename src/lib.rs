mod config;
pub mod error;
mod opensubs;
mod subdb;
mod utils;

use clap::ArgMatches;
use config::Config;
use dialoguer::{Confirm, Input, Password};
use error::{Error, Result};
use opensubs::{get_os_token, use_opensubs};
use std::fs;
use subdb::use_subdb;

pub fn run(matches: ArgMatches) -> Result<()> {
    let config = Config::new()?;

    let mut current_api: &str = &config.api;
    let mut current_lang: &str = &config.lang;
    let mut dont_use_lang_fallback = config.dont_use_lang_fallback;
    let os_token: &str = &config.os_token;

    if let Some(api) = matches.value_of("api") {
        current_api = api;
    };

    println!("Current API: {}", current_api);

    if let Some(lang) = matches.value_of("lang") {
        current_lang = lang;
    }

    println!("Current Language: {}", current_lang.to_uppercase());

    if matches.is_present("dont_use_lang_fallback") {
        dont_use_lang_fallback = true;
        println!("\nWon't fallback to English subtitles.");
    }

    match matches.values_of("file") {
        Some(files) => match current_api {
            "subdb" => use_subdb(files, current_lang, dont_use_lang_fallback)?,
            "opensubtitles" => {
                if os_token.len() > 0 {
                    use_opensubs(files, current_lang, os_token)?
                } else {
                    println!("\nNo OpenSubtitles token found.");
                    authenticate_os_user()?;
                }
            }
            _ => {}
        },
        None => {
            println!("No files provided.");
        }
    }

    Ok(())
}

pub fn authenticate_os_user() -> Result<()> {
    let mut config = Config::new()?;

    if Confirm::new()
        .with_prompt("Do you have an OpenSubtitles token?")
        .interact()?
    {
        let token: String = Input::new()
            .with_prompt("Input your token")
            .interact_text()?;
        config.set_os_token(token)?
    } else {
        println!("\nYou can generate a token with your username & password.");
        let username: String = Input::new().with_prompt("Username").interact_text()?;
        let password: String = Password::new().with_prompt("Password").interact()?;
        let token = get_os_token(&username, &password)?;
        config.set_os_token(token)?
    }

    Ok(())
}

pub fn save_file(content: &str, filename: &str) -> Result<()> {
    println!("Saving to {}", filename);
    fs::write(filename, content).map_err(|e| Error::IO(e))
}

#[cfg(test)]
mod tests {
    use super::opensubs::hash as opensubs_hash;
    use super::subdb::hash as subdb_hash;

    #[test]
    fn test_hash_fn() {
        assert_eq!(
            &*subdb_hash("./test_videos/subdb/dexter.mp4"),
            "ffd8d4aa68033dc03d1c8ef373b9028c"
        );
        assert_eq!(
            &*subdb_hash("./test_videos/subdb/justified.mp4"),
            "edc1981d6459c6111fe36205b4aff6c2"
        );
        assert_eq!(
            &opensubs_hash("./test_videos/opensubs/breakdance.avi").unwrap(),
            "8e245d9679d31e12"
        );
    }
}
