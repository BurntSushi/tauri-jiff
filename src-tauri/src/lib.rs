use jiff::{Timestamp, ToSpan};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let time: Timestamp = "2024-07-11T01:14:00Z".parse().unwrap();
    let zoned = time.intz("America/New_York").unwrap().checked_add(1.month().hours(2)).unwrap();
    println!("actual: {}", zoned);
    println!("expected: 2024-08-10T23:14:00-04:00[America/New_York]");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
