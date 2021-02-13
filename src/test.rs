use databases::menu::show_menu;

pub fn main() {
    let user_input = show_menu().and_then(|result| result.ok_or("Nothing selected".to_string()));
    if let Err(msg) = user_input {
        println!("{}", msg);
        return;
    }
    if let Ok(result) = user_input {
        println!("result : {:?}", result);
    }
}
