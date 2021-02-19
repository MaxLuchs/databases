use crate::utils::{copy_dir_all, list_all_folders};
use eyre::*;
use std::env::set_current_dir;
use std::fs::{create_dir, remove_dir_all, remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum DB {
    MONGO,
    POSTGRES,
    SQLITE3,
}

pub fn get_existing_dbs(root: &Path) -> Result<Vec<String>> {
    let folders = list_all_folders(&root.join("existing_dbs").into_boxed_path())?;
    let results = folders
        .iter()
        .filter(|p| p.is_dir())
        .map(|p| {
            p.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_owned())
        })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect::<Vec<String>>();
    Ok(results)
}

pub fn create_env_file(
    root: &Path,
    db_username: String,
    db_password: String,
    db_name: String,
    port: String,
) -> eyre::Result<()> {
    let env_file_path = root.join("existing_dbs").join(db_name.clone()).join(".env");
    // Delete env-file:
    remove_file(&env_file_path)?;

    let mut file = File::create(env_file_path)?;
    let env_content = format!(
        "DB_NAME = {}\nDB_USER = {}\nDB_PASSWORD = {}\nDB_PORT = {}",
        db_name, db_username, db_password, port
    );
    println!("Env-file : {}", env_content);
    file.write_all(env_content.as_bytes())?;
    Ok(())
}

pub fn create_db(root: &Path, name: String, db: DB) -> Result<()> {
    let target_path = root.join("existing_dbs").join(name);
    if target_path.exists() {
        return Err(eyre::eyre!("DB already exists"));
    }
    if !root.join("existing_dbs").exists() {
        create_dir(root.join("existing_dbs"))?
    }
    let src_path = match db {
        DB::MONGO => root.join("mongo"),
        DB::POSTGRES => root.join("postgres"),
        DB::SQLITE3 => root.join("sqlite3"),
    };
    println!(
        "Initialising new db folder {:?} by db sample folder {:?}",
        target_path, src_path
    );
    copy_dir_all(src_path, &target_path)?;
    Ok(())
}

pub fn start_sqlite3_db(root: &Path, db_name: String) -> Result<()> {
    set_current_dir(root.join(&"existing_dbs"))?;
    let output = Command::new("sqlite3").arg(&db_name).output()?;
    let (stdout, stderr) = (
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    if output.status.success() {
        println!(
            "Sqlite3-DB '{db_name}' successfully created: '{stdout}'",
            db_name = db_name,
            stdout = stdout
        );
        return Ok(());
    } else {
        let err_msg = format!(
            "Error could not create Sqlite3-DB '{db_name}': {stderr}",
            db_name = db_name,
            stderr = stderr
        );
        println!("{}", &err_msg);
        return Err(eyre!(err_msg.clone()));
    }
}

pub fn start_docker_compose(path: &Path) -> Result<()> {
    println!("start docker-compose : {:?}", path);
    set_current_dir(path)?;
    let output = Command::new("docker-compose")
        .arg("up")
        .spawn()
        .map(|p| p.stdout)?;
    let output_stream = output.ok_or(eyre!("No output stream"))?;
    let reader = BufReader::new(output_stream);
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
        DB::SQLITE3 => "",
    };
    port.to_string()
}

pub fn delete_db(root: &Path, db_name: String) -> Result<()> {
    delete_container(db_name.clone())?;
    remove_dir_all(root.join("existing_dbs").join(db_name))?;
    Ok(())
}

pub fn delete_container(db_name: String) -> Result<()> {
    Command::new("docker")
        .args(vec!["stop", &db_name])
        .spawn()?;
    Command::new("docker").args(vec!["rm", &db_name]).spawn()?;
    Ok(())
}

pub fn stop_db(db_name: String) -> Result<()> {
    println!("Stopping DB : {}", db_name);
    Command::new("docker")
        .args(vec!["stop", &db_name])
        .spawn()?;
    Command::new("docker").args(vec!["rm", &db_name]).spawn()?;
    Ok(())
}

pub fn get_running_dbs(root: &Path) -> Result<Vec<String>> {
    let db_names = get_existing_dbs(root)?;
    let output = Command::new("docker").arg("ps").output()?;
    let result = String::from_utf8_lossy(&output.stdout);
    return Ok(db_names
        .into_iter()
        .filter(|name| result.lines().any(|line| line.ends_with(name)))
        .collect::<Vec<String>>());
}
