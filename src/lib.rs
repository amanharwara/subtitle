mod subdb;
use clap::ArgMatches;
use std::{fs, io, path::Path};
use subdb::{download, hash, search};

pub fn run(matches: ArgMatches) -> Result<(), reqwest::Error> {
    let mut current_api = "subdb";
    if let Some(api) = matches.value_of("api") {
        current_api = api;
    };

    println!("Current API: {}", current_api);

    let mut current_lang = "en";
    if let Some(lang) = matches.value_of("lang") {
        current_lang = lang;
    }

    println!("Current Language: {}", current_lang.to_uppercase());

    match current_api {
        "subdb" => match matches.values_of("file") {
            Some(files) => {
                let files: Vec<&str> = files.collect();
                for file in files.iter() {
                    println!("Generating hash for file {}", file);

                    let hash = hash(file);
                    println!("Hash generated: {}", hash);

                    let search = search(&*hash)?;
                    let languages: Vec<&str> = search.split(",").collect();

                    println!("Available Languages: {:?}", languages);

                    // Replace extension of file to .srt
                    let ext = Path::new(file).extension().unwrap().to_str().unwrap();
                    let filename = file.to_string().replace(ext, "srt");

                    let downloaded;

                    if languages.len() > 0 {
                        if languages.contains(&current_lang) {
                            downloaded = download(&*hash, current_lang)?;
                            match save_file(&downloaded, filename) {
                                Ok(_) => println!("File saved!"),
                                Err(err) => {
                                    println!("Error while saving file: {}", err);
                                }
                            }
                        } else {
                            if languages.contains(&"en") {
                                println!("Could not find a subtitle in the selected language. Downloading the English subtitles.");
                                downloaded = download(&*hash, "en")?;
                                match save_file(&downloaded, filename) {
                                    Ok(_) => println!("File saved!"),
                                    Err(err) => {
                                        println!("Error while saving file: {}", err);
                                    }
                                }
                            } else {
                                println!("Could not find a subtitle in the selected language. You can try again by changing the language using the --lang or -l flag.");
                            }
                        }
                    } else {
                        println!("No subtitle found for the given file.");
                        return Ok(());
                    }
                }
            }
            None => {}
        },
        "opensubtitles" => todo!(),
        _ => {}
    }

    Ok(())
}

fn save_file(content: &str, filename: String) -> io::Result<()> {
    println!("Saving to {}", filename);
    fs::write(filename, content)
}

#[cfg(test)]
mod tests {
    use super::subdb::hash;

    #[test]
    fn test_hash_fn() {
        assert_eq!(
            &*hash("./test_videos/dexter.mp4"),
            "ffd8d4aa68033dc03d1c8ef373b9028c"
        );
        assert_eq!(
            &*hash("./test_videos/justified.mp4"),
            "edc1981d6459c6111fe36205b4aff6c2"
        );
    }
}
