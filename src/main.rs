#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
      .setup(|app| {
          // Obter a janela principal
          let main_window = app.get_window("main").unwrap();

          // Ler o remove_ad.js
          let js_script = include_str!("../remove_ad.js");

          // Injetar o JS
          main_window.eval(js_script).unwrap();

          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("Deu ruim, mestre!");
}
