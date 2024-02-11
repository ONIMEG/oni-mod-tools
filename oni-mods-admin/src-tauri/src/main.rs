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
fn add_new_project(sln: String, csproj: String) -> String{
  io::add_new_project(sln, csproj)
}

#[tauri::command]
fn refresh_version() -> String{io::refresh_version()}

#[tauri::command]
fn get_config_info() -> String{io::get_config_info()}

#[tauri::command]
fn update_config_info(info: String) -> String{io::update_config_info(info)}

#[tauri::command]
fn git_create_new_repo(info: String) -> String{
    io::io_create_new_repo(info)
}

#[tauri::command]
fn store_current_proj(curr_proj_info: String) -> String {
    io::io_store_current_project(curr_proj_info)
}

#[tauri::command]
fn read_current_proj_buffer() -> String{
    io::read_current_project_buffer()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_project,
            get_solution_list,
            add_new_project,
            refresh_version,
            get_config_info,
            update_config_info,
            git_create_new_repo,
            store_current_proj,
            read_current_proj_buffer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

