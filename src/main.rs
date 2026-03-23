// main.rs - silvioweb v0.1.1
// Navegador baseado em FLTK 1.5.22 + motor Ultralight v0.1.7
// Corrigido: removed unused `Duration`, fixed Window::new title type

mod browser;

use browser::{APP_NAME, VERSION, DEFAULT_URL, new_shared_state, normalize_url};

use fltk::{
    app,
    button::Button,
    enums::{Color, Font, FrameType},
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
};
use std::sync::{Arc, Mutex};

// Largura e altura padrão da janela
const WIN_W: i32 = 1280;
const WIN_H: i32 = 800;

// Alturas dos elementos da toolbar
const TOOLBAR_H: i32 = 40;
const STATUSBAR_H: i32 = 24;
const BTN_W: i32 = 36;
const BTN_H: i32 = 30;
const PADDING: i32 = 5;

fn main() {
    let state = new_shared_state();

    // ── Aplicação FLTK ──────────────────────────────────────────────────────
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    // CORREÇÃO: Window::new espera &str (não &String).
    // Usamos uma variável &'static str montada em tempo de compilação via concat!
    // ou convertemos com .as_str() logo após o format!.
    let title_string = format!("{} {}", APP_NAME, VERSION);
    let title: &str = title_string.as_str();

    let mut win = Window::new(100, 100, WIN_W, WIN_H, title);
    win.set_color(Color::from_rgb(30, 30, 30));

    // ── Toolbar ─────────────────────────────────────────────────────────────
    let toolbar_y = PADDING;

    // Botão Voltar (<-)
    let mut btn_back = Button::new(PADDING, toolbar_y + PADDING, BTN_W, BTN_H, "@<-");
    btn_back.set_tooltip("Voltar");
    btn_back.set_color(Color::from_rgb(50, 50, 50));
    btn_back.set_label_color(Color::White);
    btn_back.set_frame(FrameType::RoundedBox);

    // Botão Avançar (->)
    let mut btn_forward = Button::new(
        PADDING + BTN_W + PADDING,
        toolbar_y + PADDING,
        BTN_W,
        BTN_H,
        "@->",
    );
    btn_forward.set_tooltip("Avançar");
    btn_forward.set_color(Color::from_rgb(50, 50, 50));
    btn_forward.set_label_color(Color::White);
    btn_forward.set_frame(FrameType::RoundedBox);

    // Botão Recarregar
    let reload_x = PADDING + (BTN_W + PADDING) * 2;
    let mut btn_reload = Button::new(reload_x, toolbar_y + PADDING, BTN_W, BTN_H, "@reload");
    btn_reload.set_tooltip("Recarregar");
    btn_reload.set_color(Color::from_rgb(50, 50, 50));
    btn_reload.set_label_color(Color::White);
    btn_reload.set_frame(FrameType::RoundedBox);

    // Botão Parar
    let stop_x = reload_x + BTN_W + PADDING;
    let mut btn_stop = Button::new(stop_x, toolbar_y + PADDING, BTN_W, BTN_H, "@square");
    btn_stop.set_tooltip("Parar");
    btn_stop.set_color(Color::from_rgb(50, 50, 50));
    btn_stop.set_label_color(Color::White);
    btn_stop.set_frame(FrameType::RoundedBox);

    // Barra de endereço (Input)
    let url_x = stop_x + BTN_W + PADDING;
    let go_w = 50_i32;
    let url_w = WIN_W - url_x - go_w - PADDING * 3;
    let mut url_input = Input::new(url_x, toolbar_y + PADDING, url_w, BTN_H, "");
    url_input.set_value(DEFAULT_URL);
    url_input.set_color(Color::from_rgb(45, 45, 45));
    url_input.set_text_color(Color::White);
    url_input.set_cursor_color(Color::White);
    url_input.set_frame(FrameType::RoundedBox);
    url_input.set_text_font(Font::Helvetica);
    url_input.set_text_size(13);

    // Botão Ir
    let go_x = url_x + url_w + PADDING;
    let mut btn_go = Button::new(go_x, toolbar_y + PADDING, go_w, BTN_H, "Ir");
    btn_go.set_tooltip("Ir para a URL");
    btn_go.set_color(Color::from_rgb(0, 122, 204));
    btn_go.set_label_color(Color::White);
    btn_go.set_frame(FrameType::RoundedBox);

    // ── Área de renderização (Frame placeholder) ─────────────────────────────
    let viewport_y = TOOLBAR_H + PADDING * 2;
    let viewport_h = WIN_H - viewport_y - STATUSBAR_H - PADDING;
    let mut viewport = Frame::new(0, viewport_y, WIN_W, viewport_h, "");
    viewport.set_color(Color::White);
    viewport.set_frame(FrameType::DownBox);
    viewport.set_label_color(Color::from_rgb(80, 80, 80));
    viewport.set_label_size(14);
    viewport.set_label(
        "Motor Ultralight v0.1.7  |  ul-next v0.5.4\nCarregando www.google.com ...",
    );

    // ── Barra de status ──────────────────────────────────────────────────────
    let status_y = WIN_H - STATUSBAR_H;
    let mut status_bar = Frame::new(0, status_y, WIN_W, STATUSBAR_H, "Pronto");
    status_bar.set_color(Color::from_rgb(20, 20, 20));
    status_bar.set_label_color(Color::from_rgb(180, 180, 180));
    status_bar.set_label_size(11);
    status_bar.set_frame(FrameType::FlatBox);
    status_bar.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    win.end();
    win.show();
    win.make_resizable(true);

    // ── Clones para closures ─────────────────────────────────────────────────
    let state_back    = Arc::clone(&state);
    let state_fwd     = Arc::clone(&state);
    let state_reload  = Arc::clone(&state);
    let state_stop    = Arc::clone(&state);
    let state_go      = Arc::clone(&state);
    let state_enter   = Arc::clone(&state);

    let mut url_input_back   = url_input.clone();
    let mut url_input_fwd    = url_input.clone();
    let mut url_input_reload = url_input.clone();
    let mut url_input_go     = url_input.clone();
    let mut url_input_enter  = url_input.clone();

    let mut viewport_back    = viewport.clone();
    let mut viewport_fwd     = viewport.clone();
    let mut viewport_reload  = viewport.clone();
    let mut viewport_stop    = viewport.clone();
    let mut viewport_go      = viewport.clone();
    let mut viewport_enter   = viewport.clone();

    let mut status_back   = status_bar.clone();
    let mut status_fwd    = status_bar.clone();
    let mut status_reload = status_bar.clone();
    let mut status_stop   = status_bar.clone();
    let mut status_go     = status_bar.clone();
    let mut status_enter  = status_bar.clone();

    // helper: atualiza viewport com a URL atual
    fn render_url(viewport: &mut Frame, status: &mut Frame, url: &str) {
        let msg = format!("Carregando: {}", url);
        viewport.set_label(&msg);
        status.set_label(&format!(" Navegando para {}", url));
        viewport.redraw();
        status.redraw();
        // TODO: integrar ul-next / Ultralight para renderização real
    }

    // ── Botão Voltar ─────────────────────────────────────────────────────────
    btn_back.set_callback(move |_| {
        let mut s = state_back.lock().unwrap();
        if let Some(url) = s.go_back() {
            url_input_back.set_value(&url);
            render_url(&mut viewport_back, &mut status_back, &url);
        }
    });

    // ── Botão Avançar ────────────────────────────────────────────────────────
    btn_forward.set_callback(move |_| {
        let mut s = state_fwd.lock().unwrap();
        if let Some(url) = s.go_forward() {
            url_input_fwd.set_value(&url);
            render_url(&mut viewport_fwd, &mut status_fwd, &url);
        }
    });

    // ── Botão Recarregar ─────────────────────────────────────────────────────
    btn_reload.set_callback(move |_| {
        let s = state_reload.lock().unwrap();
        let url = s.current_url.clone();
        drop(s);
        url_input_reload.set_value(&url);
        render_url(&mut viewport_reload, &mut status_reload, &url);
    });

    // ── Botão Parar ──────────────────────────────────────────────────────────
    btn_stop.set_callback(move |_| {
        let mut s = state_stop.lock().unwrap();
        s.finish_loading();
        viewport_stop.set_label("Carregamento interrompido.");
        status_stop.set_label(" Parado");
        viewport_stop.redraw();
        status_stop.redraw();
    });

    // ── Botão Ir ─────────────────────────────────────────────────────────────
    btn_go.set_callback(move |_| {
        let raw = url_input_go.value();
        let mut s = state_go.lock().unwrap();
        s.navigate(&raw);
        let url = s.current_url.clone();
        drop(s);
        url_input_go.set_value(&url);
        render_url(&mut viewport_go, &mut status_go, &url);
    });

    // ── Enter na barra de endereço ───────────────────────────────────────────
    url_input.handle(move |inp, ev| {
        if ev == fltk::enums::Event::KeyDown {
            if app::event_key() == fltk::enums::Key::Enter {
                let raw = inp.value();
                let mut s = state_enter.lock().unwrap();
                s.navigate(&raw);
                let url = s.current_url.clone();
                drop(s);
                url_input_enter.set_value(&url);
                render_url(&mut viewport_enter, &mut status_enter, &url);
                return true;
            }
        }
        false
    });

    // Renderiza a URL inicial
    {
        let s = state.lock().unwrap();
        let url = s.current_url.clone();
        drop(s);
        viewport.set_label(&format!(
            "Motor Ultralight v0.1.7  |  ul-next v0.5.4\nCarregando: {}",
            url
        ));
        status_bar.set_label(&format!(" Conectando a {}", url));
    }

    app.run().unwrap();
}
