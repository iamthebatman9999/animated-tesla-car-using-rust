use egui::{Vec2};

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;

pub fn ease_in_out_back(t: f32) -> f32 {
	if t < 0.5 {
		((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
	} else {
		((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
	}
}
pub fn fit_to_screen_size(
    image_size: egui::Vec2,
    screen_size: egui::Vec2,
) -> egui::Vec2 {
    let image_aspect = image_size.x / image_size.y;
    let screen_aspect = screen_size.x / screen_size.y;

    if image_aspect > screen_aspect {
        // Lebih lebar daripada layar, fit lebar
        egui::vec2(screen_size.x, screen_size.x / image_aspect)
    } else {
        // Lebih tinggi, fit tinggi
        egui::vec2(screen_size.y * image_aspect, screen_size.y)
    }
}

pub fn load_svg_from_bytes(
    svg_bytes: &[u8],
    width: u32,
    height: u32,
) -> Result<egui::ColorImage, Box<dyn std::error::Error>> {
    // Parse SVG from bytes
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_bytes, &opt)?;

    // Calculate transform to fit SVG in the target size
    let svg_size = rtree.size();
    let scale_x = width as f32 / svg_size.width();
    let scale_y = height as f32 / svg_size.height();
    let scale = scale_x.min(scale_y); // Maintain aspect ratio
    
    // Center the SVG
    let offset_x = (width as f32 - svg_size.width() * scale) / 2.0;
    let offset_y = (height as f32 - svg_size.height() * scale) / 2.0;

    // Create pixmap for rendering
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or("Failed to create pixmap")?;


    let transform = usvg::Transform::from_translate(offset_x, offset_y)
        .post_scale(scale, scale);
    
    // Render SVG to pixmap
    resvg::render(&rtree, transform, &mut pixmap.as_mut());
    
    // Convert pixmap to ColorImage
    let pixels: Vec<egui::Color32> = pixmap
        .pixels()
        .iter()
        .map(|p| {
            egui::Color32::from_rgba_premultiplied(p.red(), p.green(), p.blue(), p.alpha())
        })
        .collect();
    
    Ok(egui::ColorImage {
        size: [width as usize, height as usize],
        pixels,
        source_size: Vec2{x: svg_size.width(), y: svg_size.height()}.into(),
    })
}

pub fn load_png(bytes: &[u8]) -> Result<egui::ColorImage, Box<dyn std::error::Error>> {
    // Decode PNG
    let img = image::load_from_memory(bytes).expect("Failed to load image");
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8();

    // Convert to egui ColorImage
    let pixels: Vec<egui::Color32> = rgba
        .chunks(4)
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    Ok(egui::ColorImage {
        size,
        pixels,
        source_size: Vec2{x: img.width() as f32, y: img.height() as f32}.into(),
    })
}
