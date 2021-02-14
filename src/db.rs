use crate::utils::{copy_dir_all, list_all_folders};
use std::env::set_current_dir;
use std::error::Error;
use std::fs::{create_dir, remove_dir_all, remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
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

pub fn create_env_file(
    root: &Path,
    db_username: String,
    db_password: String,
    db_name: String,
    port: String,
) -> Result<(), Box<dyn Error>> {
    let env_file_path = root.join("existing_dbs").join(db_name.clone()).join(".env");
    // Delete env-file:
    remove_file(&env_file_path)?;

    let mut file = File::create(env_file_path).map_err(|_| "Could not create .env file")?;
    let env_content = format!(
        "DB_NAME = {}\nDB_USER = {}\nDB_PASSWORD = {}\nDB_PORT = {}",
        db_name, db_username, db_password, port
    );
    println!("Env-file : {}", env_content);
    file.write_all(env_content.as_bytes())?;
    Ok(())
}

pub fn create_db(root: &Path, name: String, db: DB) -> Result<(), String> {
    let target_path = root.join("existing_dbs").join(name);
    if target_path.exists() {
        return Err("DB already exists".to_string());
    }
    if !root.join("existing_dbs").exists() {
        create_dir(root.join("existing_dbs"))
            .map_err(|_| "Could not create db folder".to_string())?
    }
    let src_path = match db {
        DB::MONGO => root.join("mongo"),
        DB::POSTGRES => root.join("postgres"),
    };
    println!(
        "Initialising new db folder {:?} by db sample folder {:?}",
        target_path, src_path
    );
    copy_dir_all(src_path, &target_path)
        .map_err(|err| format!("Could not copy mongodb setup files: {}", err))
}

pub fn start_docker_compose(path: &Path) -> Result<(), String> {
    println!("start docker-compose : {:?}", path);
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

pub fn get_default_port(db: DB) -> String {
    let port = match db {
        DB::MONGO => "27017",
        DB::POSTGRES => "5432",
    };
    port.to_string()
}

pub fn delete_db(root: &Path, db_name: String) -> Result<(), String> {
    delete_container(db_name.clone())?;
    remove_dir_all(root.join("existing_dbs").join(db_name))
        .map_err(|_| "Could not delete DB".to_string())
}

pub fn delete_container(db_name: String) -> Result<(), String> {
    Command::new("docker")
        .args(vec!["stop", &db_name])
        .spawn()
        .map_err(|_| "Could not stop container")?;
    Command::new("docker")
        .args(vec!["rm", &db_name])
        .spawn()
        .map_err(|_| "Could not delete container")?;
    Ok(())
}

pub fn stop_db(db_name: String) -> Result<(), String> {
    println!("Stopping DB : {}", db_name);
    Command::new("docker")
        .args(vec!["stop", &db_name])
        .spawn()
        .map_err(|_| "Could not stop DB".to_string())?;
    Command::new("docker")
        .args(vec!["rm", &db_name])
        .spawn()
        .map_err(|_| "Could not stop DB".to_string())?;
    Ok(())
}

pub fn get_running_dbs(root: &Path) -> Result<Vec<String>, String> {
    let db_names = get_existing_dbs(root);
    let output = Command::new("docker")
        .arg("ps")
        .output()
        .map_err(|_| "Could not check DBs".to_string())?;
    let result = String::from_utf8_lossy(&output.stdout);
    return Ok(db_names
        .into_iter()
        .filter(|name| result.lines().any(|line| line.ends_with(name)))
        .collect::<Vec<String>>());
}
