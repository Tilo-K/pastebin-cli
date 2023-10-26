use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paste {
    pub paste_key: String,
    pub paste_date: i64,
    pub paste_title: String,
    pub paste_size: u64,
    pub paste_expire_date: i64,
    pub paste_private: u8,
    pub paste_format_long: String,
    pub paste_format_short: String,
    pub paste_url: String,
    pub paste_hits: u64,
}

pub async fn create_paste(
    api_key: &str,
    user_key: &str,
    name: String,
    content: String,
    paste_id: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("api_dev_key", api_key.trim().to_owned());
    map.insert("api_user_key", user_key.trim().to_owned());
    map.insert("api_option", "paste".to_owned());
    map.insert("api_paste_code", content);
    map.insert("api_paste_private", "0".to_owned());
    map.insert("api_paste_expire_date", "1W".to_owned());
    map.insert("api_paste_name", name);

    if paste_id.is_some() {
        let id = paste_id.unwrap();
        map.insert("api_paste_key", id);
    }

    let req = client
        .post("https://pastebin.com/api/api_post.php")
        .form(&map);

    let resp = req.send().await?;
    if resp.status() == 200 {
        let resp_body = resp.text().await?;
        return Ok(resp_body);
    }
    return Ok(resp.status().to_string() + "\n" + &resp.text().await?);
}

pub async fn delete_paste(
    api_key: &str,
    user_key: &str,
    paste_id: String,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("api_dev_key", api_key.trim().to_owned());
    map.insert("api_user_key", user_key.trim().to_owned());
    map.insert("api_option", "delete".to_owned());
    map.insert("api_paste_key", paste_id);

    let req = client
        .post("https://pastebin.com/api/api_post.php")
        .form(&map)
        .send()
        .await?;

    if req.status() == 200 {
        return Ok("Deleted".to_owned());
    }
    dbg!(&req);

    Ok(req.status().to_string() + "\n" + &req.text().await?)
}

pub async fn get_user_key(api_key: &str, username: String, password: String) -> Result<String> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("api_dev_key", api_key.trim().to_owned());
    map.insert("api_user_name", username.trim().to_owned());
    map.insert("api_user_password", password.trim().to_owned());

    let req = client
        .post("https://pastebin.com/api/api_login.php")
        .form(&map)
        .send()
        .await?;

    if req.status() == 200 {
        return Ok(req.text().await?);
    }
    dbg!(&req);
    println!("{}", req.text().await?);
    panic!("Error logging in.");
}

pub async fn list_pastes(api_key: &str, user_key: &str, max_results: u16) -> Result<Vec<Paste>> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("api_dev_key", api_key.trim().to_owned());
    map.insert("api_user_key", user_key.trim().to_owned());
    map.insert("api_option", "list".to_owned());
    map.insert("api_results_limit", max_results.to_string());

    let req = client
        .post("https://pastebin.com/api/api_post.php")
        .form(&map)
        .send()
        .await?;

    if req.status() == 200 {
        let data = req.text().await?;
        let pastes: Vec<Paste> = quick_xml::de::from_str(&data).unwrap();
        return Ok(pastes);
    }

    return Err(anyhow::format_err!(
        "{}\n{}",
        req.status().to_string(),
        req.text().await?
    ));
}
