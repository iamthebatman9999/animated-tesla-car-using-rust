#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui_tesla::app::MyApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use egui::IconData;

    // tracing_subscriber::fmt::init();

    let icon_image = image::open("icon.png").expect("Should be able to open icon PNG file");
    let width = icon_image.width();
    let height = icon_image.height();
    let icon_rgba8 = icon_image.into_rgba8().to_vec();
    let icon_data = IconData {
            rgba: icon_rgba8,
            width,
            height,
    };
    
    // let native_options = eframe::NativeOptions::default();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 800.0]) // iPhone 11 dimensions
            .with_resizable(true) // Lock the window size
            .with_icon(icon_data),
        ..Default::default()
    };
    eframe::run_native(
        "Egui Tesla App",
        native_options,
        Box::new(|_cc| {
            egui_extras::install_image_loaders(&_cc.egui_ctx);
            Ok(Box::new(MyApp::new(_cc)))
        }),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;
    let web_options = eframe::WebOptions::default();
    let element = eframe::web_sys::window()
        .expect("failed to get window")
        .document()
        .expect("failed to get document")
        .get_element_by_id("canvas")
        .expect("failed to get canvas element")
        .dyn_into::<eframe::web_sys::HtmlCanvasElement>()
        .unwrap();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                element,
                web_options,
                Box::new(|_cc| {
                    egui_extras::install_image_loaders(&_cc.egui_ctx);
                    Ok(Box::new(MyApp::new(_cc)))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
