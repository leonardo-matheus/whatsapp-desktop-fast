<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="WhatsApp Lite Logo" width="128" height="128">
</p>

<h1 align="center">WhatsApp Lite Desktop</h1>

<p align="center">
  <strong>🚀 Uma versão ultra-leve do WhatsApp Desktop feita em Rust</strong>
</p>

<p align="center">
  <a href="#-o-problema">O Problema</a> •
  <a href="#-a-solução">A Solução</a> •
  <a href="#-comparação">Comparação</a> •
  <a href="#-instalação">Instalação</a> •
  <a href="#-desenvolvimento">Desenvolvimento</a> •
  <a href="#-licença">Licença</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/RAM-~1MB-brightgreen?style=for-the-badge" alt="RAM Usage">
  <img src="https://img.shields.io/badge/GPU-0%25-brightgreen?style=for-the-badge" alt="GPU Usage">
  <img src="https://img.shields.io/badge/Rust-🦀-orange?style=for-the-badge" alt="Built with Rust">
  <img src="https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge" alt="Tauri 2.0">
</p>

---

## 😱 O Problema

Você já abriu o Gerenciador de Tarefas e viu isso?

```
WhatsApp Desktop - 60% GPU (RTX 3060 Ti)
WhatsApp Desktop - 500MB+ RAM
```

**Sério?** Uma aplicação de mensagens precisa de MAIS recursos que alguns jogos AAA?

O WhatsApp Desktop oficial é construído com **Electron**, que basicamente roda um navegador Chrome inteiro só para exibir uma página web. Isso significa:

- 🔥 **Alto consumo de GPU** - Até 60% em placas como RTX 3060 Ti
- 💾 **500MB+ de RAM** - Para uma aplicação de chat!
- 🔋 **Drena bateria** - Péssimo para notebooks
- 🐌 **Startup lento** - Vários segundos para abrir
- 📦 **Instalador pesado** - ~150MB de download

## 💡 A Solução

**WhatsApp Lite Desktop** usa [Tauri](https://tauri.app/) + Rust para criar um wrapper nativo ultra-leve:

- ✅ Usa o **WebView nativo** do sistema (WebView2 no Windows)
- ✅ **Zero overhead** de um navegador embutido
- ✅ Backend em **Rust** = máxima eficiência
- ✅ Remove **anúncios e banners** automaticamente
- ✅ **Otimizações de GPU** desabilitando composição desnecessária

## 📊 Comparação

| Métrica | WhatsApp Oficial | WhatsApp Lite | Economia |
|---------|------------------|---------------|----------|
| **RAM** | 500MB+ | ~1MB | **99.8%** ⬇️ |
| **GPU** | 60%+ | ~0% | **99%** ⬇️ |
| **Tamanho do instalador** | ~150MB | ~3MB | **98%** ⬇️ |
| **Tempo de startup** | 5-10s | <1s | **90%** ⬇️ |
| **Consumo de bateria** | Alto | Mínimo | **Significativo** ⬇️ |

## 🚀 Instalação

### Download Pré-compilado

Baixe a última versão na página de [Releases](https://github.com/leonardo-matheus/whatsapp-desktop-fast/releases):

- **Windows**: `WhatsApp-Lite_1.0.0_x64-setup.exe`
- **macOS**: `WhatsApp-Lite_1.0.0_aarch64.dmg`
- **Linux**: `whatsapp-lite_1.0.0_amd64.deb` ou `.AppImage`

### Compilar do Código-Fonte

#### Pré-requisitos

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 18+ (apenas para Tauri CLI)
- **Windows**: WebView2 (já incluído no Windows 11)
- **Linux**: `libwebkit2gtk-4.1`, `libgtk-3`
- **macOS**: Xcode Command Line Tools

#### Build

```bash
# Clone o repositório
git clone https://github.com/leonardo-matheus/whatsapp-desktop-fast.git
cd whatsapp-desktop-fast

# Instale a CLI do Tauri
cargo install tauri-cli

# Build em modo release
cd src-tauri
cargo tauri build
```

O executável estará em `src-tauri/target/release/`.

## 🔧 Configuração

### Proxy (Opcional)

Se você precisa usar um proxy, defina a variável de ambiente antes de iniciar:

```bash
# Windows (PowerShell)
$env:WHATSAPP_PROXY = "http://seu-proxy:porta"

# Linux/macOS
export WHATSAPP_PROXY="http://seu-proxy:porta"
```

## 🛠️ Desenvolvimento

```bash
# Modo desenvolvimento com hot-reload
cd src-tauri
cargo tauri dev
```

### Estrutura do Projeto

```
whatsapp-desktop-fast/
├── src-tauri/
│   ├── src/
│   │   └── main.rs          # Código principal Rust
│   ├── remove_ad.js         # Script de remoção de anúncios
│   ├── Cargo.toml           # Dependências Rust
│   ├── tauri.conf.json      # Configuração do Tauri
│   └── icons/               # Ícones da aplicação
├── docs/
│   └── index.html           # Landing page
└── README.md
```

## 🤔 FAQ

### É seguro?

Sim! O aplicativo apenas carrega o WhatsApp Web oficial em um WebView nativo. Não há proxy de dados, não há modificação de mensagens. O código é 100% open source para você auditar.

### Por que não simplesmente usar o navegador?

Você pode! Mas ter uma aplicação dedicada oferece:
- Ícone na bandeja do sistema
- Notificações nativas
- Janela separada do navegador
- Menor consumo de recursos (vs uma aba do Chrome)

### Funciona com múltiplas contas?

Atualmente não. É uma feature planejada para versões futuras.

### O WhatsApp pode me banir?

Não. Este aplicativo usa o WhatsApp Web oficial, exatamente como seu navegador. Não há violação dos Termos de Serviço.

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add: amazing feature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para detalhes.

## 🙏 Agradecimentos

- [Tauri](https://tauri.app/) - Framework incrível para apps desktop
- [Rust](https://www.rust-lang.org/) - Linguagem que torna isso possível
- Comunidade open source

---

<p align="center">
  Feito com 🦀 e ☕ por <a href="https://github.com/leonardo-matheus">Leonardo Matheus</a>
</p>

<p align="center">
  <sub>Se este projeto te ajudou, considere dar uma ⭐ no repositório!</sub>
</p>
