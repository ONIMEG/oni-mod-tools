#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod io;

#[tauri::command]
fn create_project(info: String) -> String{
    io::create_project(info)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

