use chrono::DateTime;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

mod keys;
mod pastebin;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about= None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    Create { file_path: std::path::PathBuf },
    Delete { paste_id: String },
    List { max_results: Option<u16> },
    ClearKeys,
}

#[tokio::main]
async fn main() {
    keys::get_api_key();
    keys::get_user_key().await;

    let args = Cli::parse();

    match args.action {
        Action::Create { file_path } => create(file_path).await,
        Action::Delete { paste_id } => delete(paste_id).await,
        Action::List { max_results } => list(max_results).await,
        Action::ClearKeys => keys::clear_keys(),
    };
}

async fn create(file_path: PathBuf) {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_content = fs::read_to_string(&file_path).unwrap();

    let paste_id = pastebin::create_paste(
        &keys::API_KEY.lock().unwrap(),
        &keys::USER_KEY.lock().unwrap(),
        file_name.to_owned(),
        file_content,
        None,
    )
    .await
    .unwrap();

    println!("{}", paste_id);
}

async fn delete(paste_id: String) {
    let resp = pastebin::delete_paste(
        &keys::API_KEY.lock().unwrap(),
        &keys::USER_KEY.lock().unwrap(),
        paste_id,
    )
    .await
    .unwrap();

    println!("{}", resp);
}

async fn list(max_results: Option<u16>) {
    let resp = pastebin::list_pastes(
        &keys::API_KEY.lock().unwrap(),
        &keys::USER_KEY.lock().unwrap(),
        max_results.unwrap_or(10),
    )
    .await
    .unwrap();

    let line = "-".repeat(5);
    println!("{}Top {} pastes{}", line, max_results.unwrap_or(10), line);

    let mut max_length = 8;
    let max = resp.iter().map(|paste| paste.paste_title.len()).max();
    if max.is_some() {
        if max.unwrap() > max_length {
            max_length = max.unwrap();
        }
    }

    for paste in resp {
        let mut title = paste.paste_title;
        if title == "" {
            title = "Untitled".to_owned();
        }

        title = pad_string(&title, max_length);

        let datetime = DateTime::from_timestamp(paste.paste_date.into(), 0).unwrap();
        let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S");

        println!("{}\t{}\t{}", title, formatted_date, paste.paste_url);
    }
}

fn pad_string(s: &str, length: usize) -> String {
    let mut s = s.to_owned();
    while s.len() < length {
        s.push(' ');
    }
    s
}
