use clap::{Parser, Subcommand};
use rpassword::read_password;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
mod pastebin;

static API_KEY: Mutex<String> = Mutex::new(String::new());
static USER_KEY: Mutex<String> = Mutex::new(String::new());

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

        print!("Pastebin API key: ");
        io::stdout().lock().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading API_KEY");

        API_KEY.lock().unwrap().push_str(&input);

        fs::write(key_file, input).expect("Error writing API KEY");

        return;
    }

    API_KEY.lock().unwrap().push_str(&key);
}

async fn get_user_key() {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };

    let key_file = home_dir.join(".pastebin_userkey");
    let key = match fs::read_to_string(&key_file) {
        Ok(d) => d,
        Err(_) => "".to_owned(),
    };

    if key == "" {
        let mut username = String::new();
        let mut password = String::new();

        print!("Username: ");
        io::stdout().lock().flush().unwrap();
        io::stdin()
            .read_line(&mut username)
            .expect("Error reading Username");

        print!("Password: ");
        io::stdout().lock().flush().unwrap();
        password = read_password().unwrap();

        let key = pastebin::get_user_key(&API_KEY.lock().unwrap(), username, password)
            .await
            .unwrap();
        USER_KEY.lock().unwrap().push_str(&key);

        fs::write(key_file, key).expect("Error writing API KEY");

        return;
    }

    USER_KEY.lock().unwrap().push_str(&key);
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
    get_user_key().await;

    let args = Cli::parse();

    match args.action {
        Action::Create { file_path } => create(file_path).await,
        Action::Delete { paste_id } => delete(paste_id).await,
        Action::Edit {
            paste_id,
            file_path,
        } => edit(paste_id, file_path).await,
    };
}

async fn create(file_path: PathBuf) {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_content = fs::read_to_string(&file_path).unwrap();

    let paste_id = pastebin::create_paste(
        &API_KEY.lock().unwrap(),
        &USER_KEY.lock().unwrap(),
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
        &API_KEY.lock().unwrap(),
        &USER_KEY.lock().unwrap(),
        paste_id,
    )
    .await
    .unwrap();

    println!("{}", resp);
}

async fn edit(paste_id: String, file_path: PathBuf) {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_content = fs::read_to_string(&file_path).unwrap();

    let resp = pastebin::create_paste(
        &API_KEY.lock().unwrap(),
        &USER_KEY.lock().unwrap(),
        file_name.to_owned(),
        file_content,
        Some(paste_id),
    )
    .await
    .unwrap();

    println!("{}", resp);
}
