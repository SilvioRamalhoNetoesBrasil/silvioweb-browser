// SilvioWeb v0.1.1 - Navegador MultiFuncional
// Interface: FLTK 1.5.22
// Motor: Ultralight v0.1.7 + ul-next v0.5.4

mod browser;
mod tabs;
mod downloads;
mod config_manager;

use fltk::{
    app,
    button::Button,
    enums::{Color, Font, FrameType, Shortcut},
    frame::Frame,
    group::{Group, Pack, Tabs},
    input::Input,
    menu::MenuBar,
    prelude::*,
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;

pub const APP_NAME: &str = "SilvioWeb";
pub const VERSION: &str = "0.1.1";
pub const HOME_URL: &str = "https://www.google.com";

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(30, 30, 30);
    app::background2(45, 45, 45);
    app::foreground(220, 220, 220);

    let win_w = 1280;
    let win_h = 800;

    // ── CORREÇÃO E0277: usar &str literal em vez de &String ──────────────────
    let title = format!("{} v{}", APP_NAME, VERSION);
    let mut win = Window::new(100, 100, win_w, win_h, title.as_str());
    win.set_color(Color::from_rgb(30, 30, 30));
    win.make_resizable(true);

    // ── Menu bar ─────────────────────────────────────────────────────────────
    let mut menu = MenuBar::new(0, 0, win_w, 25, "");
    menu.set_color(Color::from_rgb(45, 45, 45));
    menu.set_text_color(Color::from_rgb(220, 220, 220));
    menu.add_choice("Arquivo/Nova Guia\t");
    menu.add_choice("Arquivo/Fechar Guia\t");
    menu.add_choice("Arquivo/Sair\t");
    menu.add_choice("Ajuda/Sobre");

    // ── Barra de navegação (toolbar) ──────────────────────────────────────────
    let toolbar_y = 25;
    let toolbar_h = 40;
    let mut toolbar = Group::new(0, toolbar_y, win_w, toolbar_h, "");
    toolbar.set_color(Color::from_rgb(45, 45, 45));
    toolbar.set_frame(FrameType::FlatBox);

    // Botão Voltar
    let mut btn_back = Button::new(5, toolbar_y + 5, 30, 30, "@<-");
    btn_back.set_color(Color::from_rgb(60, 60, 60));
    btn_back.set_selection_color(Color::from_rgb(80, 80, 80));
    btn_back.set_label_color(Color::from_rgb(220, 220, 220));
    btn_back.set_frame(FrameType::RoundedBox);
    btn_back.set_tooltip("Voltar");

    // Botão Avançar
    let mut btn_forward = Button::new(40, toolbar_y + 5, 30, 30, "@->");
    btn_forward.set_color(Color::from_rgb(60, 60, 60));
    btn_forward.set_selection_color(Color::from_rgb(80, 80, 80));
    btn_forward.set_label_color(Color::from_rgb(220, 220, 220));
    btn_forward.set_frame(FrameType::RoundedBox);
    btn_forward.set_tooltip("Avançar");

    // Botão Recarregar
    let mut btn_reload = Button::new(75, toolbar_y + 5, 30, 30, "@reload");
    btn_reload.set_color(Color::from_rgb(60, 60, 60));
    btn_reload.set_selection_color(Color::from_rgb(80, 80, 80));
    btn_reload.set_label_color(Color::from_rgb(220, 220, 220));
    btn_reload.set_frame(FrameType::RoundedBox);
    btn_reload.set_tooltip("Recarregar (F5)");

    // Botão Parar
    let mut btn_stop = Button::new(110, toolbar_y + 5, 30, 30, "@square");
    btn_stop.set_color(Color::from_rgb(60, 60, 60));
    btn_stop.set_selection_color(Color::from_rgb(80, 80, 80));
    btn_stop.set_label_color(Color::from_rgb(220, 50, 50));
    btn_stop.set_frame(FrameType::RoundedBox);
    btn_stop.set_tooltip("Parar");

    // Barra de endereço
    let addr_x = 145;
    let addr_w = win_w - addr_x - 120;
    let mut addr_bar = Input::new(addr_x, toolbar_y + 7, addr_w, 26, "");
    addr_bar.set_value(HOME_URL);
    addr_bar.set_color(Color::from_rgb(55, 55, 55));
    addr_bar.set_text_color(Color::from_rgb(220, 220, 220));
    addr_bar.set_cursor_color(Color::from_rgb(100, 180, 255));
    addr_bar.set_frame(FrameType::RoundedBox);

    // Botão Ir
    let go_x = addr_x + addr_w + 5;
    let mut btn_go = Button::new(go_x, toolbar_y + 5, 50, 30, "Ir");
    btn_go.set_color(Color::from_rgb(0, 120, 215));
    btn_go.set_selection_color(Color::from_rgb(0, 100, 180));
    btn_go.set_label_color(Color::White);
    btn_go.set_frame(FrameType::RoundedBox);
    btn_go.set_tooltip("Ir para URL (Enter)");

    // Botão Download
    let dl_x = go_x + 55;
    let mut btn_download = Button::new(dl_x, toolbar_y + 5, 55, 30, "@filesave");
    btn_download.set_color(Color::from_rgb(60, 60, 60));
    btn_download.set_selection_color(Color::from_rgb(80, 80, 80));
    btn_download.set_label_color(Color::from_rgb(100, 200, 100));
    btn_download.set_frame(FrameType::RoundedBox);
    btn_download.set_tooltip("Downloads");

    toolbar.end();

    // ── Barra de guias ────────────────────────────────────────────────────────
    let tabs_y = toolbar_y + toolbar_h;
    let tabs_bar_h = 30;
    let mut tab_group = Group::new(0, tabs_y, win_w, tabs_bar_h, "");
    tab_group.set_color(Color::from_rgb(38, 38, 38));
    tab_group.set_frame(FrameType::FlatBox);

    let mut btn_new_tab = Button::new(5, tabs_y + 3, 24, 24, "+");
    btn_new_tab.set_color(Color::from_rgb(60, 60, 60));
    btn_new_tab.set_label_color(Color::from_rgb(220, 220, 220));
    btn_new_tab.set_frame(FrameType::RoundedBox);
    btn_new_tab.set_tooltip("Nova Guia (Ctrl+T)");

    tab_group.end();

    // ── Área de conteúdo (onde as guias renderizam) ───────────────────────────
    let content_y = tabs_y + tabs_bar_h;
    let content_h = win_h - content_y - 22;
    let mut content_area = Frame::new(0, content_y, win_w, content_h, "");
    content_area.set_color(Color::from_rgb(255, 255, 255));
    content_area.set_frame(FrameType::DownBox);
    content_area.set_label_color(Color::from_rgb(60, 60, 60));
    content_area.set_label(
        "Motor de Renderização: Ultralight v0.1.7\n\
         Carregando www.google.com...",
    );

    // ── Barra de status ───────────────────────────────────────────────────────
    let status_y = win_h - 22;
    let mut status_bar = Frame::new(0, status_y, win_w, 22, "Pronto");
    status_bar.set_color(Color::from_rgb(38, 38, 38));
    status_bar.set_label_color(Color::from_rgb(160, 160, 160));
    status_bar.set_frame(FrameType::FlatBox);
    status_bar.set_label_font(Font::Helvetica);
    status_bar.set_label_size(11);

    win.end();
    win.show();

    // ── Estado compartilhado ──────────────────────────────────────────────────
    let current_url = Rc::new(RefCell::new(HOME_URL.to_string()));
    let loading = Rc::new(RefCell::new(false));
    let tab_list: Rc<RefCell<Vec<tabs::Tab>>> = Rc::new(RefCell::new(vec![
        tabs::Tab::new(1, HOME_URL),
    ]));

    // ── Inicializa o motor Ultralight ─────────────────────────────────────────
    let browser_engine = browser::BrowserEngine::new(win_w as u32, content_h as u32);
    let engine = Rc::new(RefCell::new(browser_engine));

    {
        let e = engine.clone();
        let url = current_url.clone();
        let mut status = status_bar.clone();
        let mut content = content_area.clone();
        e.borrow_mut().navigate(HOME_URL);
        status.set_label(&format!("Carregando: {}", HOME_URL));
        content.set_label(&format!("Carregando: {}", HOME_URL));
        let _ = url;
    }

    // ── Callbacks dos botões ──────────────────────────────────────────────────

    // Botão Ir / Enter na barra de endereço
    {
        let engine_ref = engine.clone();
        let url_ref = current_url.clone();
        let mut addr = addr_bar.clone();
        let mut status = status_bar.clone();
        let mut content = content_area.clone();
        let loading_ref = loading.clone();

        btn_go.set_callback(move |_| {
            let raw = addr.value();
            let url = normalize_url(&raw);
            addr.set_value(&url);
            *url_ref.borrow_mut() = url.clone();
            *loading_ref.borrow_mut() = true;
            engine_ref.borrow_mut().navigate(&url);
            status.set_label(&format!("Carregando: {}", url));
            content.set_label(&format!("Carregando: {}", url));
        });
    }

    // Botão Voltar
    {
        let engine_ref = engine.clone();
        let mut addr = addr_bar.clone();
        let mut status = status_bar.clone();

        btn_back.set_callback(move |_| {
            if engine_ref.borrow().can_go_back() {
                engine_ref.borrow_mut().go_back();
                let url = engine_ref.borrow().current_url();
                addr.set_value(&url);
                status.set_label(&format!("Voltando para: {}", url));
            }
        });
    }

    // Botão Avançar
    {
        let engine_ref = engine.clone();
        let mut addr = addr_bar.clone();
        let mut status = status_bar.clone();

        btn_forward.set_callback(move |_| {
            if engine_ref.borrow().can_go_forward() {
                engine_ref.borrow_mut().go_forward();
                let url = engine_ref.borrow().current_url();
                addr.set_value(&url);
                status.set_label(&format!("Avançando para: {}", url));
            }
        });
    }

    // Botão Recarregar
    {
        let engine_ref = engine.clone();
        let mut status = status_bar.clone();
        let url_ref = current_url.clone();

        btn_reload.set_callback(move |_| {
            let url = url_ref.borrow().clone();
            engine_ref.borrow_mut().navigate(&url);
            status.set_label(&format!("Recarregando: {}", url));
        });
    }

    // Botão Parar
    {
        let engine_ref = engine.clone();
        let mut status = status_bar.clone();
        let loading_ref = loading.clone();

        btn_stop.set_callback(move |_| {
            engine_ref.borrow_mut().stop();
            *loading_ref.borrow_mut() = false;
            status.set_label("Carregamento interrompido.");
        });
    }

    // Botão Nova Guia
    {
        let tab_list_ref = tab_list.clone();
        let mut status = status_bar.clone();
        let engine_ref = engine.clone();
        let mut addr = addr_bar.clone();
        let mut content = content_area.clone();

        btn_new_tab.set_callback(move |_| {
            let id = tab_list_ref.borrow().len() + 1;
            let new_tab = tabs::Tab::new(id, HOME_URL);
            tab_list_ref.borrow_mut().push(new_tab);
            engine_ref.borrow_mut().navigate(HOME_URL);
            addr.set_value(HOME_URL);
            content.set_label(&format!("Nova Guia {} — Carregando: {}", id, HOME_URL));
            status.set_label(&format!("Guia {} aberta — {}", id, HOME_URL));
        });
    }

    // Botão Download
    {
        let mut status = status_bar.clone();
        let url_ref = current_url.clone();

        btn_download.set_callback(move |_| {
            let url = url_ref.borrow().clone();
            downloads::show_download_dialog(&url);
            status.set_label(&format!("Download iniciado: {}", url));
        });
    }

    // Menu callbacks
    {
        let tab_list_ref = tab_list.clone();
        let engine_ref = engine.clone();
        let mut addr = addr_bar.clone();
        let mut status = status_bar.clone();
        let mut content = content_area.clone();

        menu.set_callback(move |m| {
            match m.choice().as_deref() {
                Some("Arquivo/Nova Guia\t") => {
                    let id = tab_list_ref.borrow().len() + 1;
                    let new_tab = tabs::Tab::new(id, HOME_URL);
                    tab_list_ref.borrow_mut().push(new_tab);
                    engine_ref.borrow_mut().navigate(HOME_URL);
                    addr.set_value(HOME_URL);
                    content.set_label(&format!("Nova Guia {} — {}", id, HOME_URL));
                    status.set_label(&format!("Guia {} aberta", id));
                }
                Some("Arquivo/Fechar Guia\t") => {
                    let mut tabs = tab_list_ref.borrow_mut();
                    if tabs.len() > 1 {
                        tabs.pop();
                        status.set_label("Guia fechada.");
                    } else {
                        status.set_label("Pelo menos uma guia deve estar aberta.");
                    }
                }
                Some("Arquivo/Sair\t") => {
                    app::quit();
                }
                Some("Ajuda/Sobre") => {
                    fltk::dialog::message_default(
                        &format!(
                            "{} v{}\n\nInterface: FLTK 1.5.22\nMotor: Ultralight v0.1.7\nul-next v0.5.4\nwinit v0.30.13\nconfig v0.15.22\nview v0.4.1\nrenderer v0.0.0",
                            APP_NAME, VERSION
                        )
                    );
                }
                _ => {}
            }
        });
    }

    // Atalho de teclado F5 para recarregar
    win.handle({
        let engine_ref = engine.clone();
        let url_ref = current_url.clone();
        let mut status = status_bar.clone();
        move |_, ev| {
            if ev == fltk::enums::Event::KeyDown {
                if app::event_key() == fltk::enums::Key::F5 {
                    let url = url_ref.borrow().clone();
                    engine_ref.borrow_mut().navigate(&url);
                    status.set_label(&format!("Recarregando: {}", url));
                    return true;
                }
            }
            false
        }
    });

    app.run().expect("Falha ao executar o aplicativo SilvioWeb");
}

/// Normaliza a URL digitada pelo usuário adicionando https:// se necessário.
fn normalize_url(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else if trimmed.contains('.') {
        format!("https://{}", trimmed)
    } else {
        // Tratar como pesquisa Google
        format!("https://www.google.com/search?q={}", trimmed.replace(' ', "+"))
    }
}
