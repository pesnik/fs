// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{fs, io};

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    match folders(name) {
        Ok(folder_list) => Ok(folder_list),
        Err(e) => Err(format!("Error reading directory: {}", e)),
    }
}

fn folders(path: &str) -> io::Result<String> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let mut result = String::new();
    for entry in entries {
        if let Some(path_str) = entry.to_str() {
            result.push_str(path_str);
            result.push('\n');
        }
    }
    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
