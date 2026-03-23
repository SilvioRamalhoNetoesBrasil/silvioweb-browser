// browser.rs - Módulo principal do navegador silvioweb
use std::time::Instant;
use std::sync::{Arc, Mutex};

pub const APP_NAME: &str = "silvioweb";
pub const VERSION: &str = "v0.1.1";
pub const DEFAULT_URL: &str = "https://www.google.com";

/// Estado compartilhado do navegador
#[derive(Debug, Clone)]
pub struct BrowserState {
    pub current_url: String,
    pub history: Vec<String>,
    pub history_index: usize,
    pub is_loading: bool,
    pub start_time: Option<Instant>,
}

impl BrowserState {
    pub fn new() -> Self {
        BrowserState {
            current_url: DEFAULT_URL.to_string(),
            history: vec![DEFAULT_URL.to_string()],
            history_index: 0,
            is_loading: false,
            start_time: None,
        }
    }

    /// Navega para uma nova URL e atualiza o histórico
    pub fn navigate(&mut self, url: &str) {
        let normalized = normalize_url(url);
        // Ao navegar para nova página, descarta o histórico à frente
        if self.history_index + 1 < self.history.len() {
            self.history.truncate(self.history_index + 1);
        }
        self.history.push(normalized.clone());
        self.history_index = self.history.len() - 1;
        self.current_url = normalized;
        self.is_loading = true;
        self.start_time = Some(Instant::now());
    }

    /// Volta para a página anterior no histórico
    pub fn go_back(&mut self) -> Option<String> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_url = self.history[self.history_index].clone();
            self.is_loading = true;
            Some(self.current_url.clone())
        } else {
            None
        }
    }

    /// Avança para a próxima página no histórico
    pub fn go_forward(&mut self) -> Option<String> {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            self.current_url = self.history[self.history_index].clone();
            self.is_loading = true;
            Some(self.current_url.clone())
        } else {
            None
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.history_index + 1 < self.history.len()
    }

    pub fn finish_loading(&mut self) {
        self.is_loading = false;
        self.start_time = None;
    }
}

/// Normaliza a URL adicionando https:// se necessário
pub fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return DEFAULT_URL.to_string();
    }
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else if trimmed.contains('.') {
        format!("https://{}", trimmed)
    } else {
        // Trata como busca no Google
        format!("https://www.google.com/search?q={}", trimmed.replace(' ', "+"))
    }
}

pub type SharedState = Arc<Mutex<BrowserState>>;

pub fn new_shared_state() -> SharedState {
    Arc::new(Mutex::new(BrowserState::new()))
}
