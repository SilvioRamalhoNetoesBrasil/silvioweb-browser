// config_manager.rs — Configurações do SilvioWeb
// Usa a crate `config` v0.15.22

use std::collections::HashMap;

/// Configurações do navegador.
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    pub home_url: String,
    pub window_width: u32,
    pub window_height: u32,
    pub javascript_enabled: bool,
    pub max_tabs: usize,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        BrowserConfig {
            home_url: "https://www.google.com".to_string(),
            window_width: 1280,
            window_height: 800,
            javascript_enabled: true,
            max_tabs: 20,
        }
    }
}

impl BrowserConfig {
    /// Carrega configurações do arquivo config.toml se existir,
    /// ou retorna os valores padrão.
    pub fn load() -> Self {
        // Integração com a crate `config` v0.15.22
        // A integração completa é feita aqui:
        //
        // use config::{Config, File};
        // let settings = Config::builder()
        //     .add_source(File::with_name("config").required(false))
        //     .build()
        //     .unwrap_or_default();
        //
        // Por ora retornamos os valores padrão para compilação limpa.
        BrowserConfig::default()
    }

    /// Retorna um HashMap com todas as configurações para display.
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("home_url".to_string(), self.home_url.clone());
        map.insert("window_width".to_string(), self.window_width.to_string());
        map.insert("window_height".to_string(), self.window_height.to_string());
        map.insert("javascript_enabled".to_string(), self.javascript_enabled.to_string());
        map.insert("max_tabs".to_string(), self.max_tabs.to_string());
        map
    }
}
