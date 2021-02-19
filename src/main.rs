use databases::db::{
    create_db, create_env_file, delete_container, delete_db, get_default_port,
    start_docker_compose, start_sqlite3_db, stop_db, DB,
};
use databases::menu::{show_menu, UISelection};
use eyre::*;
use rustyline::Editor;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "db")]
struct CommandLineArgs {
    /// path to local clone of git-repository: https://github.com/MaxLuchs/databases
    #[structopt(short, long, env = "DB_DIR")]
    dir: Option<PathBuf>,
}

pub fn main() -> Result<()> {
    let args: CommandLineArgs = CommandLineArgs::from_args();
    let root = args
        .dir
        .and_then(|db_dir| if db_dir.exists() { Some(db_dir) } else { None })
        .ok_or(eyre!("No valid DB directory given"))?;
    println!("Using project dir for DBs: {}", root.display());
    let user_input = show_menu(&root)?;

    //println!("user input : {:?}", &user_input);
    if let Some(result) = user_input {
        match result {
            UISelection::StartDB { db_name } => {
                if root
                    .join("existing_dbs")
                    .join(&db_name)
                    .join(".sqlite3")
                    .exists()
                {
                    start_sqlite3_db(&root, db_name.clone())?;
                } else {
                    start_docker_compose(&root.join("existing_dbs").join(&db_name))?;
                }
            }
            UISelection::CreateDB { db_type } => {
                let mut editor = Editor::<()>::new();

                // db name:
                let db_input = editor.readline("New DB-Name > ").unwrap();
                let new_db_name = db_input.trim().to_string();
                create_db(&root, new_db_name.clone(), db_type).unwrap();

                if db_type != DB::SQLITE3 {
                    let default_port = get_default_port(db_type);
                    let message = format!("DB-Port (optional, default: {}) > ", &default_port);
                    let port_input = editor.readline(&message).unwrap();
                    let new_port = if port_input.trim().is_empty() {
                        default_port
                    } else {
                        port_input.trim().to_string()
                    };

                    let user_input = editor
                        .readline(&format!(
                            "User (optional, default: {}) > ",
                            std::env::var("USER").unwrap()
                        ))
                        .unwrap();
                    let new_user = if user_input.trim().is_empty() {
                        std::env::var("USER").unwrap()
                    } else {
                        user_input.trim().to_string()
                    };

                    let password_input = editor
                        .readline("Password (optional, default: test) > ")
                        .unwrap();
                    let new_password = if password_input.trim().is_empty() {
                        "test".to_string()
                    } else {
                        password_input.trim().to_string()
                    };

                    create_env_file(&root, new_user, new_password, new_db_name.clone(), new_port)?;
                    delete_container(new_db_name.clone())?;
                    start_docker_compose(&root.join("existing_dbs").join(&new_db_name))?;
                } else {
                    start_sqlite3_db(&root, new_db_name.clone())?;
                }
            }
            UISelection::DeleteDB { db_name } => {
                delete_db(&root, db_name.clone()).map(|_| println!("DB {} deleted!", &db_name))?;
            }
            UISelection::StopDB { db_name } => {
                stop_db(db_name.clone()).map(|_| println!("DB {} stopped!", &db_name))?;
            }
        }
    } else {
        println!("User aborted selection");
    };
    Ok(())
}
