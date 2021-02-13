use crate::utils::{copy_dir_all, list_all_folders};
use std::env::set_current_dir;
use std::fs::create_dir;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum DB {
    MONGO,
    POSTGRES,
}

pub fn get_existing_dbs(root: &Path) -> Vec<String> {
    let folders = list_all_folders(&root.join("existing_dbs").into_boxed_path());
    folders
        .iter()
        .filter(|p| p.is_dir())
        .map(|p| {
            p.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_owned())
                .unwrap()
        })
        .collect()
}

pub fn create_db(root: &Path, name: String, db: DB) -> Result<(), String> {
    let target_path = root.join(name);
    if target_path.exists() {
        return Err("DB already exists".to_string());
    }
    create_dir(&target_path).map_err(|_| "Could not create folder".to_string())?;
    match db {
        DB::MONGO => {
            let src = root.join("mongo");
            copy_dir_all(src, &target_path)
                .map_err(|_| "Could not copy mongodb setup files".to_string())
        }
        DB::POSTGRES => {
            let src = root.join("postgres");
            copy_dir_all(src, &target_path)
                .map_err(|_| "Could not copy postgres setup files".to_string())
        }
    }
    // TODO modify envs
}

pub fn start_docker_compose(path: &Path) -> Result<(), String> {
    set_current_dir(path).map_err(|_| "DB-folder does not exist".to_string())?;
    let output = Command::new("docker-compose")
        .arg("up")
        .spawn()
        .map_err(|_| "Could not start docker-compose")
        .and_then(|p| p.stdout.ok_or("Could not output"))?;
    let reader = BufReader::new(output);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    Ok(())
}
