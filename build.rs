use std::{
    env::var,
    fs::{create_dir,copy,read_dir},
    path::{Path, PathBuf},
};

const CONFIG_FOLDER: (&'static str, &'static str) = ("config", ".cfg");
const LEVEL_FOLDER: (&'static str, &'static str) = ("level", ".lvl");
const RESOURCE_FOLDER: &'static str = "resources";

fn create_resource_directory(location: &Path) {
    print!("Creating resource folder ... ");
    let mut path = PathBuf::from(location);
    path.push(RESOURCE_FOLDER);
    match create_dir(path) {
        Err(what) => println!("{}", what),
        _ => println!("ok"),
    }
}

fn fill_resource_directory(source: &Path, location: &Path) {
    let mut from = PathBuf::from(source);
    let mut to = PathBuf::from(location);
    from.push(RESOURCE_FOLDER);
    to.push(RESOURCE_FOLDER);
    copy_files(&from, &to);
}

fn copy_files(from: &Path, to: &Path) {
    match read_dir(from) {
        Ok(files) => {
            files
                .filter(|file| file.is_ok())
                .map(|file| file.unwrap().file_name().to_str().unwrap_or_else(|| "").to_string())
                .map(|file| {
                    let mut from = PathBuf::from(from);
                    let mut to = PathBuf::from(to);
                    from.push(file.clone());
                    to.push(file.clone());
                    print!("Copying {} from {:?} to {:?}... ", file, from, to);
                    copy(from, to)
                })
                .for_each(|result| {
                    match result {
                        Ok(_) => println!("ok"),
                        Err(what) => println!("{}", what),
                    }
                })
        }
        Err(what) => println!("Could not copy from {:?} to {:?} because: {}", from, to, what),
    }
}

fn main() {
    //let output = PathBuf::from(var("OUT_DIR").unwrap());
    let mut output = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    output.push("target/debug");
    println!("Output dir is: {:?}", output);
    create_resource_directory(&output);
    fill_resource_directory(&PathBuf::from(env!("CARGO_MANIFEST_DIR")), &output);
}