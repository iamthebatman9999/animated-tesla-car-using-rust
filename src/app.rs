use std::time::{Duration, Instant};

use eframe::egui::{self, CentralPanel};
use egui::{load::SizedTexture, ImageSource, TextureHandle};

use crate::{charge::ChargeState, lock::{LockState, DOOR_SVG_HEIGHT, DOOR_SVG_WIDTH}, temp::{TempState, TempType}, tyre::TyreState, utils::{fit_to_screen_size, load_svg_from_bytes}};

const CAR_SVG: &[u8] = include_bytes!("../assets/icons/Car.svg");
const CAR_WIDTH: u32 = 222;
const CAR_HEIGHT: u32 = 477;
const CAR_SCALE: u32 = 2;

const NAV_LOCK_SVG: ImageSource<'_> = egui::include_image!("../assets/icons/Lock.svg");
const NAV_CHARGE_SVG: ImageSource<'_> = egui::include_image!("../assets/icons/Charge.svg");
const NAV_TEMP_SVG: ImageSource<'_> = egui::include_image!("../assets/icons/Temp.svg");
const NAV_TYRE_SVG: ImageSource<'_> = egui::include_image!("../assets/icons/Tyre.svg");

const NAV_SIZE: f32 = 40.0;

#[derive(PartialEq)]
enum Nav {
    Lock,
    Charge,
    Temp,
    Tyre,
}

pub struct MyApp {
    car_texture: Option<TextureHandle>,
    nav: Nav,
    left_lock: LockState,
    right_lock: LockState,
    top_lock: LockState,
    bottom_lock: LockState,
    charge: ChargeState,
    temp: TempState,
    tyre: TyreState,
    animation_switch_duration: Duration,
    animation_bounce_duration: Duration,
    animation_charge_duration: Duration,
    animation_temp_duration: Duration,
    animation_temp_icon_duration: Duration,
    animation_temp_glow_duration: Duration,
    animation_temp_fade_duration: Duration,
    animation_tyre_duration: Duration,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut car_texture = None;

        let ctx = &cc.egui_ctx;

        if let Some(ctx) = ctx.clone().into() {
            if let Ok(image) = load_svg_from_bytes(CAR_SVG, CAR_WIDTH * CAR_SCALE, CAR_HEIGHT * CAR_SCALE) {
                let texture = ctx.load_texture("car_svg", image, egui::TextureOptions::default());
                car_texture = Some(texture);
            }
        }

