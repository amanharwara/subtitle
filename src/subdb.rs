use super::error::Result;
use super::save_file;
use clap::Values;
use md5::{Digest, Md5};
use reqwest::Url;
use std::fs;
use std::path::Path;

pub fn use_subdb(files: Values, current_lang: &str, dont_use_lang_fallback: bool) -> Result<()> {
    let files: Vec<&str> = files.collect();
    if files.len() > 0 {
        for file in files.iter() {
            println!("\nGenerating hash for file {}", file);

            let hash = hash(file);
            println!("Hash generated: {}", hash);

            let search = search(&*hash)?;
            let languages: Vec<&str> = search.split(",").collect();

            println!("\nAvailable Languages: {:?}", languages);

            // Replace extension of file to .srt
            let ext = Path::new(file).extension().unwrap().to_str().unwrap();
            let filename = file.to_string().replace(ext, "srt");

            let downloaded;

            if languages.len() > 0 {
                if languages.contains(&current_lang) {
                    println!("\nSubtitle found in the currently selected language.");
                    downloaded = download(&*hash, current_lang)?;
                    match save_file(&downloaded, &filename) {
                        Ok(_) => println!("File saved!"),
                        Err(err) => {
                            println!("\nError while saving file: {}", err);
                        }
                    }
                } else {
                    if languages.contains(&"en") && !dont_use_lang_fallback {
                        println!("\nCould not find a subtitle in the selected language. Downloading the English subtitles.");
                        downloaded = download(&*hash, "en")?;
                        match save_file(&downloaded, &filename) {
                            Ok(_) => println!("File saved!"),
                            Err(err) => {
                                println!("\nError while saving file: {}", err);
                            }
                        }
                    } else {
                        println!("\nCould not find a subtitle in the selected language. You can try again by changing the language using the --lang or -l flag.");
                    }
                }
            } else {
                println!("\nNo subtitle found for the given file.");
                return Ok(());
            }
        }
    }

    Ok(())
}

pub fn search(hash: &str) -> Result<String, reqwest::Error> {
    let response;

    let base_url = if cfg!(debug_assertions) {
        "http://sandbox.thesubdb.com/"
    } else {
        "http://api.thesubdb.com/"
    };

    let url = Url::parse_with_params(base_url, &[("action", "search"), ("hash", hash)]).unwrap();

    println!("Searching: {}", url);

    let user_agent = concat!(
        "SubDB/1.0 (",
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
        "; ",
        env!("CARGO_PKG_REPOSITORY"),
        ")"
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()?;

    response = client.get(url).send()?.text();

    response
}

pub fn download(hash: &str, language: &str) -> Result<String, reqwest::Error> {
    let base_url = if cfg!(debug_assertions) {
        "http://sandbox.thesubdb.com/"
    } else {
        "http://api.thesubdb.com/"
    };

    let url = Url::parse_with_params(
        base_url,
        &[
            ("action", "download"),
            ("hash", hash),
            ("language", language),
        ],
    )
    .unwrap();

    println!("Downloading: {}", url);

    let user_agent = concat!(
        "SubDB/1.0 (",
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
        "; ",
        env!("CARGO_PKG_REPOSITORY"),
        ")"
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()?;

    client.get(url).send()?.text()
}

/**
SubDB Hash Function as described on http://thesubdb.com/api/
*/
pub fn hash(file: &str) -> String {
    let mut hasher = Md5::new();
    let file_stream = fs::read(file).unwrap();

    // First 64kb of file
    let first_hash = file_stream[0..(64 * 1024)].to_owned();
    hasher.update(first_hash);

    // Last 64kb of file
    let last_hash = file_stream[file_stream.len() - (64 * 1024)..file_stream.len()].to_owned();
    hasher.update(last_hash);

    let hash = hasher.finalize();
    let hash = hash.iter().map(|byte| format!("{:02x}", byte)).collect();

    hash
}
