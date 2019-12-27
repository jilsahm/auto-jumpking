use std::{
    env::var,
    fs::{create_dir,copy,read_dir},
    path::{Path, PathBuf},
};

const CONFIG_FOLDER: (&'static str, &'static str) = ("config", ".cfg");
const LEVEL_FOLDER: (&'static str, &'static str) = ("level", ".lvl");
const RESOURCE_FOLDER: &'static str = "resources";

fn create_directories(location: &Path) {
    vec![CONFIG_FOLDER.0.clone(), LEVEL_FOLDER.0.clone()]
    .iter()
    .map(|dir| {
        print!("Creating folder {}... ", dir);
        let mut path = PathBuf::from(location);
        path.push(dir);
        path
    })
    .for_each(|dir| match create_dir(dir) {
        Err(what) => println!("{}", what),
        _ => println!("ok"),
    });
}

fn fill_directories(source: &Path, location: &Path) {
    vec![CONFIG_FOLDER.clone(), LEVEL_FOLDER.clone()]
    .iter()
    .for_each(|(folder, filter)| {
        let mut from = PathBuf::from(source);
        let mut to = PathBuf::from(location);
        from.push(RESOURCE_FOLDER);
        to.push(folder);
        copy_files(&from, &to, filter);
    });
}

fn copy_files(from: &Path, to: &Path, filter: &str) {
    match read_dir(from) {
        Ok(files) => {
            files
                .filter(|file| file.is_ok())
                .map(|file| file.unwrap().file_name().to_str().unwrap_or_else(|| "").to_string())
                .filter(|file| file.contains(filter))
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
    let output = PathBuf::from(var("OUT_DIR").unwrap());
    println!("Output dir is: {:?}", output);
    create_directories(&output);
    fill_directories(&PathBuf::from(env!("CARGO_MANIFEST_DIR")), &output);
}