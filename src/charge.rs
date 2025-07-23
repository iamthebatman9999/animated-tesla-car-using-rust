use std::time::{Duration, Instant};
use egui::{Color32, TextureHandle, TextureOptions};
use crate::utils::load_svg_from_bytes;

pub const UV: egui::Rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

const BATTERY_SVG: &[u8] = include_bytes!("../assets/icons/Battery.svg");
const BATTERY_SVG_WIDTH: u32 = 483;
const BATTERY_SVG_HEIGHT: u32 = 717;
// const BATTERY_RATIO: f32 = BATTERY_SVG_WIDTH as f32 / BATTERY_SVG_HEIGHT as f32;


#[derive(Default)]
pub struct ChargeState {
    pub animation_charge: Option<Instant>,
    progress_charge: f32,
    pub is_charged: bool,
    charge_texture: Option<TextureHandle>,
}

impl ChargeState {
    pub fn new() -> Self {
        ChargeState {
            animation_charge: None,
            progress_charge: 0.0,
            is_charged: false,
            charge_texture: None,
        }
    }

    pub fn animate_charge(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_charge {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_charge = None;
            } else {
                self.progress_charge = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
            }
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        if self.charge_texture.is_none() {
            let image = load_svg_from_bytes(BATTERY_SVG, BATTERY_SVG_WIDTH, BATTERY_SVG_HEIGHT).unwrap();
            let tex_charge = ui.ctx().load_texture("battery", image, TextureOptions::default());
            self.charge_texture = Some(tex_charge);
        }

        let delta_y: f32 = 40.0;
        let mut y_end: f32 = 80.0;
        let mut y_start: f32 = y_end + delta_y;
        let mut y_end2: f32 = (rect.max.y + rect.center().y) / 2.0;
        let mut y_start2: f32 = y_end2 + delta_y;
        let mut y_end3: f32 = rect.max.y - 35.0;
        let mut y_start3: f32 = y_end3 + delta_y;
        let mut progress = self.progress_charge;
        if self.animation_charge.is_none() {
            self.progress_charge = 1.0;
            progress = self.progress_charge;
        } else {
            if !self.is_charged {
                progress = 1.0 - self.progress_charge;
                (y_start, y_end) = (y_end, y_start);
                (y_start2, y_end2) = (y_end2, y_start2);
                (y_start3, y_end3) = (y_end3, y_start3);
            }
        }

        let painter = ui.painter();

        if self.animation_charge.is_none() {
            if let Some(tex) = &self.charge_texture {
                let rect_center = rect * 0.45;
                // let ratio = rect.max.x / rect.max.y;
                // if ratio > BATTERY_RATIO {
                //     rect_center.max.x = BATTERY_RATIO * rect.max.y;
                // } else {
                //     rect_center.max.y = BATTERY_RATIO * rect.max.x;
                // }
                let center_x = rect.center().x;
                let center_y = rect.center().y;
                painter.image(tex.id(), egui::Rect::from_center_size(egui::Pos2::new(center_x, center_y), rect_center.size()), UV, Color32::WHITE);
            }
        } else {
            if let Some(tex) = &self.charge_texture {
                let rect_center = rect * 0.45;
                let center_x = rect.center().x;
                let center_y = rect.center().y;
                let opacity = Color32::from_white_alpha((progress * 255.0) as u8);
                painter.image(tex.id(), egui::Rect::from_center_size(egui::Pos2::new(center_x, center_y), rect_center.size()), UV, opacity);
            }
        }

        let y = y_start + (y_end - y_start) * self.progress_charge;
        let y2 = y_start2 + (y_end2 - y_start2) * self.progress_charge;
        let y3 = y_start3 + (y_end3 - y_start3) * self.progress_charge;
        let position = egui::Pos2::new(rect.center().x, y);
        let position2 = egui::Pos2::new(rect.center().x, position.y + 40.0);
        let position3 = egui::Pos2::new(rect.center().x, y2);
        let position4 = egui::Pos2::new(rect.center().x, position3.y + 40.0);
        let position5 = egui::Pos2::new(0.0, y3);
        let position6 = egui::Pos2::new(rect.max.x, y3);
        let opacity = Color32::from_white_alpha((progress * 255.0) as u8);
        painter.text(position, egui::Align2::CENTER_TOP, "220 mi", egui::FontId::new(36.0, egui::FontFamily::Proportional), opacity);
        painter.text(position2, egui::Align2::CENTER_TOP, "62%", egui::FontId::new(24.0, egui::FontFamily::Proportional), opacity);
        painter.text(position3, egui::Align2::CENTER_TOP, "CHARGING", egui::FontId::new(20.0, egui::FontFamily::Proportional), opacity);
        painter.text(position4, egui::Align2::CENTER_TOP, "18 min remaining", egui::FontId::new(20.0, egui::FontFamily::Proportional), opacity);
        painter.text(position5, egui::Align2::LEFT_TOP, "22 min/hr", egui::FontId::new(20.0, egui::FontFamily::Proportional), opacity);
        painter.text(position6, egui::Align2::RIGHT_TOP, "232 v", egui::FontId::new(20.0, egui::FontFamily::Proportional), opacity);
    }
}