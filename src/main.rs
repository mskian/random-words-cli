use clap::{Arg, Command};
use reqwest::Error;
use serde::Deserialize;
use std::env;
use tokio;
use dotenvy::dotenv;

#[derive(Deserialize, Debug)]
struct Word {
    word: String,
    definition: String,
    pronunciation: String,
}

async fn fetch_random_word(api_url: &str) -> Result<Vec<Word>, Error> {
    let response = reqwest::get(api_url).await?.json::<Vec<Word>>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_url = env::var("RANDOMWORDS_API_URL").expect("RANDOMWORDS_API_URL is not set in .env");
    if !api_url.starts_with("http://") && !api_url.starts_with("https://") {
        eprintln!("Invalid API_URL in .env: Must start with 'http://' or 'https://'");
        std::process::exit(1);
    }

    let matches = Command::new("Random Words CLI")
        .version("0.0.1")
        .author("Santhosh Veer")
        .about("Fetch a Random Word with Its Definition and Pronunciation")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Increases verbosity"),
        )
        .get_matches();

    match fetch_random_word(&api_url).await {
        Ok(words) => {
            if !words.is_empty() {
                let word = &words[0];
                println!("Word: {}", word.word);
                println!("Definition: {}", word.definition);
                println!("Pronunciation: {}", word.pronunciation);

                if matches.get_flag("verbose") {
                    println!("\nFull API Response: {:?}", words);
                }
            } else {
                eprintln!("No words found in the API response.");
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch word: {}", e);
        }
    }
}
