use std::{
    fs::{self, read},
    io::Result,
};

fn main() {
    println!("Coding ...");

    let read_dirs = read_toml_workspace();
    let config = check_toml_workspace().expect("err: load toml err");
    let members = config.workspace.unwrap().members.unwrap();

    let missing_dirs: Vec<&String> = read_dirs
        .iter()
        .filter(|dir| !members.contains(dir))
        .collect();
    println!("{:#?}", &missing_dirs);
}
extern crate regex;
extern crate serde;
extern crate toml;

use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    workspace: Option<WorkspaceConfig>,
}
#[derive(Debug, Deserialize)]
struct WorkspaceConfig {
    members: Option<Vec<String>>,
}

fn check_toml_workspace() -> Result<Config> {
    let toml_string = fs::read_to_string("./Cargo.toml").unwrap();
    let decoded: Config = toml::from_str(&toml_string).unwrap();
    Ok(decoded)
}

fn read_toml_workspace() -> Vec<String> {
    let paths = fs::read_dir("./").unwrap();

    let re = Regex::new(r"_\d+_.+").unwrap();

    let mut read_path: Vec<String> = Vec::new();

    for path in paths {
        // println!("Name: {}", path.unwrap().path().display());

        let str = path.unwrap().path().into_os_string();
        match str.into_string() {
            Ok(v) => {
                let ref_v = &(v.clone());
                match re.is_match(ref_v) {
                    true => {
                        read_path.push(String::from(&v[2..v.len()]));
                    }
                    false => {}
                }
            }
            Err(e) => {
                println!("err: {}", e.to_string_lossy())
            }
        }
    }
    read_path
}
