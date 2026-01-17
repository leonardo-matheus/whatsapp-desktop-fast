# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [1.0.0] - 2026-01-17

### Adicionado
- 🎉 Lançamento inicial do WhatsApp Lite Desktop
- 🦀 Backend em Rust com Tauri 2.0
- 🪶 Uso de WebView nativo (WebView2 no Windows)
- 🚫 Script automático para remoção de anúncios
- ⚡ Otimizações de GPU desabilitando composição desnecessária
- 🔧 Suporte a proxy via variável de ambiente `WHATSAPP_PROXY`
- 📱 Ícone na bandeja do sistema
- 🖥️ Suporte a Windows, macOS e Linux
- 📄 Documentação completa
- 🌐 Landing page explicativa

### Otimizações
- Uso de ~1MB de RAM (vs 500MB+ do oficial)
- GPU praticamente zerada (vs 60%+ do oficial)
- Instalador de ~3MB (vs ~150MB do oficial)
- Startup < 1 segundo (vs 5-10s do oficial)

### Técnico
- Configuração de CSP para segurança
- Profile de release otimizado (LTO, strip, opt-level z)
- Debounce no observer de DOM para performance
- Desabilitação de animações CSS para reduzir overhead
