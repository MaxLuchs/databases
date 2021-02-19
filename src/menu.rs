extern crate terminal_menu;

use self::terminal_menu::{
    activate, button, get_submenu, menu, scroll, selection_value, submenu, wait_for_exit,
};
use crate::db::{get_existing_dbs, get_running_dbs, DB};
use crate::menu::UISelection::{CreateDB, DeleteDB, StartDB, StopDB};
use eyre::Result;
use std::io::stdin;
use std::path::Path;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum UISelection {
    CreateDB { db_type: DB },
    StartDB { db_name: String },
    DeleteDB { db_name: String },
    StopDB { db_name: String },
}

#[derive(Debug)]
struct UISelectionInput {
    db_existing: String,
    db_new: String,
    db_delete: String,
    db_stop: String,
}

impl From<UISelectionInput> for Option<UISelection> {
    fn from(input: UISelectionInput) -> Option<UISelection> {
        //println!("input : {:?}", &input);
        let UISelectionInput {
            db_existing,
            db_new,
            db_delete,
            db_stop,
        } = input;
        match (
            db_existing.as_ref(),
            db_new.as_ref(),
            db_delete.as_ref(),
            db_stop.as_ref(),
        ) {
            ("none", "mongodb", "none", "none") => Some(CreateDB { db_type: DB::MONGO }),
            ("none", "sqlite3", "none", "none") => Some(CreateDB {
                db_type: DB::SQLITE3,
            }),
            ("none", "postgres", "none", "none") => Some(CreateDB {
                db_type: DB::POSTGRES,
            }),
            (db_name, "none", "none", "none") if db_name != "none" => Some(StartDB {
                db_name: db_name.to_string(),
            }),
            ("none", "none", db_name, "none") if db_name != "none" => Some(DeleteDB {
                db_name: db_name.to_string(),
            }),

            ("none", "none", "none", db_name) if db_name != "none" => Some(StopDB {
                db_name: db_name.to_string(),
            }),
            _ => None,
        }
    }
}

pub fn show_menu(root: &Path) -> Result<Option<UISelection>> {
    let menu = menu(vec![
        submenu(
            "Start existing DB",
            vec![
                scroll("Select DB", {
                    let mut options = get_existing_dbs(&root, false)?;
                    options.insert(0, "none".to_string());
                    options
                }),
                button("Exit"),
            ],
        ),
        submenu(
            "Create new DB",
            vec![
                scroll("Select DB-Type", {
                    let mut options = vec!["mongodb", "postgres", "sqlite3"];
                    options.insert(0, "none");
                    options
                }),
                button("Exit"),
            ],
        ),
        submenu(
            "Delete a DB",
            vec![
                scroll("Select DB", {
                    let mut options = get_existing_dbs(&root, true)?;
                    options.insert(0, "none".to_string());
                    options
                }),
                button("Exit"),
            ],
        ),
        submenu(
            "Stop a running DB",
            vec![
                scroll("Select DB", {
                    let mut options = get_running_dbs(&root)?;
                    options.insert(0, "none".to_string());
                    options
                }),
                button("Exit"),
            ],
        ),
        button("Exit"),
    ]);
    activate(&menu);
    wait_for_exit(&menu);

    // menus:
    let existing_menu = get_submenu(&menu, "Start existing DB");
    let new_menu = get_submenu(&menu, "Create new DB");
    let delete_menu = get_submenu(&menu, "Delete a DB");
    let stop_menu = get_submenu(&menu, "Stop a running DB");

    // values:
    let db_existing = selection_value(&existing_menu, "Select DB");
    let db_new = selection_value(&new_menu, "Select DB-Type");
    let db_delete = selection_value(&delete_menu, "Select DB");
    let db_stop = selection_value(&stop_menu, "Select DB");

    let ui_selection: Option<UISelection> = (UISelectionInput {
        db_new,
        db_existing,
        db_delete,
        db_stop,
    })
    .into();

    return Ok(ui_selection);
}

pub fn get_user_input(message: String) -> String {
    print!("{}", message.to_string());
    let input = stdin();
    let mut user_input = String::new();
    input.read_line(&mut user_input).unwrap();
    return user_input.trim().to_string();
}
