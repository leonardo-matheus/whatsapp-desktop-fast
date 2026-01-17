//! # WhatsApp Lite Desktop
//!
//! Uma versão leve do WhatsApp Desktop feita em Rust usando Tauri.
//!
//! ## Motivação
//! O WhatsApp Desktop oficial baseado em Electron consome recursos excessivos,
//! chegando a usar 60% da GPU em algumas máquinas. Esta versão alternativa
//! usa apenas ~1MB de RAM e praticamente zero GPU.
//!
//! ## Arquitetura
//! - **Backend**: Rust + Tauri 2.0
//! - **Frontend**: WebView nativo (WebView2 no Windows)
//! - **Injeção de scripts**: Remove anúncios e elementos desnecessários

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use tauri::Manager;

/// Configuração padrão do proxy (pode ser sobrescrita via variável de ambiente)
const DEFAULT_PROXY: Option<&str> = None;

/// Script JavaScript para remover anúncios e otimizar a interface
const AD_REMOVAL_SCRIPT: &str = include_str!("../remove_ad.js");

/// Configura argumentos adicionais do WebView2 para otimização
fn configure_webview_args() {
    let mut args = Vec::new();

    // Configuração de proxy (opcional)
    if let Some(proxy) = env::var("WHATSAPP_PROXY")
        .ok()
        .or_else(|| DEFAULT_PROXY.map(String::from))
    {
        args.push(format!("--proxy-server={}", proxy));
    }

    // Otimizações de performance
    args.extend([
        "--disable-gpu-compositing".to_string(),      // Reduz uso de GPU
        "--disable-smooth-scrolling".to_string(),     // Menos overhead de animação
        "--disable-features=msWebOOUI".to_string(),   // Remove UI desnecessária
    ]);

    if !args.is_empty() {
        env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", args.join(" "));
    }
}

/// Inicializa a janela principal com scripts de otimização
fn setup_main_window(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app
        .get_webview_window("main")
        .ok_or("Falha ao obter janela principal")?;

    // Injeta script de remoção de anúncios
    main_window.eval(AD_REMOVAL_SCRIPT)?;

    Ok(())
}

fn main() {
    // Configura otimizações do WebView antes de iniciar
    configure_webview_args();

    tauri::Builder::default()
        .setup(|app| {
            setup_main_window(app).map_err(|e| {
                eprintln!("Erro na inicialização: {}", e);
                e
            })?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar WhatsApp Lite Desktop");
}
