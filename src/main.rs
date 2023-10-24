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
    /*
    Edit {
        paste_id: String,
        file_path: std::path::PathBuf,
    },
    */
}

#[tokio::main]
async fn main() {
    keys::get_api_key();
    keys::get_user_key().await;

    let args = Cli::parse();

    match args.action {
        Action::Create { file_path } => create(file_path).await,
        Action::Delete { paste_id } => delete(paste_id).await,
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

async fn edit(paste_id: String, file_path: PathBuf) {
    todo!("No offical way to do that yet");
}
