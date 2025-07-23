use std::time::{Duration, Instant};
use egui::{TextureHandle, TextureOptions};
use crate::utils::{load_svg_from_bytes, load_png};

pub const UV: egui::Rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

const COOL_SVG: &[u8] = include_bytes!("../assets/icons/coolShape.svg");
const HEAT_SVG: &[u8] = include_bytes!("../assets/icons/heatShape.svg");
const TEMP_SVG_WIDTH: u32 = 40;
const TEMP_SVG_HEIGHT: u32 = 40;

const COOL_PNG: &[u8] = include_bytes!("../assets/images/Cool_glow_2.png");
const HEAT_PNG: &[u8] = include_bytes!("../assets/images/Hot_glow_4.png");

const UP_SVG: &[u8] = include_bytes!("../assets/icons/arrow_drop_up.svg");
const DOWN_SVG: &[u8] = include_bytes!("../assets/icons/arrow_drop_down.svg");
const ARROW_SVG_WIDTH: u32 = 36;
const ARROW_SVG_HEIGHT: u32 = 36;

// const TEMP_PNG_WIDTH: u32 = 347;
// const TEMP_PNG_HEIGHT: u32 = 1490;

#[derive(PartialEq)]
pub enum TempType {
    Cool,
    Heat
}
#[derive(Default)]
pub struct TempState {
    pub animation_temp: Option<Instant>,
    pub animation_temp_icon: Option<Instant>,
    pub animation_temp_glow: Option<Instant>,
    pub animation_temp_fade: Option<Instant>,
    pub progress_temp: f32,
    progress_temp_icon: f32,
    progress_temp_glow: f32,
    progress_temp_fade: f32,
    pub rect_right: Option<egui::Rect>,
    pub is_temp: bool,
    pub temp_type: Option<TempType>,
    cool_texture: Option<TextureHandle>,
    heat_texture: Option<TextureHandle>,
    cool_glow_texture: Option<TextureHandle>,
    heat_glow_texture: Option<TextureHandle>,
    arrow_up_texture: Option<TextureHandle>,
    arrow_down_texture: Option<TextureHandle>,
    pub temp: i32,
}

impl TempState {
    pub fn new() -> Self {
        TempState {
            animation_temp: None,
            animation_temp_icon: None,
            animation_temp_glow: None,
            animation_temp_fade: None,
            progress_temp: 0.0,
            progress_temp_icon: 0.0,
            progress_temp_glow: 0.0,
            progress_temp_fade: 0.0,
            rect_right: Some(egui::Rect::NOTHING),
            is_temp: false,
            temp_type: Some(TempType::Cool),
            cool_texture: None,
            heat_texture: None,
            cool_glow_texture: None,
            heat_glow_texture: None,
            arrow_up_texture: None,
            arrow_down_texture: None,
            temp: 29,
        }
    }