        Self {
            car_texture,
            nav: Nav::Lock,
            left_lock: LockState::new(ctx.clone()),
            right_lock: LockState::new(ctx.clone()),
            top_lock: LockState::new(ctx.clone()),
            bottom_lock: LockState::new(ctx.clone()),
            charge: ChargeState::default(),
            temp: TempState::new(),
            tyre: TyreState::default(),
            animation_switch_duration: Duration::from_millis(300),
            animation_bounce_duration: Duration::from_millis(500),
            animation_charge_duration: Duration::from_millis(700),
            animation_temp_duration: Duration::from_millis(300),
            animation_temp_icon_duration: Duration::from_millis(200),
            animation_temp_glow_duration: Duration::from_millis(300),
            animation_temp_fade_duration: Duration::from_millis(300),
            animation_tyre_duration: Duration::from_millis(300),
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Animate all locks
        let animation_duration = self.animation_switch_duration;
        self.left_lock.animate_switch(animation_duration);
        self.right_lock.animate_switch(animation_duration);
        self.top_lock.animate_switch(animation_duration);
        self.bottom_lock.animate_switch(animation_duration);

        let animation_duration = self.animation_bounce_duration;
        self.left_lock.animate_bounce(animation_duration);
        self.right_lock.animate_bounce(animation_duration);
        self.top_lock.animate_bounce(animation_duration);
        self.bottom_lock.animate_bounce(animation_duration);

        let animation_duration = self.animation_charge_duration;
        self.charge.animate_charge(animation_duration);

        let animation_duration = self.animation_temp_duration;
        self.temp.animate_temp(animation_duration);
        let animation_duration = self.animation_temp_icon_duration;
        self.temp.animate_temp_icon(animation_duration);
        let animation_duration = self.animation_temp_glow_duration;
        self.temp.animate_temp_glow(animation_duration);
        let animation_duration = self.animation_temp_fade_duration;
        self.temp.animate_temp_fade(animation_duration);

        let animation_duration = self.animation_tyre_duration;
        self.tyre.animate_tyre_left_up(animation_duration);
        self.tyre.animate_tyre_right_up(animation_duration);
        self.tyre.animate_tyre_left_down(animation_duration);
        self.tyre.animate_tyre_right_down(animation_duration);

        // Request repaint if any animation is active
        if self.left_lock.animation_switch.is_some()
            || self.right_lock.animation_switch.is_some()
            || self.top_lock.animation_switch.is_some()
            || self.bottom_lock.animation_switch.is_some()
            || self.left_lock.animation_bounce.is_some()
            || self.right_lock.animation_bounce.is_some()
            || self.top_lock.animation_bounce.is_some()
            || self.bottom_lock.animation_bounce.is_some()
            || self.charge.animation_charge.is_some()
            || self.temp.animation_temp.is_some()
            || self.temp.animation_temp_icon.is_some()
            || self.temp.animation_temp_glow.is_some()
            || self.temp.animation_temp_fade.is_some()
            || self.tyre.animation_tyre_left_up.is_some()
            || self.tyre.animation_tyre_right_up.is_some()
            || self.tyre.animation_tyre_left_down.is_some()
            || self.tyre.animation_tyre_right_down.is_some()
        {
            ctx.request_repaint();
        }

        // let frame = egui::Frame {
        //     outer_margin: egui::Margin {left: 0, right: 0, top: 0, bottom: 0},
        //     inner_margin: egui::Margin {left: 0, right: 0, top: 0, bottom: 0},
        //     stroke: egui::Stroke::new(1.0, egui::Color32::GREEN),
        //     ..egui::Frame::NONE
        // };
        // egui::TopBottomPanel::top("top").show_separator_line(false).frame(frame).show(ctx, |_ui| {

        // });
        
        let frame = egui::Frame {
            outer_margin: egui::Margin {left: 0, right: 0, top: 0, bottom: 50},
            // stroke: egui::Stroke::new(1.0, egui::Color32::RED),
            ..egui::Frame::NONE
        };
        egui::TopBottomPanel::bottom("bottom").show_separator_line(false).frame(frame).show(ctx, |ui| {
            egui_flex::Flex::horizontal()
            .grow_items(1.0)
            .w_full()
            .wrap(true)
            .show(ui, |flex| {
                let tint_lock = if self.nav == Nav::Lock { egui::Color32::from_rgba_premultiplied(83, 249, 255, 255) } else { egui::Color32::from_rgba_premultiplied(132, 132, 132, 138) };
                let tint_charge = if self.nav == Nav::Charge { egui::Color32::from_rgba_premultiplied(83, 249, 255, 255) } else { egui::Color32::from_rgba_premultiplied(132, 132, 132, 138) };
                let tint_temp = if self.nav == Nav::Temp { egui::Color32::from_rgba_premultiplied(83, 249, 255, 255) } else { egui::Color32::from_rgba_premultiplied(132, 132, 132, 138) };
                let tint_tyre = if self.nav == Nav::Tyre { egui::Color32::from_rgba_premultiplied(83, 249, 255, 255) } else { egui::Color32::from_rgba_premultiplied(132, 132, 132, 138) };

                let res_lock = flex.add(
                    egui_flex::FlexItem::new(),
                    egui::Image::new(NAV_LOCK_SVG).sense(egui::Sense::CLICK).fit_to_exact_size(egui::vec2(NAV_SIZE, NAV_SIZE)).tint(tint_lock)
                );
                let res_charge = flex.add(
                    egui_flex::FlexItem::new(),
                    egui::Image::new(NAV_CHARGE_SVG).sense(egui::Sense::CLICK).fit_to_exact_size(egui::vec2(NAV_SIZE, NAV_SIZE)).tint(tint_charge)
                );
                let res_temp = flex.add(
                    egui_flex::FlexItem::new(),
                    egui::Image::new(NAV_TEMP_SVG).sense(egui::Sense::CLICK).fit_to_exact_size(egui::vec2(NAV_SIZE, NAV_SIZE)).tint(tint_temp)
                );
                let res_tyre = flex.add(
                    egui_flex::FlexItem::new(),
                    egui::Image::new(NAV_TYRE_SVG).sense(egui::Sense::CLICK).fit_to_exact_size(egui::vec2(NAV_SIZE, NAV_SIZE)).tint(tint_tyre)
                );

                if res_lock.clicked() {
                    let start = Instant::now();
                    // old status
                    match self.nav {
                        Nav::Charge => {
                            self.charge.is_charged = false;
                            self.charge.animation_charge = Some(start);
                        },
                        Nav::Temp => {
                            self.temp.is_temp = false;
                            self.temp.animation_temp = Some(start);
                        },
                        Nav::Tyre => {
                            self.tyre.is_tyre = false;
                            self.tyre.animation_tyre_left_up = Some(start);
                        },
                        _ => ()
                    }
                    self.nav = Nav::Lock;
                    for lock in vec![
                            &mut self.left_lock,
                            &mut self.right_lock,
                            &mut self.top_lock,
                            &mut self.bottom_lock] {
                        lock.animation_switch = None;
                        lock.animation_bounce = Some(start);
                        lock.is_show = true;
                    }
                }
                if res_charge.clicked() {
                    let start = Instant::now();
                    match self.nav {
                        Nav::Lock => {
                            for lock in vec![
                                    &mut self.left_lock,
                                    &mut self.right_lock,
                                    &mut self.top_lock,
                                    &mut self.bottom_lock] {
                                lock.animation_switch = None;
                                lock.animation_bounce = Some(start);
                                lock.is_show = false;
                            }
                        },
                        Nav::Temp => {
                            self.temp.is_temp = false;
                            self.temp.animation_temp = Some(start);
                        },
                        Nav::Tyre => {
                            self.tyre.is_tyre = false;
                            self.tyre.animation_tyre_left_up = Some(start);
                        },
                        _ => ()
                    }
                    self.nav = Nav::Charge;
                    self.charge.is_charged = true;
                    self.charge.animation_charge = Some(start);
                }
                if res_temp.clicked() {
                    let start = Instant::now();
                    match self.nav {
                        Nav::Lock => {
                            for lock in vec![
                                    &mut self.left_lock,
                                    &mut self.right_lock,
                                    &mut self.top_lock,
                                    &mut self.bottom_lock] {
                                lock.animation_switch = None;
                                lock.animation_bounce = Some(start);
                                lock.is_show = false;
                            }
                        },
                        Nav::Charge => {
                            self.charge.is_charged = false;
                            self.charge.animation_charge = Some(start);
                        },
                        Nav::Tyre => {
                            self.tyre.is_tyre = false;
                            self.tyre.animation_tyre_left_up = Some(start);
                        },
                        _ => (),
                    }
                    self.temp.is_temp = true;
                    self.temp.animation_temp = Some(start);
                    self.temp.animation_temp_icon = Some(start);
                    self.nav = Nav::Temp;
                }
                if res_tyre.clicked() {
                    let start = Instant::now();
                    match self.nav {
                        Nav::Lock => {
                            for lock in vec![
                                    &mut self.left_lock,
                                    &mut self.right_lock,
                                    &mut self.top_lock,
                                    &mut self.bottom_lock] {
                                lock.animation_switch = None;
                                lock.animation_bounce = Some(start);
                                lock.is_show = false;
                            }
                        },
                        Nav::Charge => {
                            self.charge.is_charged = false;
                            self.charge.animation_charge = Some(start);
                        },
                        Nav::Temp => {
                            self.temp.is_temp = false;
                            self.temp.animation_temp = Some(start);
                        },
                        _ => ()
                    }
                    self.tyre.is_tyre = true;
                    self.nav = Nav::Tyre;
                    self.tyre.animation_tyre_left_up = Some(start + self.animation_temp_duration);
                    self.tyre.animation_tyre_right_up = None;
                    self.tyre.animation_tyre_left_down = None;
                    self.tyre.animation_tyre_right_down = None;
                }
                
            });
        });

        let frame = egui::Frame {
            // outer_margin: egui::Margin {left: 0, right: 0, top: 0, bottom: 0},
            inner_margin: egui::Margin {left: 0, right: 0, top: 0, bottom: 0},
            // stroke: egui::Stroke { width: 1.0, color: egui::Color32::YELLOW },
            ..egui::Frame::NONE
        };
        CentralPanel::default().frame(frame).show(ctx, |ui| {
            let available_size = ui.available_size();

            // Save center position
            let center = egui::pos2(available_size.x / 2.0, available_size.y / 2.0);

            let padding_left = 10.0;
            let padding_right = 10.0;
            let padding_top = 80.0;
            let padding_bottom = 80.0;

            let left_pos = egui::Pos2 {x: padding_left + (DOOR_SVG_WIDTH as f32/ 2.0), y: center.y};
            let right_pos = egui::Pos2 {x: available_size.x - padding_right - (DOOR_SVG_WIDTH as f32 / 2.0), y: center.y};
            let top_pos = egui::Pos2 {x: center.x, y: padding_top + (DOOR_SVG_HEIGHT as f32 / 2.0)};
            let bottom_pos = egui::Pos2 {x: center.x, y: available_size.y - padding_bottom - (DOOR_SVG_HEIGHT as f32 / 2.0)};

            if let Some(tex) = &self.car_texture {
                let size = fit_to_screen_size(tex.size_vec2(), available_size);
                let new_size = size * 0.9;
                let img_src = ImageSource::Texture(SizedTexture {id: tex.id(), size: new_size});
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let start_x = (available_size.x - new_size.x) / 2.0;
                    let end_x = available_size.x - (new_size.x / 2.0);
                    let mut space = start_x;
                    if self.temp.is_temp {
                        if self.temp.animation_temp.is_none() {
                            self.temp.progress_temp = 1.0;
                        }
                        space = start_x + (end_x - start_x) * self.temp.progress_temp;
                    } else {
                        if self.temp.animation_temp.is_some() {
                            space = start_x + (end_x - start_x) * (1.0 - self.temp.progress_temp);
                        }
                    }

                    ui.add_space(space);
                    let mut rect_right = ui.image(img_src).rect;
                    self.tyre.rect_car = Some(rect_right);
                    if self.temp.animation_temp.is_none() {
                        if self.nav == Nav::Tyre {
                            self.tyre.is_tyre = true;
                        }
                    }
                    rect_right.min.y -= 10.0;
                    rect_right.max.y += 10.0;
                    rect_right.max.x = rect_right.min.x + rect_right.size().x / 2.0;
                    self.temp.rect_right = Some(rect_right);

                    // debugging purposes
                    // ui.painter().line_segment([rect_right.min, rect_right.max], Stroke::new(2.0, egui::Color32::BLUE));
                });
            }

            // Draw locks on top of the car
            let left_response = self.left_lock.draw(ui, left_pos, center);
            let right_response = self.right_lock.draw(ui, right_pos, center);
            let top_response = self.top_lock.draw(ui, top_pos, center);
            let bottom_response = self.bottom_lock.draw(ui, bottom_pos, center);

            if self.charge.is_charged {
                let charge_rect = egui::Rect {
                    min: egui::Pos2::new(0.0, 0.0),
                    max: egui::Pos2::new(available_size.x, available_size.y)
                };
                self.charge.draw(ui, charge_rect);
            } else {
                if self.charge.animation_charge.is_some() {
                    let charge_rect = egui::Rect {
                        min: egui::Pos2::new(0.0, 0.0),
                        max: egui::Pos2::new(available_size.x, available_size.y)
                    };
                    self.charge.draw(ui, charge_rect);
                }
            }
            
            // Handle clicks
            if left_response.clicked() {
                self.left_lock.is_locked = !self.left_lock.is_locked;
                self.left_lock.animation_switch = Some(Instant::now());
            }
            if right_response.clicked() {
                self.right_lock.is_locked = !self.right_lock.is_locked;
                self.right_lock.animation_switch = Some(Instant::now());
            }
            if top_response.clicked() {
                self.top_lock.is_locked = !self.top_lock.is_locked;
                self.top_lock.animation_switch = Some(Instant::now());
            }
            if bottom_response.clicked() {
                self.bottom_lock.is_locked = !self.bottom_lock.is_locked;
                self.bottom_lock.animation_switch = Some(Instant::now());
            }

            let res = self.temp.draw(ui, egui::Rect::from_center_size(center, available_size));
            if res[0].clicked() || res[1].clicked() {
                if self.temp.temp_type == Some(TempType::Cool) {
                    self.temp.temp_type = Some(TempType::Heat);
                } else {
                    self.temp.temp_type = Some(TempType::Cool);
                }
                self.temp.animation_temp_icon = Some(Instant::now());
            }
            if res[2].clicked() {
                self.temp.temp += 1;
            }
            if res[3].clicked() {
                self.temp.temp -= 1;
            }

            // Handle tyre
            self.tyre.draw(ui, egui::Rect::from_center_size(center, available_size), self.tyre.rect_car.unwrap());

            // debugging purposes
            // ui.painter().line_segment([egui::Pos2::new(0.0, 0.0), egui::Pos2::new(available_size.x, available_size.y)], egui::Stroke::new(1.0, egui::Color32::WHITE));
            // ui.painter().line_segment([egui::Pos2::new(0.0, available_size.y), egui::Pos2::new(available_size.x, 0.0)], egui::Stroke::new(1.0, egui::Color32::WHITE));
            // ui.painter().line_segment([egui::Pos2::new(0.0, center.y), egui::Pos2::new(available_size.x, center.y)], egui::Stroke::new(1.0, egui::Color32::WHITE));
            // ui.painter().line_segment([egui::Pos2::new(center.x, 0.0), egui::Pos2::new(center.x, available_size.y)], egui::Stroke::new(1.0, egui::Color32::WHITE));

        });
    }
}

