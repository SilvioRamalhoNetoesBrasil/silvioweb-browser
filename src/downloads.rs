// downloads.rs — Gerenciamento de Downloads do SilvioWeb

use fltk::{dialog, prelude::*};

/// Representa um item de download.
#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub url: String,
    pub filename: String,
    pub progress: f32, // 0.0 a 1.0
    pub status: DownloadStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    Pendente,
    Baixando,
    Concluido,
    Erro(String),
}

impl DownloadItem {
    pub fn new(url: &str) -> Self {
        let filename = url
            .split('/')
            .last()
            .unwrap_or("download")
            .to_string();
        let filename = if filename.is_empty() || !filename.contains('.') {
            "download.html".to_string()
        } else {
            filename
        };

        DownloadItem {
            url: url.to_string(),
            filename,
            progress: 0.0,
            status: DownloadStatus::Pendente,
        }
    }

    pub fn status_label(&self) -> &str {
        match &self.status {
            DownloadStatus::Pendente => "Pendente",
            DownloadStatus::Baixando => "Baixando...",
            DownloadStatus::Concluido => "Concluído",
            DownloadStatus::Erro(_) => "Erro",
        }
    }
}

/// Gerenciador de downloads.
pub struct DownloadManager {
    pub items: Vec<DownloadItem>,
}

impl DownloadManager {
    pub fn new() -> Self {
        DownloadManager { items: Vec::new() }
    }

    pub fn add(&mut self, url: &str) {
        let item = DownloadItem::new(url);
        println!("[Downloads] Adicionando: {} -> {}", url, item.filename);
        self.items.push(item);
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Exibe um diálogo simples de download para a URL fornecida.
pub fn show_download_dialog(url: &str) {
    let filename = url
        .split('/')
        .last()
        .unwrap_or("download")
        .to_string();
    let filename = if filename.is_empty() || !filename.contains('.') {
        "pagina.html".to_string()
    } else {
        filename
    };

    let msg = format!(
        "Download\n\nURL: {}\nArquivo: {}\n\nFuncionalidade de download\nrequere integração com Ultralight.",
        url, filename
    );
    dialog::message_default(&msg);
}
