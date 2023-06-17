#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod io;

#[tauri::command]
fn create_project(info: String) -> String{
    io::create_project(info)
}

#[tauri::command]
fn get_solution_list() -> String{
  io::get_solution_list()
}

#[tauri::command]
fn get_csproj_list(info: String) -> String{
  io::get_project_list(info)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          create_project,
          get_solution_list,
          get_csproj_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

