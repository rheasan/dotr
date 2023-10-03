use std::fs::{create_dir_all, copy, remove_file, write, remove_dir_all};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::exit;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct DotFile {
    name: String,
    desc: String,
    path: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct DotrData {
    remote: Option<String>,
    files: Vec<DotFile>
}

pub fn add(src: &Path, dest: &Path, is_symlink: bool){

    if src.is_dir() {
        eprintln!("Source is a directory");
        exit(1);
    }

    if !src.try_exists().expect("Unable to read source") {
        eprintln!("Source file does not exist");
        exit(1);
    }


    check_config_exists();
    let home_dir = std::env::var("HOME").unwrap();
    let config_dir = PathBuf::from(home_dir).join(".dotr");
    let dotfile_name = src.file_name().unwrap().to_str().unwrap();
    let dotfile_path = config_dir.join(dotfile_name);

    if dest.join(dotfile_name).try_exists().unwrap() {
        println!("File already exists at {:?}. Replace? y/n", dest);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_ascii_lowercase();

        // TODO: find better way to handle this (?)
        if input == "y" || input == "yes" {
            remove_file(dest.join(dotfile_name)).expect("failed to remove file");
        }
        else if input == "n" || input == "no" {
            return;
        }
        else {
            eprintln!("unknown option");
            exit(1);
        }
    }

    if is_symlink {
        copy(src, &dotfile_path).expect(format!("failed to copy dotfile from {:?}", src).as_str());
        symlink(dotfile_path, dest.join(dotfile_name)).expect("failed to create symlink");
    }
    else {
        copy(src, dest.join(dotfile_name)).expect(format!("failed to copy dotfile from {:?}", src).as_str());
    }

}

fn check_config_exists(){
    let home_dir = std::env::var("HOME").expect("unable to read $HOME");
    let config_path = PathBuf::from(home_dir).join(".dotr");
    let config_file_path = config_path.join("dotr.json");
    if !config_path.try_exists().expect("Unable to read dotr config path") {
        eprintln!("Dotr config path does not exist");
        eprintln!("run `dotr init` to configure");
        exit(1);
    }

    if !config_file_path.try_exists().expect("Unable to read dotr config") {
        eprintln!("Dotr config file does not exist");
        eprintln!("run `dotr init` to configure");
        exit(1);
    }
}

pub fn init(){
    let home_dir = std::env::var("HOME").expect("unable to read $HOME");
    let config_path = PathBuf::from(home_dir).join(".dotr");


    if !config_path.try_exists().expect("Unable to create dotr config path") {
        create_dir_all(&config_path).expect("failed to create dotr directory");
    }
    else {
        println!("Config directory already exists.");
        println!("Deleting it will erase all config data. (Files added by symlinks will be lost)");
        println!("Proceed? y/n?");

        let mut option = String::new();
        std::io::stdin().read_line(&mut option).unwrap();
        option = option.trim().to_lowercase();

        if option == "yes" || option == "y" {
            remove_dir_all(&config_path).expect("failed to remove existing config directory");
            create_dir_all(&config_path).expect("failed to create dotr directory");
        }
        else if option == "no" || option == "n" {
            exit(0);
        }
        else {
            eprintln!("unknown option");
            exit(1);
        }
    }


    let data = DotrData {
        remote: None,
        files: Vec::new()
    };
    let serialized = serde_json::to_string(&data).unwrap();

    write(config_path.join("dotr.json"), &serialized).expect("failed to write dotr config");
}