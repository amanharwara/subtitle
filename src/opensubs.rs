use super::error::Result;
use super::{authenticate_os_user, save_file};
use colored::Colorize;
use dialoguer::Confirm;
use reqwest::{header::CONTENT_TYPE, Url};
use std::{
    fs,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

const API_KEY: &str = "pmGQgkYQjVnEUBc05cApQs7cnfw3Mrdo";

pub fn get_user_info(token: &str) -> Result<serde_json::Value> {
    let client = reqwest::blocking::Client::builder().build()?;
    let url = "https://www.opensubtitles.com/api/v1/infos/user";
    let response: serde_json::Value = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header("Api-Key", API_KEY)
        .header("Authorization", format!("Bearer {}", token))
        .send()?
        .json()?;
    Ok(response)
}

pub fn get_os_token(username: &str, password: &str) -> Result<String> {
    let token: String;

    println!(
        "{}",
        "Generating token using username & password...".green()
    );

    let client = reqwest::blocking::Client::builder().build()?;

    let url = "https://www.opensubtitles.com/api/v1/login";

    let response: serde_json::Value = client
        .post(url)
        .body(format!(
            "{{\n\"username\":\"{}\",\n\"password\":\"{}\"\n}}",
            username, password
        ))
        .header(CONTENT_TYPE, "application/json")
        .header("Api-Key", API_KEY)
        .send()?
        .json()?;

    if response["status"] == 401 {
        println!("{}", "[!] Error: Unauthorized.".red().bold());
        return Ok(String::new());
    }

    token = response["token"].to_string();

    println!(
        "{}: {}",
        format!("Generated token for {}", username).bold().green(),
        &token
    );

    Ok(token)
}

fn search_subtitles(fname: &str, hash: &str) -> Result<serde_json::Value> {
    let client = reqwest::blocking::Client::builder().build()?;

    let url = Url::parse_with_params(
        "https://www.opensubtitles.com/api/v1/subtitles",
        &[("query", fname), ("moviehash", hash)],
    )
    .unwrap();

    let response: serde_json::Value = client.get(url).header("Api-Key", API_KEY).send()?.json()?;

    Ok(response["data"].to_owned())
}

fn get_subtitle_link(file_id: &str, token: &str) -> Result<String> {
    println!("{}", "Getting download link.".green());

    let client = reqwest::blocking::Client::builder().build()?;

    let url = "https://www.opensubtitles.com/api/v1/download";

    let response: serde_json::Value = client
        .post(url)
        .body(format!(
            "{{\"file_id\":\"{}\",\"sub_format\":\"srt\"}}",
            file_id
        ))
        .header("Api-Key", API_KEY)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()?
        .json()?;

    Ok(response["link"].to_string().replace("\"", ""))
}

fn download_subtitle(link: &str, fname: &str) -> Result<()> {
    println!("{}", "[!] Download started...".green().bold());

    let client = reqwest::blocking::Client::builder().build()?;

    let response = client.get(link).send()?.text()?;

    match save_file(&response, fname) {
        Ok(_) => println!("{}", "[!] File saved!".green().bold()),
        Err(err) => println!("{} {}", "[!] Error while saving file:".red().bold(), err),
    }

    Ok(())
}

pub fn use_opensubs(files: clap::Values, lang: &str, token: &str) -> Result<()> {
    match get_user_info(token) {
        Ok(user_info) => {
            println!(
                "{}",
                format!(
                    "[!] {} downloads remaining.",
                    user_info["data"]["remaining_downloads"].to_string()
                )
                .green()
                .bold()
            );
        }
        Err(_) => {
            if Confirm::new()
                .with_prompt(format!(
                    "{} Do you want to reset your token?",
                    "[!] Error loading user info.".red().bold()
                ))
                .interact()?
            {
                match authenticate_os_user() {
                    Ok(_) => {
                        use_opensubs(files.to_owned(), lang, token)?;
                    }
                    Err(err) => {
                        println!("{} {:#?}", "[!] Error: ".red().bold(), err);
                        return Ok(());
                    }
                }
            } else {
                println!(
                    "{}",
                    "[!] Can't use OpenSubtitles API without valid token."
                        .red()
                        .bold()
                );
                return Ok(());
            }
        }
    }

    let files: Vec<&str> = files.collect();
    if files.len() > 0 {
        for file in files.iter() {
            println!("{} {}", "Generating hash for".green(), file);
            let hash = hash(*file)?;
            println!("{} {}", "[!] Hash generated:".green().bold(), &hash);

            println!("{}", "Searching subtitles...".green());
            let results = search_subtitles(*file, &hash)?;
            let filtered_results: Vec<&serde_json::Value> = results
                .as_array()
                .unwrap()
                .iter()
                .filter(|&subtitle| subtitle["attributes"]["language"] == lang)
                .collect();

            if filtered_results.len() > 0 {
                println!("{} Downloading...", "[!] Found subtitle.".green().bold());
                let subtitle = filtered_results[0];
                let file_id = subtitle["attributes"]["files"][0]["file_id"].to_string();
                let link = get_subtitle_link(&file_id, token)?;

                let ext = Path::new(file).extension().unwrap().to_str().unwrap();
                let fname = file.replace(ext, "srt");

                download_subtitle(&link, &fname)?
            } else {
                println!("{}", "[!] Could not find suitable subtitle.".red().bold());
            }
        }
    }

    Ok(())
}

const HASH_BLK_SIZE: u64 = 65536;

/** OpenSubs hash function
taken from https://trac.opensubtitles.org/projects/opensubtitles/wiki/HashSourceCodes#RUST */
pub fn hash(fname: &str) -> Result<String> {
    let fsize = fs::metadata(fname)?.len();
    let mut hash = String::new();

    if fsize > HASH_BLK_SIZE {
        let file = File::open(fname)?;

        let mut buf = [0u8; 8];
        let mut word: u64;

        let mut hash_val: u64 = fsize; // seed hash with file size

        let iterations = HASH_BLK_SIZE / 8;

        let mut reader = BufReader::with_capacity(HASH_BLK_SIZE as usize, file);

        for _ in 0..iterations {
            reader.read(&mut buf)?;
            word = u64::from_ne_bytes(buf);
            hash_val = hash_val.wrapping_add(word);
        }

        reader.seek(SeekFrom::Start(fsize - HASH_BLK_SIZE))?;

        for _ in 0..iterations {
            reader.read(&mut buf)?;
            word = u64::from_ne_bytes(buf);
            hash_val = hash_val.wrapping_add(word);
        }

        hash = format!("{:01$x}", hash_val, 16);
    }

    Ok(hash)
}
