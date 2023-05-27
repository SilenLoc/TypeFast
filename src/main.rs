#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod current;
mod random;
mod scoring;
mod settings;
mod typewriter;

#[cfg(target_os = "linux")]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions {
        active: true,
        app_id: Some("type".to_owned()),
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: Option::from(egui::Pos2::new(300_f32, 100_f32)),
        initial_window_size: Option::from(egui::Vec2::new(1000_f32, 1000_f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        mouse_passthrough: Default::default(),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: Default::default(),
        default_theme: eframe::Theme::Dark,
        run_and_return: Default::default(),
        event_loop_builder: Default::default(),
        shader_version: Default::default(),
        centered: true,
    };
    eframe::run_native(
        "TypeFast",
        native_options,
        Box::new(|cc| Box::new(app::TypeFastApp::new(cc))),
    )
}

#[cfg(target_os = "windows")]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions {
        active: true,
        app_id: Some("type".to_owned()),
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: Option::from(egui::Pos2::new(300_f32, 100_f32)),
        initial_window_size: Option::from(egui::Vec2::new(1000_f32, 1000_f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        mouse_passthrough: Default::default(),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: Default::default(),
        default_theme: eframe::Theme::Dark,
        run_and_return: Default::default(),
        event_loop_builder: Default::default(),
        shader_version: Default::default(),
        centered: true,
    };
    eframe::run_native(
        "TypeFast",
        native_options,
        Box::new(|cc| Box::new(TypeFastApp::new(cc))),
    )
}

#[cfg(target_os = "macos")]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        active: true,
        app_id: Some("type".to_owned()),
        fullsize_content: false,
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: Option::from(egui::Pos2::new(300_f32, 100_f32)),
        initial_window_size: Option::from(egui::Vec2::new(1000_f32, 1000_f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        mouse_passthrough: Default::default(),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: Default::default(),
        default_theme: eframe::Theme::Dark,
        run_and_return: Default::default(),
        event_loop_builder: Default::default(),
        shader_version: Default::default(),
        centered: true,
    };
    eframe::run_native(
        "TypeFast",
        native_options,
        Box::new(|cc| Box::new(TypeFastApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
mod web {
    use eframe::WebRunner;
    use wasm_bindgen::prelude::*;

    use crate::app::TypeFastApp;

    #[derive(Clone)]
    #[wasm_bindgen]
    pub struct WebHandle {
        runner: eframe::WebRunner,
    }

    #[wasm_bindgen]
    impl WebHandle {
        /// Installs a panic hook, then returns.
        #[allow(clippy::new_without_default)]
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            // Redirect [`log`] message to `console.log` and friends:
            eframe::WebLogger::init(log::LevelFilter::Debug).ok();

            Self {
                runner: WebRunner::new(),
            }
        }

        /// Call this once from JavaScript to start your app.
        #[wasm_bindgen]
        pub async fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
            self.runner
                .start(
                    canvas_id,
                    eframe::WebOptions::default(),
                    Box::new(|cc| Box::new(TypeFastApp::new(cc))),
                )
                .await
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        let w = web::WebHandle::new();
        w.start("type_fast").await.expect("failed to start in web")
    });
}
