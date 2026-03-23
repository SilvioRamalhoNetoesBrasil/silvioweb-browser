# SilvioWeb v0.1.1

Navegador MultiFuncional baseado em **FLTK 1.5.22** e motor de renderização **Ultralight v0.1.7**.

---

## Dependências utilizadas

| Crate         | Versão   | Finalidade                        |
|---------------|----------|-----------------------------------|
| `fltk`        | 1.5.22   | Interface gráfica                 |
| `ultralight`  | 0.1.7    | Motor de renderização web         |
| `ul-next`     | 0.5.4    | Bindings Rust para Ultralight     |
| `winit`       | 0.30.13  | Janela nativa multiplataforma     |
| `config`      | 0.15.22  | Gerenciamento de configurações    |
| `view`        | 0.4.1    | Componente de visualização        |
| `renderer`    | 0.0.0    | Renderer base                     |
| `serde`       | 1.0      | Serialização                      |
| `serde_json`  | 1.0      | JSON                              |
| `open`        | 5.0      | Abrir URLs externas               |

---

## Pré-requisitos

### Linux (Ubuntu/Debian)

```bash
# Atualizar repositórios
sudo apt update -y
sudo apt upgrade -y

# Compilador Rust e Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Dependências de sistema para FLTK
sudo apt install -y \
    build-essential \
    cmake \
    libx11-dev \
    libxext-dev \
    libxft-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxrender-dev \
    libxfixes-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libasound2-dev \
    pkg-config \
    git \
    curl \
    wget

# Dependências para Ultralight (motor de renderização)
sudo apt install -y \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev
```

### Linux (Fedora/RHEL)

```bash
sudo dnf update -y
sudo dnf install -y \
    gcc gcc-c++ cmake \
    libX11-devel libXext-devel libXft-devel \
    libXinerama-devel libXcursor-devel \
    libXrender-devel mesa-libGL-devel \
    alsa-lib-devel openssl-devel \
    gtk3-devel webkit2gtk3-devel
```

### Windows

```powershell
# Instalar Rust via rustup
# https://rustup.rs

# Instalar Visual Studio Build Tools 2022
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
# Selecionar: "Desktop development with C++"

# CMake (necessário para FLTK)
winget install Kitware.CMake
```

### macOS

```bash
# Instalar Homebrew se necessário
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Dependências
brew install cmake pkg-config

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Instalação e Execução

### 1. Baixar o projeto

```bash
# O Projeto Simplificado
https://github.com/SilvioRamalhoNetoesBrasil/silvioweb-browser/releases/download/v0.1.1/silvioweb-browser-v0.1.1-sourcecode.tar.gz

# O Projeto Com MúltiFuncional
wget https://github.com/SilvioRamalhoNetoesBrasil/silvioweb-browser/releases/download/v0.1.1/SilvioWeb-Browser-v0.1.1-MultiTab.tar.xz
```

```bash
# Extrair o Pacote Simplificado
tar xpvf silvioweb-browser-v0.1.1-sourcecode.tar.gz
cd SilvioWeb-Browser-v0.1.1-sourcecode

# Extrair o Pacote MúltiFuncional
tar xpvf SilvioWeb-Browser-v0.1.1-MultiTab.tar.xz
cd SilvioWeb-Browser-v0.1.1-MultiTab
```

### 3. Executar diretamente

```bash
./silvioweb
```

---

## Solução de erros comuns

### `error[E0277]` — trait `From<&String>` não implementado para `Option<&str>`

**Causa:** O FLTK espera `&str` no título da janela, não `&String`.

**Correção aplicada no código:**
```rust
// ERRADO:
let mut win = Window::new(100, 100, w, h, &format!("{} v{}", APP_NAME, VERSION));

// CORRETO:
let title = format!("{} v{}", APP_NAME, VERSION);
let mut win = Window::new(100, 100, w, h, title.as_str());
```

### `warning: unused import: Duration`

**Causa:** `Duration` foi importado mas não usado no `browser.rs`.

**Correção aplicada:**
```rust
// ERRADO:
use std::time::{Duration, Instant};

// CORRETO:
use std::time::Instant;
```

### FLTK não encontrado / erro de compilação

```bash
# Verificar se as dependências de sistema estão instaladas
sudo apt install -y libx11-dev libxext-dev libxft-dev libxinerama-dev \
    libxcursor-dev libxrender-dev libgl1-mesa-dev

# Limpar cache e recompilar
cargo clean
cargo build
```

### Ultralight não compila

O Ultralight requer os headers de sistema instalados.
Consulte: https://github.com/ultralight-ux/Ultralight

---

## Funcionalidades

- ✅ Botão Voltar (`<-`)
- ✅ Botão Avançar (`->`)
- ✅ Botão Recarregar (F5)
- ✅ Botão Parar
- ✅ Barra de Endereço com botão Ir
- ✅ URL padrão: `https://www.google.com`
- ✅ Múltiplas Guias (abertura e fechamento)
- ✅ Botão de Download
- ✅ Barra de Status
- ✅ Normalização de URL (adiciona `https://` automaticamente)
- ✅ Pesquisa Google ao digitar texto sem ponto

---

## Licença

Projeto de uso pessoal — SilvioWeb v0.1.1
