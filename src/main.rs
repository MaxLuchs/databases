use databases::db::{
    create_db, create_env_file, delete_container, delete_db, get_default_port,
    start_docker_compose, stop_db,
};
use databases::menu::{show_menu, UISelection};
use rustyline::Editor;
use std::env::current_dir;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let root = current_dir().unwrap();
    let user_input = show_menu().and_then(|result| result.ok_or("Nothing selected".to_string()));
    if let Err(msg) = user_input {
        println!("{}", msg);
        return Ok(());
    }
    if let Ok(result) = user_input {
        match result {
            UISelection::CreateDB { db_type } => {
                let mut editor = Editor::<()>::new();

                // db name:
                let db_input = editor.readline("New DB-Name > ").unwrap();
                let new_db_name = db_input.trim().to_string();
                create_db(&root, new_db_name.clone(), db_type).unwrap();

                let port_input = editor.readline("DB-Port (optional) > ").unwrap();
                let new_port = if port_input.trim().is_empty() {
                    get_default_port(db_type)
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
            }
            UISelection::StartDB { db_name } => {
                start_docker_compose(&root.join("existing_dbs").join(&db_name))?;
            }
            UISelection::DeleteDB { db_name } => {
                delete_db(&root, db_name.clone()).map(|_| println!("DB {} deleted!", &db_name))?;
            }
            UISelection::StopDB { db_name } => {
                stop_db(db_name.clone()).map(|_| println!("DB {} stopped!", &db_name))?;
            }
        }
    };
    Ok(())
}
