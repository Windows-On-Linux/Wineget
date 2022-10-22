use std::{fs, process::exit};
use dirs::home_dir;
use std::process::Command;

pub fn open_app_in_prefix(name: String, app: String){

    let path = home_dir().unwrap().as_path().join("wol");
    let mut all_apps: Vec<String> = Vec::new();
    for file in fs::read_dir(path.clone()).unwrap() {
        let app_path = file.unwrap().path();
        if app_path.as_path().join("name.txt").exists(){

            let contents = fs::read_to_string(app_path.as_path().join("name.txt"))
            .expect("Should have been able to read the file");

            all_apps.push(contents.clone().replace("\n", ""));  // Sanitize the string

            if contents.contains(&name) {

                println!("Found {contents} in installed applications!");
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("WINEPREFIX={:?} wine {}", app_path.as_path(), app))
                    .output()
                    .expect("Failed to execute application in Wineprefix");
                exit(0);    // We have nothing to do now, we can exit safely

            }else{
                // Exit not successfully so we must display all applications
                println!("{contents} not found in installed applications!, this is all the installed applications:\n");
                println!("{:?}", all_apps);

            }
        }

    }
}