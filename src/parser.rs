use std::{process::exit, path::PathBuf};
use serde_json::Value;
use crate::installer;


pub fn search_and_install(repos: Value, appname: String, clonerepo: &dyn Fn(&str, &str, PathBuf) -> bool){
    for i in 0..repos.as_array().unwrap().len(){
    let app_name_from_repo = repos.get(i).unwrap()["CliName"].as_str().unwrap();
    // Search for application
    if  app_name_from_repo == appname {
        let url = repos.get(i).unwrap()["Repository"].as_str().unwrap();
        let program_name = repos.get(i).unwrap()["Path"].as_str().unwrap();
        // We find the program
        println!("\nFind {} in {} Git repository. \nAre you sure to install this program?[Y/N]", app_name_from_repo, url);
        // Require confirm by user
        let mut confirm = String::new();
        std::io::stdin().read_line(&mut confirm).unwrap();
        // If user accept the installation, clone repository via git2 for Rust library and run script
        if confirm.to_lowercase().contains("y") {
            println!("Cloning repository...");
            let name = program_name.split("/").last().unwrap();
            // Create PathBug for ~/wol/downloads, the folder where WOL apps downloads script
            let dir = dirs::home_dir().unwrap().as_path().join("wol").join("Downloads").join(name);
            if clonerepo(url, name, dir.clone()){
                println!("Clone finished successfully, starting the installation script");
                // Run script
                installer::install(dir.join("build.sh"));
                // The installation script is started, close Wineget
                exit(0);
            }else{
                panic!("Error during cloning repository");
            };
        }else{
            // N in pressed
            println!("Abort installation");
            exit(0);
        };
    };
}
}
