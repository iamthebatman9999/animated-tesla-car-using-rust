use std::time::{Duration, Instant};
use egui::{Color32, CornerRadius, Stroke, TextureHandle, TextureOptions};
use crate::utils::load_svg_from_bytes;

pub const UV: egui::Rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

const TYRE_SVG: &[u8] = include_bytes!("../assets/icons/FL_Tyre.svg");
const TYRE_SVG_WIDTH: u32 = 28;
const TYRE_SVG_HEIGHT: u32 = 81;

#[derive(Default)]
pub struct TyreState {
    pub is_tyre: bool,
    pub animation_tyre_left_up: Option<Instant>,
    pub animation_tyre_right_up: Option<Instant>,
    pub animation_tyre_left_down: Option<Instant>,
    pub animation_tyre_right_down: Option<Instant>,
    tyre_texture: Option<TextureHandle>,
    pub rect_car: Option<egui::Rect>,
    left_up_tyre: egui::Pos2,
    right_up_tyre: egui::Pos2,
    left_down_tyre: egui::Pos2,
    right_down_tyre: egui::Pos2,
    progress_left_up: f32,
    progress_right_up: f32,
    progress_left_down: f32,
    progress_right_down: f32,
}

impl TyreState {
    pub fn new(&mut self) -> Self {
        TyreState {
            is_tyre: false,
            tyre_texture: None,
            rect_car: None,
            animation_tyre_left_up: None,
            animation_tyre_right_up: None,
            animation_tyre_left_down: None,
            animation_tyre_right_down: None,
            left_up_tyre: egui::pos2(0.0, 0.0),
            right_up_tyre: egui::pos2(0.0, 0.0),
            left_down_tyre: egui::pos2(0.0, 0.0),
            right_down_tyre: egui::pos2(0.0, 0.0),
            progress_left_up: 0.0,
            progress_right_up: 0.0,
            progress_left_down: 0.0,
            progress_right_down: 0.0,
        }
    }

