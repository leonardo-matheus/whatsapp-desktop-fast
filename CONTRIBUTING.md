# Contribuindo para o WhatsApp Lite Desktop

Obrigado por considerar contribuir para o WhatsApp Lite Desktop! 🎉

## Como Contribuir

### Reportando Bugs

1. Verifique se o bug já não foi reportado nas [Issues](https://github.com/leonardo-matheus/whatsapp-desktop-fast/issues)
2. Se não encontrar, abra uma nova issue com:
   - Título claro e descritivo
   - Passos para reproduzir o bug
   - Comportamento esperado vs atual
   - Screenshots se aplicável
   - Informações do sistema (OS, versão)

### Sugerindo Features

1. Abra uma issue com a tag `enhancement`
2. Descreva a feature e por que seria útil
3. Se possível, inclua mockups ou exemplos

### Pull Requests

1. Fork o repositório
2. Clone seu fork: `git clone https://github.com/seu-usuario/whatsapp-desktop-fast.git`
3. Crie uma branch: `git checkout -b feature/minha-feature`
4. Faça suas mudanças
5. Teste localmente: `cargo tauri dev`
6. Commit: `git commit -m 'Add: minha feature'`
7. Push: `git push origin feature/minha-feature`
8. Abra um Pull Request

### Convenções de Commit

Usamos prefixos para commits:

- `Add:` Nova feature
- `Fix:` Correção de bug
- `Docs:` Apenas documentação
- `Style:` Formatação, sem mudança de código
- `Refactor:` Refatoração de código
- `Test:` Adição/modificação de testes
- `Chore:` Manutenção, builds, etc.

### Estilo de Código

- Rust: Siga as convenções do `rustfmt`
- JavaScript: Use `'use strict'` e comentários JSDoc
- Mantenha o código simples e legível

## Ambiente de Desenvolvimento

```bash
# Instale Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instale Tauri CLI
cargo install tauri-cli

# Clone e rode
git clone https://github.com/leonardo-matheus/whatsapp-desktop-fast.git
cd whatsapp-desktop-fast/src-tauri
cargo tauri dev
```

## Perguntas?

Abra uma issue com a tag `question` ou entre em contato pelo GitHub.

Obrigado! 🦀
