#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![increment, decrement])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn increment(value: i32) -> i32 {
   value + 1
}

#[tauri::command]
fn decrement(value: i32) -> i32 {
   value - 1
}