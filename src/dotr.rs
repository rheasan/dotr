use std::fs::{create_dir_all, copy, remove_file, write, remove_dir_all, read};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct DotFile {
    name: String,
    desc: String,
    path: String,
    is_symlink: bool
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DotrData {
    remote: Option<String>,
    files: Vec<DotFile>
}

pub fn add(src: &Path, dest: &Path, is_symlink: bool, name: Option<String>, desc: Option<String>){
    

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

    let data = DotFile {
        name: name.unwrap_or_default(),
        desc: desc.unwrap_or_default(),
        path: dest.join(dotfile_name).to_str().unwrap().to_string(),
        is_symlink
    };

    if dest.join(dotfile_name).try_exists().unwrap() {
        println!("File already exists at {:?}. Replace? y/n", dest);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_ascii_lowercase();

        // TODO: find better way to handle this (?)
        if input == "y" || input == "yes" {
            remove_file(dest.join(dotfile_name)).expect("failed to remove file");
            remove_data(&data);
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
        write_data(&data);
    }
    else {
        copy(src, dest.join(dotfile_name)).expect(format!("failed to copy dotfile from {:?}", src).as_str());
        write_data(&data);
    }

}

fn remove_data(data: &DotFile) {
    let home_dir = std::env::var("HOME").expect("unable to read $HOME, data not saved");
    let config_path = PathBuf::from(home_dir).join(".dotr/dotr.json");
    let saved_data = read(&config_path).expect("Unable to read data");

    let mut dotr_data = serde_json::from_slice::<DotrData>(&saved_data).unwrap();
    
    dotr_data.files.remove(
        dotr_data.files.iter()
            .position(|f| *f.path == *data.path)
            .unwrap()
    );

    let serialized = serde_json::to_string::<DotrData>(&dotr_data).unwrap();
    write(&config_path, serialized).unwrap();

}

fn write_data(data: &DotFile){
    let home_dir = std::env::var("HOME").expect("unable to read $HOME, data not saved");
    let config_path = PathBuf::from(home_dir).join(".dotr/dotr.json");
    let saved_data = read(&config_path).expect("Unable to read data");

    let mut dotr_data = serde_json::from_slice::<DotrData>(&saved_data).unwrap();
    dotr_data.files.push(data.clone());
    let serialized = serde_json::to_string::<DotrData>(&dotr_data).unwrap();
    write(&config_path, serialized).unwrap();
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

pub fn remote_push(){
    todo!()
}

pub fn remote_set_url(url: String){
    let home_dir = std::env::var("HOME").expect("unable to read $HOME, data not saved");
    let config_path = PathBuf::from(home_dir).join(".dotr/dotr.json");
    let saved_data = read(&config_path).expect("Unable to read data");

    let mut serialized = serde_json::from_slice::<DotrData>(&saved_data).unwrap();
    if serialized.remote.is_some(){
        println!("A remote already exists: {}. Replace y/n?", serialized.remote.unwrap());
        let mut option = String::new();
        std::io::stdin().read_line(&mut option).unwrap();
        option = option.trim().to_lowercase();

        if option == "yes" || option == "y" {
        }
        else if option == "no" || option == "n" {
            exit(0);
        }
        else {
            eprintln!("unknown option");
            exit(1);
        }
    }

    if !is_valid_repo(&url){
        eprintln!("The remote repo: {} is not a valid git repository", url);
        exit(1);
    }

    serialized.remote = Some(url);
    let json = serde_json::to_string::<DotrData>(&serialized).unwrap();
    write(&config_path, json).expect("failed to write data");
}

fn is_valid_repo(url: &String) -> bool {
    // calls git ls-remote to probe the remote repo
    // the command will return 0 if it exists and 128 if it doesnt
    let exit_status = Command::new("sh")
    .arg("-c")
    .arg(format!("git ls-remote {}", url))
    .output()
    .expect("failed to run git")
    .status
    .code().unwrap();

    match exit_status {
        0 => {
            return true;
        }
        128 => {
            return false;
        }
        _ => {
            unreachable!();
        }
    }
}

