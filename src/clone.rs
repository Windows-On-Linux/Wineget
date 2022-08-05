use std::path::PathBuf;
use git2::Repository;

pub fn clonerepo(url: &str, _name: &str, dir: PathBuf) -> bool{
    let repo = match Repository::clone(url, dir.clone()) {
        Ok(_repo) => true,
        Err(_e) => false,
    };
    if !repo{
        let error = match Repository::clone(url, dir) {
            Ok(_repo) => git2::Error::new(git2::ErrorCode::GenericError, git2::ErrorClass::None, "No error occurred, ignore this message"),
            Err(e) => e,
        };
        println!("{}", error.message());
    };
    repo
}