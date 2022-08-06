#![allow(dead_code)]    // Just to remove useless warnings

use serde_json::{Value};

#[derive(Debug)]
pub struct App {    // App struct in repository
    name: String,
    cli_name: String,
    description: String,
    repository: String,
    author: String,
    version: String
}

// Uppercase first letter from https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Download our repository
pub async fn update_repo(url: &str) -> Value {
    let resp = reqwest::get(url)
        .await;
    let data = resp.unwrap().text().await.unwrap();
    let parsed:Value = serde_json::from_str(data.as_str()).unwrap();
    parsed
}

// Filter result in repository
pub async fn search_repo(url: String, app: String) -> Vec<App>{
    let apps_from_repo = update_repo(&url).await;
    let mut apps = Vec::new();
    for i in 0..apps_from_repo.as_array().unwrap().len(){
        let app_name_from_repo = apps_from_repo.get(i).unwrap()["App"].as_str().unwrap().to_lowercase();
        if app_name_from_repo.contains(&app){
            let app = App{
              cli_name: apps_from_repo.get(i).unwrap()["CliName"].to_string(),
                name: uppercase_first_letter(apps_from_repo.get(i).unwrap()["App"].as_str().unwrap()),
                description: apps_from_repo.get(i).unwrap()["Description"].to_string(),
                repository: apps_from_repo.get(i).unwrap()["Repository"].to_string(),
                author: apps_from_repo.get(i).unwrap()["Author"].to_string(),
                version: apps_from_repo.get(i).unwrap()["Version"].to_string(),
            };
            apps.push(app);
        }
    }
    apps
}