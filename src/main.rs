mod repo;
mod installer;
mod search;
mod open;
mod parser;
mod clone;
extern crate sargparse;
use std::process::exit;
use sargparse::{ArgumentParser, ArgumentType};
use std::fs;

#[tokio::main]
async fn main() {

    let mut parser = ArgumentParser::new(Some("Wineget, The package manager for Wine"));
    parser.add_argument("-i", "--install", "Program you want to install",
                        false, None, ArgumentType::STR);
    parser.add_argument("-r", "--repo", "Repository where you want to search application, if not specified, wineget search in default",
                        false, None, ArgumentType::STR);
    parser.add_argument("-s", "--search", "Search in repository the application that you want",
                        false, None, ArgumentType::STR);
    parser.add_argument("-l", "--list", "List all applications in the repository",
                        false, None, ArgumentType::STR);
    parser.add_argument("-c", "--clear", "Clear downloads folder, where the installation script are downloaded",
                        false, None, ArgumentType::STR);
    parser.add_argument("-o", "--open", "Open an executable in Selected application prefix", 
                        false, None, ArgumentType::STR);
    parser.add_argument("-a", "-app", "App which want to execute in wineprefix, need also -o argument", 
                        false, None, ArgumentType::STR);

    // Check if there is any argument passed, else the program won't panic
    let argstemp = parser.parse_args();
    if argstemp.is_none() {
        exit(1);
    }
    // Parse command line arguments
    let args = parser.parse_args().unwrap();
    let appname = args.get("install").unwrap().get_str();
    let mut repo = args.get("repo").unwrap().get_str();
    let search = args.get("search").unwrap().get_str();
    let list = args.get("list").unwrap().get_str();
    let clear = args.get("clear").unwrap().get_str();
    let open = args.get("open").unwrap().get_str();
    let app = args.get("app").unwrap().get_str();
    // Check if user want to use a custom repository
    if repo == "" {
        // Default repository
        repo = "https://raw.githubusercontent.com/Windows-On-Linux/Repo/main/repository.json".to_string();
    }
    if !list.is_empty(){
        println!("{}", repo::update_repo(&repo).await);
        exit(0);
    }
    if !search.is_empty(){
        let repo_filter = search::search_repo(repo, search.to_lowercase()).await;
        println!("{:?}", repo_filter);
        exit(0);
    }
    if !clear.is_empty(){
        // Get Download dir
        let dir = dirs::home_dir().unwrap().as_path().join("wol").join("Downloads");
        // Remove Download dir
        println!("Removing old dir");
        fs::remove_dir_all(&dir).unwrap();
        // Create Download dir without all old files
        println!("Creating new clean download dir");
        fs::create_dir_all(&dir).unwrap();
        println!("Clear complete successfully");
        exit(0);
    }

    if !open.is_empty(){
        if !app.is_empty(){
            // Open an executable in selected application prefix
            open::open_app_in_prefix(open, app);
        }else{
            println!("Please specify application to run in prefix");
        }
        exit(0);
    }

    // If -h or other parameters aren't specified, we show a welcome message
    if appname.is_empty() {
        println!("Welcome to Wineget, the package manager for Wine, to install a program type Wineget -i name of application that you want to install");
        exit(0);
    }


    // Downloading repository
    println!("Updating repository...");
    let repos = repo::update_repo(&repo).await;
    parser::search_and_install(repos, appname, &clone::clonerepo);
    // If program don't exit first, the program isn't found in repository
    println!("Program not found in repository");
}



