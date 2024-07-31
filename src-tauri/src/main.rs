// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_message() -> String {
    format!("This is my message to you hu hu hu")
}

struct Count {
    pub current: u32
}

impl Count {
    pub fn add(&mut self) -> u32 {
        self.current += 1;
        self.current
    }
    pub fn remove(&mut self) -> u32 {
        self.current -= 1;
        self.current
    }
}

// #[tauri::command]
// fn count_up() -> u32 {
//     let counter = Count {
//         current: 0
//     };
//     return counter.current;
// }

const CS:Count = Count {current: 0};


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_message])
        .manage(CS)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
