use std::fs::{create_dir_all, copy, remove_file};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::exit;

pub fn add(src: &Path, dest: &Path){

    if src.is_dir() {
        eprintln!("Source is a directory");
        exit(1);
    }

    if !src.try_exists().expect("Unable to read source") {
        eprintln!("Source file does not exist");
        exit(1);
    }


    make_config_dir();
    let home_dir = std::env::var("HOME").unwrap();
    let config_dir = PathBuf::from(home_dir).join(".dotr");
    let dotfile_name = src.file_name().unwrap().to_str().unwrap();
    let dotfile_path = config_dir.join(dotfile_name);

    copy(src, &dotfile_path).expect(format!("failed to copy dotfile from {:?}", src).as_str());

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

    symlink(dotfile_path, dest.join(dotfile_name)).expect("failed to create symlink");

}

fn make_config_dir(){
    let home_dir = std::env::var("HOME").expect("unable to read $HOME");
    let config_path = PathBuf::from(home_dir).join(".dotr");
    if !config_path.try_exists().expect("Unable to create dotr config path") {
        create_dir_all(config_path).expect("failed to create dotr directory");
    }
}