#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#![allow(unused_must_use)]
use notify_rust::Notification;


fn main() {
  Notification::new()
    .timeout(3000)
    .summary("notification with sound")
    .show();
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
