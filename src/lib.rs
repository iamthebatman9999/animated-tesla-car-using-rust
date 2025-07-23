pub mod utils;
// pub mod safearea;
pub mod app;
pub mod lock;
pub mod charge;
pub mod temp;
pub mod tyre;

// use eframe::{egui, CreationContext};
#[cfg(target_os = "android")]
use egui_winit::winit;
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use eframe::Renderer;
    use crate::app::MyApp;
    // Log to android output
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        renderer: Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
    .unwrap()
}
