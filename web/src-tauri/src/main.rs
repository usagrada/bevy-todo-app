#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::fs;

#[tauri::command]
async fn write_report(app: tauri::AppHandle) -> Result<(), String> {
    let home_dir = dirs::home_dir().unwrap();
    let todo_dir = home_dir.join(".todo-app");
    if todo_dir.exists() {
        println!("{:?}", todo_dir);
    } else {
        fs::create_dir_all(todo_dir).unwrap();
        println!("create dir");
    }
    println!("write report!!");
    let app_dir = app.path_resolver().app_dir().expect("failed to get app dir");
    let report_path = app_dir.join("report.txt");
    std::fs::write(&report_path, "the file content")
    .map_err(|e| e.to_string());
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_report])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
