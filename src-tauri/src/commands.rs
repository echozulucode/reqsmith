// Tauri IPC commands for frontend communication

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ReqSmith.", name)
}

// Future commands will be added here:
// - open_reqif
// - save_reqif
// - get_requirements
// - search
// etc.
