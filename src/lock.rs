use std::time::{Duration, Instant};
use egui::{Context, Pos2, TextureHandle, TextureOptions};
use crate::utils::{ease_in_out_back, load_svg_from_bytes};

pub const UV: egui::Rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
const DOOR_LOCK_SVG: &[u8] = include_bytes!("../assets/icons/door_lock.svg");
const DOOR_UNLOCK_SVG: &[u8] = include_bytes!("../assets/icons/door_unlock.svg");
pub const DOOR_SVG_WIDTH: u32 = 50;
pub const DOOR_SVG_HEIGHT: u32 = 50;

#[derive(Default)]
pub struct LockState {
    pub is_locked: bool,
    pub is_show: bool,
    // rect: Option<egui::Rect>,
    scale_up: f32,
    scale_down: f32,
    progress_switch: f32,
    pub animation_switch: Option<Instant>,
    progress_bounce: f32,
    pub animation_bounce: Option<Instant>,
    transition_alpha: f32,
    lock_texture: Option<TextureHandle>,
    unlock_texture: Option<TextureHandle>,
}

impl LockState {
    pub fn new(ctx: Context) -> Self {
        // lock
        let lock_image = load_svg_from_bytes(DOOR_LOCK_SVG, DOOR_SVG_WIDTH, DOOR_SVG_HEIGHT).unwrap();
        let tex_lock = ctx.load_texture("lock", lock_image, TextureOptions::default());
        // unlock
        let unlock_image = load_svg_from_bytes(DOOR_UNLOCK_SVG, DOOR_SVG_WIDTH, DOOR_SVG_HEIGHT).unwrap();
        let tex_unlock = ctx.load_texture("unlock", unlock_image, TextureOptions::default());

        // construct
        LockState {
            is_locked: true,
            is_show: true,
            progress_switch: 0.0,
            scale_up: 0.0,
            scale_down: 1.0,
            transition_alpha: 1.0,
            lock_texture: Some(tex_lock),
            unlock_texture: Some(tex_unlock),
            ..Default::default()
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, src_pos: Pos2, dest_pos: Pos2) -> egui::Response {
        let painter = ui.painter();

        let src_rect = egui::Rect::from_center_size(src_pos, egui::Vec2::new(DOOR_SVG_WIDTH as f32, DOOR_SVG_HEIGHT as f32));
        let dest_rect = egui::Rect::from_center_size(dest_pos, egui::Vec2::new(DOOR_SVG_WIDTH as f32, DOOR_SVG_HEIGHT as f32));
        let mut rect = src_rect;
        let mut pos = rect.center();
        let mut opacity = egui::Color32::WHITE;

        // status show/hide
        if self.animation_bounce.is_some() {
            opacity = egui::Color32::from_white_alpha((self.transition_alpha * 255.0) as u8);
            if self.is_show {
                let start_x = dest_rect.center().x;
                let start_y = dest_rect.center().y;
                let end_x = src_rect.center().x;
                let end_y = src_rect.center().y;
                let x = start_x + (end_x - start_x) * self.progress_bounce;
                let y = start_y + (end_y - start_y) * self.progress_bounce;
                pos = Pos2::new(x, y);
                rect = egui::Rect::from_center_size(pos, src_rect.size());
            } else {
                let start_x = src_rect.center().x;
                let start_y = src_rect.center().y;
                let end_x = dest_rect.center().x;
                let end_y = dest_rect.center().y;
                let x = start_x + (end_x - start_x) * self.progress_bounce;
                let y = start_y + (end_y - start_y) * self.progress_bounce;
                pos = Pos2::new(x, y);
                rect = egui::Rect::from_center_size(pos, src_rect.size());
            }
        }

        // status lock/unlock
        if self.animation_switch.is_none() {
            if self.animation_bounce.is_none() {
                if !self.is_show {
                    opacity = egui::Color32::from_white_alpha((self.transition_alpha * 255.0) as u8);
                }
            }
            if self.is_locked {
                if let Some(tex) = &self.lock_texture {
                    painter.image(tex.id(), rect, UV, opacity);
                }
            } else {
                if let Some(tex) = &self.unlock_texture {
                    painter.image(tex.id(), rect, UV, opacity);
                }
            }
        } else {
            let size = rect.size();
            let size_reduced = size * self.scale_down;
            let rect_reduced = egui::Rect::from_center_size(pos, size_reduced);
            let size_expanded = size * self.scale_up;
            let rect_expanded = egui::Rect::from_center_size(pos, size_expanded);
            
            if self.is_locked {
                if let Some(tex) = &self.unlock_texture {
                    painter.image(tex.id(), rect_reduced, UV, egui::Color32::WHITE);
                }
                if let Some(tex) = &self.lock_texture {
                    painter.image(tex.id(), rect_expanded, UV, egui::Color32::WHITE);
                }
            } else {
                if let Some(tex) = &self.lock_texture {
                    painter.image(tex.id(), rect_reduced, UV, egui::Color32::WHITE);
                }
                if let Some(tex) = &self.unlock_texture {
                    painter.image(tex.id(), rect_expanded, UV, egui::Color32::WHITE);
                }
            }
        }

        ui.allocate_rect(rect, egui::Sense::click())

    }

    pub fn set_progress_switch(&mut self, progress: f32) {
        self.progress_switch = progress;
        self.scale_up = ease_in_out_back(self.progress_switch);
        if self.scale_up < 0.0 {
            self.scale_up = 0.0;
        }
        self.scale_down = 1.0 - self.progress_switch;
    }
    
    pub fn animate_switch(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_switch {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_switch = None;
            } else {
                let progress = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.set_progress_switch(progress);
            }
        }
    }

    pub fn animate_bounce(&mut self, animation_duration: Duration) {
        if let Some(start_time) = self.animation_bounce {
            let elapsed = start_time.elapsed();

            if elapsed >= animation_duration {
                self.animation_bounce = None;
                self.transition_alpha = if self.is_show { 1.0 } else { 0.0 };
                self.progress_bounce = 1.0;
            } else {
                self.progress_bounce = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
                self.transition_alpha = if self.is_show {
                    self.progress_bounce
                } else {
                    1.0 - self.progress_bounce
                };
            }
        }
    }
}