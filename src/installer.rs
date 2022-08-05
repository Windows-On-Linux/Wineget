use std::path::PathBuf;
use std::process::Command;

pub fn install(path: PathBuf){
    Command::new("/bin/bash")
        .arg(path.as_os_str())
        .spawn()
        .expect("Failed to execute installation script");
}