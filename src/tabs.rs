// tabs.rs — Gerenciamento de Guias do SilvioWeb

/// Representa uma guia do navegador.
#[derive(Debug, Clone)]
pub struct Tab {
    pub id: usize,
    pub url: String,
    pub title: String,
    pub is_loading: bool,
}

impl Tab {
    /// Cria uma nova guia com a URL fornecida.
    pub fn new(id: usize, url: &str) -> Self {
        Tab {
            id,
            url: url.to_string(),
            title: format!("Guia {}", id),
            is_loading: false,
        }
    }

    /// Atualiza a URL da guia.
    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
        self.is_loading = true;
    }

    /// Define o título da guia.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    /// Marca o carregamento como concluído.
    pub fn finish_loading(&mut self) {
        self.is_loading = false;
    }

    /// Retorna um rótulo exibível para a aba (truncado).
    pub fn display_label(&self) -> String {
        let label = if self.title.len() > 20 {
            format!("{}…", &self.title[..19])
        } else {
            self.title.clone()
        };
        if self.is_loading {
            format!("⏳ {}", label)
        } else {
            label
        }
    }
}

/// Gerenciador de guias.
pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_index: usize,
}

impl TabManager {
    pub fn new() -> Self {
        TabManager {
            tabs: Vec::new(),
            active_index: 0,
        }
    }

    pub fn add_tab(&mut self, url: &str) -> usize {
        let id = self.tabs.len() + 1;
        self.tabs.push(Tab::new(id, url));
        self.active_index = self.tabs.len() - 1;
        self.active_index
    }

    pub fn close_tab(&mut self, index: usize) -> bool {
        if self.tabs.len() <= 1 {
            return false; // Pelo menos uma guia deve existir
        }
        self.tabs.remove(index);
        if self.active_index >= self.tabs.len() {
            self.active_index = self.tabs.len() - 1;
        }
        true
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_index)
    }

    pub fn count(&self) -> usize {
        self.tabs.len()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
