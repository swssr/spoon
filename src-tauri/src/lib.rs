// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    match name.parse::<i64>() {
        Ok(n) => {
            format!(
                "Hello, {}! You've been greeted from Rust! You're grown {bro}",
                name,
                bro = 20
            )
        }
        Err(e) => format!("Bathong!"),
    }
}

#[tauri::command]
fn file(file_path: &str) {
    //println!("Opening {} on path", file_path);
    let path_name = "/Documents";
    format!("Hello, {}! Path: {path_name}", file_path);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
