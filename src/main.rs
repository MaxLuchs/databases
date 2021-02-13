use databases::db::{create_db, start_docker_compose};
use databases::menu::{get_user_input, show_menu, UISelection};
use std::env::current_dir;

pub fn main() {
    let root = current_dir().unwrap();
    let user_input = show_menu().and_then(|result| result.ok_or("Nothing selected".to_string()));
    if let Err(msg) = user_input {
        println!("{}", msg);
        return;
    }
    if let Ok(result) = user_input {
        match result {
            UISelection::CreateDB { db_type } => {
                let new_db_name = get_user_input("New DB-Name: ".to_string());
                create_db(&root.join("existing_dbs"), new_db_name.to_string(), db_type).unwrap();
                start_docker_compose(&root.join(new_db_name));
            }
            UISelection::StartDB { db_name } => {
                start_docker_compose(&root.join(&db_name));
            }
        }
    }
}
