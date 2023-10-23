use clap::{Parser, Subcommand};
use std::fs;
use std::io;
use std::sync::Mutex;

mod pastebin;

static API_KEY: Mutex<String> = Mutex::new(String::new());

fn get_api_key() {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };

    let key_file = home_dir.join(".pastebin_key");
    let key = match fs::read_to_string(&key_file) {
        Ok(d) => d,
        Err(_) => "".to_owned(),
    };

    if key == "" {
        let mut input = String::new();

        println!("Pastebin API key: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading API_KEY");

        API_KEY.lock().unwrap().push_str(&input);

        fs::write(key_file, input).expect("Error writing API KEY");

        return;
    }

    API_KEY.lock().unwrap().push_str(&key);
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about= None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    Create {
        file_path: std::path::PathBuf,
    },
    Delete {
        paste_id: String,
    },
    Edit {
        paste_id: String,
        file_path: std::path::PathBuf,
    },
}

#[tokio::main]
async fn main() {
    get_api_key();

    let args = Cli::parse();

    match args.action {
        Action::Create { file_path } => todo!(),
        Action::Delete { paste_id } => todo!(),
        Action::Edit {
            paste_id,
            file_path,
        } => todo!(),
    }
}
