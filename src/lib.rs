mod config;
pub mod error;
mod opensubs;
mod utils;

use clap::ArgMatches;
use config::Config;
use dialoguer::{Confirm, Input, Password};
use error::{Error, Result};
use opensubs::{get_os_token, use_opensubs};
use std::fs;

pub fn run(matches: ArgMatches) -> Result<()> {
    let config = Config::new()?;

    let mut current_lang: &str = &config.lang;
    let os_token: &str = &config.os_token;

    if let Some(lang) = matches.value_of("lang") {
        current_lang = lang;
    }

    println!("Current Language: {}", current_lang.to_uppercase());

    match matches.values_of("file") {
        Some(files) => {
            if os_token.len() > 0 {
                use_opensubs(files, current_lang, os_token)?
            } else {
                println!("\nNo OpenSubtitles token found.");
                authenticate_os_user()?;
            }
        }
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
    use super::opensubs::hash;

    #[test]
    fn test_hash_fn() {
        assert_eq!(
            &hash("./test_videos/opensubs/breakdance.avi").unwrap(),
            "8e245d9679d31e12"
        );
    }
}
