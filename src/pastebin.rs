use std::error::Error;

pub async fn create_paste(
    api_key: &str,
    name: String,
    content: String,
    paste_id: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut req = client
        .post("https://pastebin.com/api/api_post.php")
        .query(&[
            ("api_dev_key", api_key),
            ("api_option", "paste"),
            ("api_paste_code", &content),
            ("api_paste_private", "1"),
            ("api_paste_expire_date", "1W"),
            ("api_paste_name", &name),
        ]);

    if (paste_id.is_some()) {
        req = req.query(&[("api_paste_key", &paste_id.unwrap())]);
    }

    let resp = req.send().await?;
    if resp.status() == 200 {
        let resp_body = resp.text().await?;
        return Ok(resp_body);
    }

    return Ok(resp.status().to_string());
}

pub async fn delete_paste(api_key: &str, paste_id: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let req = client
        .post("https://pastebin.com/api/api_post.php")
        .query(&[
            ("api_dev_key", api_key),
            ("api_option", "delete"),
            ("api_paste_key", &paste_id),
        ])
        .send()
        .await?;

    if req.status() == 200 {
        return Ok("Deleted".to_owned());
    }

    Ok(req.status().to_string())
}
