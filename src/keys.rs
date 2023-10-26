use rpassword::read_password;
use std::fs;
use std::io;
use std::io::Write;
use std::sync::Mutex;

use crate::pastebin;

pub static API_KEY: Mutex<String> = Mutex::new(String::new());
pub static USER_KEY: Mutex<String> = Mutex::new(String::new());

pub fn get_api_key() {
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

pub async fn get_user_key() {
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

        print!("Username: ");
        io::stdout().lock().flush().unwrap();
        io::stdin()
            .read_line(&mut username)
            .expect("Error reading Username");

        print!("Password: ");
        io::stdout().lock().flush().unwrap();
        let password = read_password().unwrap();

        let key = pastebin::get_user_key(&API_KEY.lock().unwrap(), username, password)
            .await
            .unwrap();
        USER_KEY.lock().unwrap().push_str(&key);

        fs::write(key_file, key).expect("Error writing API KEY");

        return;
    }

    USER_KEY.lock().unwrap().push_str(&key);
}

pub fn clear_keys() {
    API_KEY.lock().unwrap().clear();
    USER_KEY.lock().unwrap().clear();

    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };

    let key_file = home_dir.join(".pastebin_key");
    let user_key_file = home_dir.join(".pastebin_userkey");

    fs::remove_file(key_file).expect("Error removing API KEY file");
    fs::remove_file(user_key_file).expect("Error removing USER KEY file");
}