    pub fn animate_tyre_left_up(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_tyre_left_up {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_tyre_left_up = None;
                self.animation_tyre_right_up = Some(Instant::now());
                self.progress_left_up = 1.0;
            } else {
                self.progress_left_up = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.progress_right_up = 0.0;
                self.progress_left_down = 0.0;
                self.progress_right_down = 0.0;
            }
        }
    }

    pub fn animate_tyre_right_up(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_tyre_right_up {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_tyre_right_up = None;
                self.animation_tyre_right_down = Some(Instant::now());
                self.progress_right_up = 1.0;
            } else {
                self.progress_right_up = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.progress_left_up = 1.0;
                self.progress_left_down = 0.0;
                self.progress_right_down = 0.0;
            }
        }
    }

    pub fn animate_tyre_right_down(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_tyre_right_down {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_tyre_right_down = None;
                self.animation_tyre_left_down = Some(Instant::now());
                self.progress_right_down = 1.0;
            } else {
                self.progress_right_down = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.progress_left_up = 1.0;
                self.progress_right_up = 1.0;
                self.progress_left_down = 0.0;
            }
        }
    }

    pub fn animate_tyre_left_down(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_tyre_left_down {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_tyre_left_down = None;
                self.progress_left_down = 1.0;
            } else {
                self.progress_left_down = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.progress_left_up = 1.0;
                self.progress_right_up = 1.0;
                self.progress_right_down = 1.0;
            }
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, rect_layout: egui::Rect, rect: egui::Rect) {
        if self.tyre_texture.is_none() {
            let image = load_svg_from_bytes(TYRE_SVG, TYRE_SVG_WIDTH, TYRE_SVG_HEIGHT).unwrap();
            let tex = ui.ctx().load_texture("tyre", image, TextureOptions::default());
            self.tyre_texture = Some(tex);
        }

        let painter = ui.painter();

        let rect_left_up: egui::Rect = egui::Rect {
            min: egui::pos2(10.0, 10.0),
            max: egui::pos2(rect_layout.center().x - 10.0, rect_layout.center().y - 10.0)
        };
        let rect_right_up: egui::Rect = egui::Rect {
            min: egui::pos2(rect_layout.center().x + 10.0, 10.0),
            max: egui::pos2(rect_layout.max.x - 10.0, rect_layout.center().y - 10.0)
        };
        let rect_left_down: egui::Rect = egui::Rect {
            min: egui::pos2(10.0, rect_layout.center().y + 10.0),
            max: egui::pos2(rect_layout.center().x - 10.0, rect_layout.center().y * 2.0 - 10.0)
        };
        let rect_right_down: egui::Rect = egui::Rect {
            min: egui::pos2(rect_layout.center().x + 10.0, rect_layout.center().y + 10.0),
            max: egui::pos2(rect_layout.max.x - 10.0, rect_layout.center().y * 2.0 - 10.0)
        };

        if self.is_tyre {
            if let Some(tex) = &self.tyre_texture {
                self.left_up_tyre = egui::pos2(rect.min.x + 30.0, rect.min.y + 40.0 + TYRE_SVG_HEIGHT as f32);
                self.right_up_tyre = egui::pos2(rect.max.x - 30.0, rect.min.y + 40.0 + TYRE_SVG_HEIGHT as f32);
                self.left_down_tyre = egui::pos2(rect.min.x + 30.0, rect.max.y - 50.0 - TYRE_SVG_HEIGHT as f32);
                self.right_down_tyre = egui::pos2(rect.max.x - 30.0, rect.max.y - 50.0 - TYRE_SVG_HEIGHT as f32);
                for tyre in vec![
                    self.left_up_tyre,
                    self.right_up_tyre,
                    self.left_down_tyre,
                    self.right_down_tyre
                ] {
                    let rect_tyre = egui::Rect::from_center_size(tyre, egui::Vec2::new(TYRE_SVG_WIDTH as f32, TYRE_SVG_HEIGHT as f32));
                    painter.image(tex.id(), rect_tyre, UV, egui::Color32::from_white_alpha(128));
                }
            }
        }

        if self.animation_tyre_left_up.is_some() {
            let mut progress = self.progress_left_up;
            if !self.is_tyre && self.progress_left_up != 1.0 {
                progress = 1.0 - self.progress_left_up;
            }
            let rect3 = egui::Rect::from_center_size(rect_left_up.center(), rect_left_up.size() * progress);
            painter.rect_filled(rect3, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(30, 0, 0, 5));
            painter.rect_stroke(rect3, CornerRadius::same(5), Stroke::new(2.0, Color32::RED), egui::StrokeKind::Middle);
        } else {
            if self.is_tyre {
                let position = egui::Pos2::new(rect_left_up.center().x, rect_left_up.min.y + 10.0);
                let position2 = egui::Pos2::new(rect_left_up.center().x, rect_left_up.min.y + 60.0);
                let position3 = egui::Pos2::new(rect_left_up.center().x, rect_left_up.max.y - 50.0);
                let position4 = egui::Pos2::new(rect_left_up.center().x, rect_left_up.max.y - 20.0);
                painter.rect_filled(rect_left_up, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(30, 0, 0, 5));
                painter.rect_stroke(rect_left_up, CornerRadius::same(5), Stroke::new(2.0, Color32::RED), egui::StrokeKind::Middle);
                painter.text(position, egui::Align2::CENTER_TOP, "23.6psi", egui::FontId::new(40.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                painter.text(position2, egui::Align2::CENTER_TOP, "56°C", egui::FontId::new(20.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                painter.text(position3, egui::Align2::CENTER_BOTTOM, "LOW", egui::FontId::new(60.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                painter.text(position4, egui::Align2::CENTER_BOTTOM, "PRESSURE", egui::FontId::new(25.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
            }
        }
        
        if self.animation_tyre_right_up.is_some() {
            let mut progress = self.progress_right_up;
            if !self.is_tyre && self.progress_right_up != 1.0 {
                progress = 1.0 - self.progress_right_up;
            }
            let rect3 = egui::Rect::from_center_size(rect_right_up.center(), rect_right_up.size() * progress);
            painter.rect_filled(rect3, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
            painter.rect_stroke(rect3, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
        } else {
            if self.animation_tyre_left_up.is_some() {
                if !self.is_tyre {
                    let progress = 1.0;
                    let rect3 = egui::Rect::from_center_size(rect_right_up.center(), rect_right_up.size() * progress);
                    painter.rect_filled(rect3, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
                    painter.rect_stroke(rect3, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
                }
            }
            if self.animation_tyre_left_up.is_none() {
                if self.is_tyre || self.animation_tyre_left_up.is_some() {
                    let position = egui::Pos2::new(rect_right_up.center().x, rect_right_up.min.y + 10.0);
                    let position2 = egui::Pos2::new(rect_right_up.center().x, rect_right_up.min.y + 60.0);
                    let position3 = egui::Pos2::new(rect_right_up.center().x, rect_right_up.max.y - 50.0);
                    let position4 = egui::Pos2::new(rect_right_up.center().x, rect_right_up.max.y - 20.0);
                    painter.rect_filled(rect_right_up, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
                    painter.rect_stroke(rect_right_up, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
                    painter.text(position, egui::Align2::CENTER_TOP, "35.0psi", egui::FontId::new(40.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position2, egui::Align2::CENTER_TOP, "41°C", egui::FontId::new(20.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position3, egui::Align2::CENTER_BOTTOM, "LOW", egui::FontId::new(60.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position4, egui::Align2::CENTER_BOTTOM, "PRESSURE", egui::FontId::new(25.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                }
            }
        }

        if self.animation_tyre_right_down.is_some() {
            let mut progress = self.progress_right_down;
            if !self.is_tyre && self.progress_right_down != 1.0 {
                progress = 1.0 - self.progress_right_down;
            }
            let rect3 = egui::Rect::from_center_size(rect_right_down.center(), rect_right_down.size() * progress);
            painter.rect_filled(rect3, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
            painter.rect_stroke(rect3, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
        } else {
            if self.animation_tyre_left_up.is_some() || self.animation_tyre_right_up.is_some() {
                if !self.is_tyre {
                    let progress = 1.0;
                    let rect3 = egui::Rect::from_center_size(rect_right_down.center(), rect_right_down.size() * progress);
                    painter.rect_filled(rect3, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
                    painter.rect_stroke(rect3, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
                }
            }
            if self.animation_tyre_left_up.is_none() && self.animation_tyre_right_up.is_none() {
                if self.is_tyre || self.animation_tyre_left_up.is_some() || self.animation_tyre_right_up.is_some() {
                    let position = egui::Pos2::new(rect_right_down.center().x, rect_right_down.min.y + 10.0);
                    let position2 = egui::Pos2::new(rect_right_down.center().x, rect_right_down.min.y + 60.0);
                    let position3 = egui::Pos2::new(rect_right_down.center().x, rect_right_down.max.y - 50.0);
                    let position4 = egui::Pos2::new(rect_right_down.center().x, rect_right_down.max.y - 20.0);
                    painter.rect_filled(rect_right_down, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
                    painter.rect_stroke(rect_right_down, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
                    painter.text(position, egui::Align2::CENTER_TOP, "34.8psi", egui::FontId::new(40.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position2, egui::Align2::CENTER_TOP, "42°C", egui::FontId::new(20.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position3, egui::Align2::CENTER_BOTTOM, "LOW", egui::FontId::new(60.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position4, egui::Align2::CENTER_BOTTOM, "PRESSURE", egui::FontId::new(25.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                }
            }
        }

        if self.animation_tyre_left_down.is_some() {
            let mut progress = self.progress_left_down;
            if !self.is_tyre && self.progress_left_down != 1.0 {
                progress = 1.0 - self.progress_left_down;
            }
            let rect3 = egui::Rect::from_center_size(rect_left_down.center(), rect_left_down.size() * progress);
            painter.rect_filled(rect3, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
            painter.rect_stroke(rect3, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
        } else {
            if self.animation_tyre_left_up.is_some() || self.animation_tyre_right_up.is_some() || self.animation_tyre_right_down.is_some() {
                if !self.is_tyre {
                    let progress = 1.0;
                    let rect3 = egui::Rect::from_center_size(rect_left_down.center(), rect_left_down.size() * progress);
                    painter.rect_filled(rect3, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
                    painter.rect_stroke(rect3, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
                }
            }
            if self.animation_tyre_left_up.is_none() && self.animation_tyre_right_up.is_none() && self.animation_tyre_right_down.is_none() {
                if self.is_tyre {
                    let position = egui::Pos2::new(rect_left_down.center().x, rect_left_down.min.y + 10.0);
                    let position2 = egui::Pos2::new(rect_left_down.center().x, rect_left_down.min.y + 60.0);
                    let position3 = egui::Pos2::new(rect_left_down.center().x, rect_left_down.max.y - 50.0);
                    let position4 = egui::Pos2::new(rect_left_down.center().x, rect_left_down.max.y - 20.0);
                    painter.rect_filled(rect_left_down, CornerRadius::same(5), egui::Color32::from_rgba_premultiplied(0, 30, 30, 5));
                    painter.rect_stroke(rect_left_down, CornerRadius::same(5), Stroke::new(2.0, Color32::CYAN), egui::StrokeKind::Middle);
                    painter.text(position, egui::Align2::CENTER_TOP, "34.6psi", egui::FontId::new(40.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position2, egui::Align2::CENTER_TOP, "41°C", egui::FontId::new(20.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position3, egui::Align2::CENTER_BOTTOM, "LOW", egui::FontId::new(60.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                    painter.text(position4, egui::Align2::CENTER_BOTTOM, "PRESSURE", egui::FontId::new(25.0, egui::FontFamily::Proportional), egui::Color32::WHITE);
                }
            }
        }
    }
}