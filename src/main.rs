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
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        run_and_return: Default::default(),
        event_loop_builder: Default::default(),
        window_builder: Default::default(),
        shader_version: Default::default(),
        centered: true,
        persist_window: false,
        viewport: Default::default(),
        persistence_path: Default::default(),
        dithering: Default::default(),
    };
    eframe::run_native(
        "TypeFast",
        native_options,
        Box::new(|cc| Ok(Box::new(app::TypeFastApp::new(cc)))),
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
fn main() {
    use app::TypeFastApp;
    use eframe::wasm_bindgen::JsCast as _;
    use eframe::web_sys;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("type_fast")
            .expect("Failed to find type_fast")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("type_fast was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(TypeFastApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
