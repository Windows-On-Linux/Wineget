# Wineget

## The package manager for Wine

This is the repository of Wineget project, a package manager for Wine.

You can install new applications using the following command: Wineget -i Appname

Wineget is very lightweight, the size of package is only 1,43Mb and don't require any dependencies!

# Build and run

```console
cargo run -- -i Appname # Run Wineget
cargo install cargo-deb # Install cargo-deb to produce .deb file
cargo deb # Build .deb file
```

# New applications support

The script to install Wine app is very simple, it's a bash script like that:
https://github.com/Windows-On-Linux/Ableton-On-Linux
You must create a script that create a Wineprefix in ~/wol and then install dependencies and app
In the repository
If you want to add a new application in repository, open an issue or a pull request in following repository:
https://github.com/Windows-On-Linux/Repo

# Custom repository

If you want to use a custom repository, you can add the following argument to Wineget command: wineget -i Appname -r RepoURL