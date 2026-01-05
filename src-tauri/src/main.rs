#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use std::env;

fn main() {
  // Proxy 
  let default_proxy = "http://187.86.59.122:80";
  
  // Permite sobrescrever via variável de ambiente
  let proxy = env::var("WHATSAPP_PROXY").unwrap_or(default_proxy.to_string());
  
  // Configura o WebView2 para usar o proxy
  env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", 
      format!("--proxy-server={}", proxy));

  tauri::Builder::default()
      .setup(|app| {
          let main_window = app.get_webview_window("main").unwrap();

          // Injetar o remove_ad.js
          let js_script = include_str!("../remove_ad.js");
          let _ = main_window.eval(js_script);

          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("Deu ruim, mestre!");
}