    pub fn animate_temp(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_temp {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_temp = None;
                self.animation_temp_glow = Some(Instant::now());
                self.animation_temp_fade = Some(Instant::now());
            } else {
                self.progress_temp = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.progress_temp_glow = 0.0;
            }
        }
    }

    pub fn animate_temp_icon(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_temp_icon {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_temp_icon = None;
                self.animation_temp_glow = Some(Instant::now());
            } else {
                self.progress_temp_icon = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.progress_temp_glow = 0.0;
            }
        }
    }

    pub fn animate_temp_glow(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_temp_glow {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_temp_glow = None;
            } else {
                self.progress_temp_glow = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
            }
        }
    }

    pub fn animate_temp_fade(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_temp_fade {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_temp_fade = None;
            } else {
                self.progress_temp_fade = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
            }
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, rect: egui::Rect) -> [egui::Response; 4] {
        if self.cool_texture.is_none() {
            let image = load_svg_from_bytes(COOL_SVG, TEMP_SVG_WIDTH, TEMP_SVG_HEIGHT).unwrap();
            let tex = ui.ctx().load_texture("cool", image, TextureOptions::default());
            self.cool_texture = Some(tex);
        }
        if self.heat_texture.is_none() {
            let image = load_svg_from_bytes(HEAT_SVG, TEMP_SVG_WIDTH, TEMP_SVG_HEIGHT).unwrap();
            let tex = ui.ctx().load_texture("heat", image, TextureOptions::default());
            self.heat_texture = Some(tex);
        }
        if self.cool_glow_texture.is_none() {
            let image = load_png(COOL_PNG).unwrap();
            let tex = ui.ctx().load_texture("cool_glow", image, TextureOptions::default());
            self.cool_glow_texture = Some(tex);
        }
        if self.heat_glow_texture.is_none() {
            let image = load_png(HEAT_PNG).unwrap();
            let tex = ui.ctx().load_texture("heat_glow", image, TextureOptions::default());
            self.heat_glow_texture = Some(tex);
        }
        if self.arrow_up_texture.is_none() {
            let image = load_svg_from_bytes(UP_SVG, ARROW_SVG_WIDTH, ARROW_SVG_HEIGHT).unwrap();
            let tex = ui.ctx().load_texture("up", image, TextureOptions::default());
            self.arrow_up_texture = Some(tex);
        }
        if self.arrow_down_texture.is_none() {
            let image = load_svg_from_bytes(DOWN_SVG, ARROW_SVG_WIDTH, ARROW_SVG_HEIGHT).unwrap();
            let tex = ui.ctx().load_texture("down", image, TextureOptions::default());
            self.arrow_down_texture = Some(tex);
        }
        
        if self.animation_temp_fade.is_none() {
            self.progress_temp_fade = 1.0;
        }

        let painter = ui.painter();

        let delta_y: f32 = 40.0;
        let mut y_end: f32 = 80.0;
        let mut y_start: f32 = y_end + delta_y;
        let mut y_end2: f32 = rect.center().y;
        let mut y_start2: f32 = y_end2 + delta_y;
        let mut y_end3: f32 = rect.max.y - 90.0;
        let mut y_start3: f32 = y_end3 + delta_y;
        let mut y_end4: f32 = rect.max.y - 70.0;
        let mut y_start4: f32 = y_end4 + delta_y;
        let mut y_end5: f32 = rect.max.y - 40.0;
        let mut y_start5: f32 = y_end5 + delta_y;
        let mut progress = self.progress_temp_fade;
        if self.animation_temp_fade.is_none() {
            self.progress_temp_fade = 1.0;
            progress = self.progress_temp_fade;
        } else {
            if !self.is_temp {
                progress = 1.0 - self.progress_temp_fade;
                (y_start, y_end) = (y_end, y_start);
                (y_start2, y_end2) = (y_end2, y_start2);
                (y_start3, y_end3) = (y_end3, y_start3);
                (y_start4, y_end4) = (y_end4, y_start4);
                (y_start5, y_end5) = (y_end5, y_start5);
            }
        }

        let y = y_start + (y_end - y_start) * self.progress_temp_fade;
            
        let rect_cool_small = egui::Rect::from_center_size(egui::Pos2{x: TEMP_SVG_WIDTH as f32 / 2.0 + 10.0, y: y}, egui::Vec2::new(TEMP_SVG_WIDTH as f32, TEMP_SVG_HEIGHT as f32));
        let rect_heat_small = egui::Rect::from_center_size(egui::Pos2{x: TEMP_SVG_WIDTH as f32 + 70.0, y: y}, egui::Vec2::new(TEMP_SVG_WIDTH as f32, TEMP_SVG_HEIGHT as f32));
        
        let rect_cool_big = egui::Rect {
            min: egui::Pos2{x: rect_cool_small.min.x, y: rect_cool_small.min.y},
            max: egui::Pos2{x: rect_cool_small.max.x + 20.0, y: rect_cool_small.max.y + 20.0},
        };
        let rect_heat_big = egui::Rect {
            min: egui::Pos2{x: rect_heat_small.min.x - 20.0, y: rect_heat_small.min.y},
            max: egui::Pos2{x: rect_heat_small.max.x, y: rect_heat_small.max.y + 20.0},
        };

        let mut opacity_cool = egui::Color32::from_rgba_premultiplied(83, 249, 255, 255);
        let mut opacity_heat = egui::Color32::from_rgba_premultiplied(255, 83, 104, 255);

        let mut rect_cool = rect_cool_small;
        let mut rect_heat = rect_heat_small;

        // animate icon
        if self.animation_temp_icon.is_none() {
            if self.is_temp {
                self.progress_temp_icon = 1.0;
            }
        }

        // animate glow
        if self.animation_temp_glow.is_none() {
            if self.is_temp {
                self.progress_temp_glow = 1.0;
            }
        }

        let mut rect_arrow_up: egui::Rect = egui::Rect::NOTHING;
        let mut rect_arrow_down: egui::Rect = egui::Rect::NOTHING;

        if self.is_temp {
            if self.animation_temp.is_none() {
                let opacity = egui::Color32::from_white_alpha((progress * 255.0) as u8);
                let y2 = y_start2 + (y_end2 - y_start2) * self.progress_temp_fade;
                let position = egui::Pos2::new(rect.center().x / 2.0, y2);
                let f = format!("{}°C", self.temp);
                painter.text(position, egui::Align2::CENTER_CENTER, f, egui::FontId::new(80.0, egui::FontFamily::Proportional), opacity);
                // buat panah atas
                let y2_up = y2 - 55.0;
                let y2_down = y2 + 55.0;
                if let Some(tex) = &self.arrow_up_texture {
                    let position = egui::Pos2::new(rect.center().x / 2.0, y2_up);
                    let rect = egui::Rect::from_center_size(position, egui::Vec2::new(ARROW_SVG_WIDTH as f32, ARROW_SVG_HEIGHT as f32));
                    rect_arrow_up = rect;
                    painter.image(tex.id(), rect, UV, opacity);
                }
                if let Some(tex) = &self.arrow_down_texture {
                    let position = egui::Pos2::new(rect.center().x / 2.0, y2_down);
                    let rect = egui::Rect::from_center_size(position, egui::Vec2::new(ARROW_SVG_WIDTH as f32, ARROW_SVG_HEIGHT as f32));
                    rect_arrow_down = rect;
                    painter.image(tex.id(), rect, UV, opacity);
                }
                let y3 = y_start3 + (y_end3 - y_start3) * self.progress_temp_fade;
                let position = egui::Pos2::new(rect.center().x / 2.0, y3);
                painter.text(position, egui::Align2::CENTER_CENTER, "CURRENT TEMPERATURE", egui::FontId::new(15.0, egui::FontFamily::Proportional), opacity);
                let y4 = y_start4 + (y_end4 - y_start4) * self.progress_temp_fade;
                let position = egui::Pos2::new(rect.center().x / 4.0, y4);
                painter.text(position, egui::Align2::CENTER_CENTER, "INSIDE", egui::FontId::new(15.0, egui::FontFamily::Proportional), opacity);
                let position = egui::Pos2::new(10.0 + rect.center().x / 2.0, y4);
                painter.text(position, egui::Align2::CENTER_CENTER, "OUTSIDE", egui::FontId::new(15.0, egui::FontFamily::Proportional), opacity);
                let y5 = y_start5 + (y_end5 - y_start5) * self.progress_temp_fade;
                let position = egui::Pos2::new(rect.center().x / 4.0, y5);
                painter.text(position, egui::Align2::CENTER_CENTER, "20°C", egui::FontId::new(25.0, egui::FontFamily::Proportional), opacity);
                let position = egui::Pos2::new(10.0 + rect.center().x / 2.0, y5);
                painter.text(position, egui::Align2::CENTER_CENTER, "35°C", egui::FontId::new(25.0, egui::FontFamily::Proportional), opacity);

                match self.temp_type {
                    Some(TempType::Cool) => {
                        // blue
                        let rect_progress_cool = egui::Rect {
                            min: egui::Pos2{
                                x: rect_cool_small.min.x,
                                y: rect_cool_small.min.y
                            },
                            max: egui::Pos2{
                                x: rect_cool_small.max.x + ((rect_cool_big.max.x - rect_cool_small.max.x) * self.progress_temp_icon),
                                y: rect_cool_small.max.y + ((rect_cool_big.max.y - rect_cool_small.max.y) * self.progress_temp_icon)
                            },
                        };
                        let pos_title_progress = egui::Pos2{x:rect_progress_cool.center().x, y: rect_progress_cool.max.y + 10.0};
                        if let Some(tex) = &self.cool_texture {
                            painter.image(tex.id(), rect_progress_cool, UV, opacity_cool);
                            painter.text(pos_title_progress, egui::Align2::CENTER_TOP, "COOL", egui::FontId::new(18.0, egui::FontFamily::Proportional), opacity_cool);
                        }
                        // red
                        opacity_heat = egui::Color32::from_rgba_premultiplied(255, 255, 255, 98);
                        let rect_progress_heat = egui::Rect {
                            min: egui::Pos2{
                                x: rect_heat_small.min.x + ((rect_heat_big.min.x - rect_heat_small.min.x) * (1.0 - self.progress_temp_icon)),
                                y: rect_heat_small.min.y
                            },
                            max: egui::Pos2{
                                x: rect_heat_small.max.x,
                                y: rect_heat_small.max.y + ((rect_heat_big.max.y - rect_heat_small.max.y) * (1.0 - self.progress_temp_icon))
                            },
                        };
                        let pos_title_progress = egui::Pos2{x:rect_progress_heat.center().x, y: rect_progress_heat.max.y + 10.0};
                        if let Some(tex) = &self.heat_texture {
                            painter.image(tex.id(), rect_progress_heat, UV, opacity_heat);
                            painter.text(pos_title_progress, egui::Align2::CENTER_TOP, "HEAT", egui::FontId::new(18.0, egui::FontFamily::Proportional), opacity_heat);                
                        }
                        rect_cool = rect_progress_cool;
                    },
                    Some(TempType::Heat) => {
                        // red
                        let rect_progress_heat = egui::Rect {
                            min: egui::Pos2{
                                x: rect_heat_small.min.x + ((rect_heat_big.min.x - rect_heat_small.min.x) * self.progress_temp_icon),
                                y: rect_heat_small.min.y
                            },
                            max: egui::Pos2{
                                x: rect_heat_small.max.x,
                                y: rect_heat_small.max.y + ((rect_heat_big.max.y - rect_heat_small.max.y) * self.progress_temp_icon)
                            },
                        };
                        let pos_title_progress = egui::Pos2{x: rect_progress_heat.center().x, y: rect_progress_heat.max.y + 10.0};
                        if let Some(tex) = &self.heat_texture {
                            painter.image(tex.id(), rect_progress_heat, UV, opacity_heat);
                            painter.text(pos_title_progress, egui::Align2::CENTER_TOP, "HEAT", egui::FontId::new(18.0, egui::FontFamily::Proportional), opacity_heat);
                        }
                        // blue
                        opacity_cool = egui::Color32::from_rgba_premultiplied(255, 255, 255, 98);
                        let rect_progress_cool = egui::Rect {
                            min: egui::Pos2{
                                x: rect_cool_small.min.x,
                                y: rect_cool_small.min.y
                            },
                            max: egui::Pos2{
                                x: rect_cool_small.max.x + ((rect_cool_big.max.x - rect_cool_small.max.x) * (1.0 - self.progress_temp_icon)),
                                y: rect_cool_small.max.y + ((rect_cool_big.max.y - rect_cool_small.max.y) * (1.0 - self.progress_temp_icon))
                            },
                        };
                        let pos_title_progress = egui::Pos2{x:rect_progress_cool.center().x, y: rect_progress_cool.max.y + 10.0};
                        if let Some(tex) = &self.cool_texture {
                            painter.image(tex.id(), rect_progress_cool, UV, opacity_cool);
                            painter.text(pos_title_progress, egui::Align2::CENTER_TOP, "COOL", egui::FontId::new(18.0, egui::FontFamily::Proportional), opacity_cool);                
                        }
                       rect_heat = rect_progress_heat;
                    },
                    _ => (),
                }
    

            }
        }

        // animate glow
        if self.is_temp {
            if self.animation_temp_icon.is_none() && self.animation_temp.is_none() {
                let mut rect = self.rect_right.unwrap();
                let size = self.rect_right.unwrap().size();
                rect.min.x =  rect.max.x - size.x * self.progress_temp_glow;
                rect.max.x = rect.min.x + size.x;
                match self.temp_type {
                    Some(TempType::Cool) => {
                        let image_source: egui::ImageSource = egui::include_image!("../assets/images/Cool_glow_2.png");
                        let z = egui::Image::new(image_source).tint(egui::Color32::WHITE);
                        z.show_loading_spinner(false).paint_at(ui, rect);
                    },
                    Some(TempType::Heat) => {
                        let image_source: egui::ImageSource = egui::include_image!("../assets/images/Hot_glow_4.png");
                        let z = egui::Image::new(image_source).tint(egui::Color32::WHITE);
                        z.show_loading_spinner(false).paint_at(ui, rect);
                    },
                    _ => (),
                }
            }
        }

        [
            ui.allocate_rect(rect_cool, egui::Sense::click()),
            ui.allocate_rect(rect_heat, egui::Sense::click()),
            ui.allocate_rect(rect_arrow_up, egui::Sense::click()),
            ui.allocate_rect(rect_arrow_down, egui::Sense::click())
        ]
    }

}

