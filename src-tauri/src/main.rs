#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      let _ = window.set_decorations(false);
      let _ = window.set_always_on_top(true);
      // Set the window to fullscreen
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
