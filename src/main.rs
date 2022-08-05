mod repo;
mod installer;
mod clone;
extern crate sargparse;
use serde_json::{Value};
use sargparse::{ArgumentParser, ArgumentType};

#[tokio::main]
async fn main() {
    let mut parser = ArgumentParser::new(Some("Awesome program"));
    parser.add_argument("-i", "--install", "Program you want to install",
                        true, None, ArgumentType::STR);
    parser.add_argument("-r", "--repo", "Repository where you want to search application, if not specified, wineget search in default",
                        false, None, ArgumentType::STR);
    let args = parser.parse_args().unwrap();
    let appname = args.get("install").unwrap().get_str();
    let mut repo = args.get("repo").unwrap().get_str();
    if repo == "" {
        // Default repository
        repo = "https://raw.githubusercontent.com/Windows-On-Linux/Repo/main/repository.json".to_string();
    }
    let repos = update_repo(&repo).await;
    for (i, app) in repos.as_array().iter().enumerate() {
        let app_name_from_repo = app.get(i).unwrap()["CliName"].as_str().unwrap();
        if  app_name_from_repo == appname {
            let url = app.get(i).unwrap()["Repository"].as_str().unwrap();
            let program_name = app.get(i).unwrap()["Path"].as_str().unwrap();
            println!("\nFind {} in {} Git repository. \nAre you sure to install this program?[Y/N]", app_name_from_repo, url);
            let mut confirm = String::new();
            std::io::stdin().read_line(&mut confirm).unwrap();
            if confirm.to_lowercase().contains("y") {
                println!("Cloning repository...");
                let name = program_name.split("/").last().unwrap();
                let dir = dirs::home_dir().unwrap().as_path().join("wol").join("Downloads").join(name);
                if clone::clonerepo(url, name, dir.clone()){
                    println!("Clone finished successfully, starting the installation script");
                    installer::install(dir.join("build.sh"));
                }else{
                    panic!("Error during cloning repository");
                };
            }else{
                panic!("Abort installation");
            };
        };
    }
}

async fn update_repo(url: &str) -> Value {
    println!("Updating repository...");
    repo::update_repo(url).await
}

