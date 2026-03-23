# silvioweb v0.1.1

Navegador web feito em Rust, usando **FLTK 1.5.22** como interface gráfica e o motor de renderização **Ultralight v0.1.7**.

---

## Dependências do projeto

| Crate       | Versão   | Função                              |
|-------------|----------|-------------------------------------|
| fltk        | 1.5.22   | Interface gráfica (toolbar, janela) |
| ul-next     | 0.5.4    | Bindings Rust para Ultralight       |
| winit       | 0.30.13  | Gerenciamento de janela/eventos     |
| config      | 0.15.22  | Configurações da aplicação          |
| view        | 0.4.1    | Abstração de view/viewport          |
| renderer    | 0.0.0    | Motor de renderização               |

---

## Pré-requisitos do sistema

### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    cmake \
    ninja-build \
    libx11-dev \
    libxext-dev \
    libxft-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxrender-dev \
    libxfixes-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    pkg-config \
    curl \
    git
```

### Windows
- Instale o [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) com "Desktop development with C++"
- Instale o [CMake](https://cmake.org/download/)
- Instale o [Ninja](https://ninja-build.org/)

### macOS
```bash
xcode-select --install
brew install cmake ninja
```

---

## Instalação do Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup update stable
```

---

## Instalação do Ultralight SDK (motor de renderização)

O **Ultralight v0.1.7** precisa ser instalado separadamente antes de compilar.

```bash
# 1. Baixe o SDK do Ultralight v0.1.7
#    https://github.com/ultralight-ux/Ultralight/releases/tag/v1.3.0
#    (ul-next 0.5.4 usa Ultralight SDK 1.3.x internamente)

# 2. No Linux, copie as libs para o sistema:
sudo cp libUltralight.so /usr/local/lib/
sudo cp libUltralightCore.so /usr/local/lib/
sudo cp libWebCore.so /usr/local/lib/
sudo cp libAppCore.so /usr/local/lib/
sudo ldconfig

# 3. Defina a variável de ambiente apontando para o SDK:
export ULTRALIGHT_SDK_PATH="/caminho/para/ultralight-sdk"
# Adicione ao ~/.bashrc para persistir:
echo 'export ULTRALIGHT_SDK_PATH="/caminho/para/ultralight-sdk"' >> ~/.bashrc
```

---

## Clonar e compilar o silvioweb

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/silvioweb.git
cd silvioweb

# Executar direto
cargo run

---

## Estrutura do projeto

```
silvioweb/
├── Cargo.toml          # Dependências e metadados
├── README.md           # Este arquivo
└── src/
    ├── main.rs         # Ponto de entrada, janela FLTK, toolbar, callbacks
    └── browser.rs      # Estado do navegador, histórico, normalização de URLs
```

---

## Funcionalidades implementadas

| Botão / Elemento    | Função                                              |
|---------------------|-----------------------------------------------------|
| `<-` Voltar         | Navega para a página anterior no histórico          |
| `->` Avançar        | Navega para a próxima página no histórico           |
| Recarregar          | Recarrega a URL atual                               |
| Parar               | Interrompe o carregamento                           |
| Barra de endereço   | Digite uma URL ou termo de busca e pressione Enter  |
| Botão **Ir**        | Confirma e carrega a URL digitada                   |
| URL padrão          | `https://www.google.com`                            |

---

## Correções aplicadas na v0.1.1

1. **`unused import: Duration`** — Removido `Duration` do `use std::time::{Duration, Instant}` em `browser.rs`; apenas `Instant` é utilizado.

2. **`E0277: From<&std::string::String>` não implementado** — `Window::new` do FLTK 1.5.22 exige `T: Into<Option<&str>>`. A correção foi:
   ```rust
   // ERRADO (causa E0277):
   Window::new(x, y, w, h, &format!("..."))
   // ↑ &format!() produz &String, que não implementa Into<Option<&str>>

   // CORRETO:
   let title_string = format!("{} {}", APP_NAME, VERSION);
   let title: &str = title_string.as_str();
   Window::new(x, y, w, h, title)
   // ↑ &str implementa Into<Option<&str>> ✓
   ```

---

## Licença

MIT — veja [LICENSE](LICENSE)
