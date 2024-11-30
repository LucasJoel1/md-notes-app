// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've beens greeted from Rust!", name)
}

#[tauri::command]
fn replace_char(_str: &str, _index: usize, _char: char) -> String {
    let mut new_string = String::new();

    if _index >= _str.len() {
        new_string = _str.to_string();
        new_string.push(_char);
    } else if _char == '\0' {
        let (start, end) = _str.split_at(_index);
        let (_, removed_first_index) = end.split_at(1);
        new_string.push_str(start);
        new_string.push_str(removed_first_index);
    } else {
        let (start, end) = _str.split_at(_index);
        new_string.push_str(start);
        new_string.push(_char);
        new_string.push_str(end);
    }
    new_string
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, replace_char])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
