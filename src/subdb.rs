use md5::{Digest, Md5};
use reqwest::Url;
use std::fs;

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
