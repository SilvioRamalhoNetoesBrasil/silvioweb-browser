// browser.rs — Encapsula o motor de renderização Ultralight v0.1.7
// CORREÇÃO: removida importação não utilizada de `Duration` (warning unused import)

use std::time::Instant; // apenas Instant é usado

/// Representa o motor de navegação baseado em Ultralight.
pub struct BrowserEngine {
    width: u32,
    height: u32,
    current_url: String,
    history: Vec<String>,
    history_index: usize,
    pub loading: bool,
    start_time: Instant,
}

impl BrowserEngine {
    /// Cria uma nova instância do motor de renderização.
    pub fn new(width: u32, height: u32) -> Self {
        BrowserEngine {
            width,
            height,
            current_url: String::new(),
            history: Vec::new(),
            history_index: 0,
            loading: false,
            start_time: Instant::now(),
        }
    }

    /// Navega para a URL especificada.
    pub fn navigate(&mut self, url: &str) {
        self.loading = true;
        self.start_time = Instant::now();

        // Se já existe histórico e não estamos no fim, truncar
        if self.history_index + 1 < self.history.len() {
            self.history.truncate(self.history_index + 1);
        }

        self.current_url = url.to_string();
        self.history.push(url.to_string());
        self.history_index = self.history.len().saturating_sub(1);

        // TODO: integração real com ul-next / ultralight
        // let view = ul_next::View::new(self.width, self.height);
        // view.load_url(url);
        println!("[BrowserEngine] Navegando para: {} ({}x{})", url, self.width, self.height);

        self.loading = false;
    }

    /// Para o carregamento atual.
    pub fn stop(&mut self) {
        self.loading = false;
        println!("[BrowserEngine] Carregamento interrompido.");
    }

    /// Retorna a URL atual.
    pub fn current_url(&self) -> String {
        self.current_url.clone()
    }

    /// Verifica se é possível voltar no histórico.
    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    /// Verifica se é possível avançar no histórico.
    pub fn can_go_forward(&self) -> bool {
        self.history_index + 1 < self.history.len()
    }

    /// Vai para a página anterior no histórico.
    pub fn go_back(&mut self) {
        if self.can_go_back() {
            self.history_index -= 1;
            self.current_url = self.history[self.history_index].clone();
            println!("[BrowserEngine] Voltando para: {}", self.current_url);
        }
    }

    /// Vai para a próxima página no histórico.
    pub fn go_forward(&mut self) {
        if self.can_go_forward() {
            self.history_index += 1;
            self.current_url = self.history[self.history_index].clone();
            println!("[BrowserEngine] Avançando para: {}", self.current_url);
        }
    }

    /// Retorna o tempo decorrido desde o início do carregamento.
    pub fn elapsed_ms(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }

    /// Retorna as dimensões do viewport.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
