use macroquad::{
    input::mouse_position,
    math::{vec2, Vec2},
    ui::{hash, root_ui, widgets},
};

use crate::orb::OrbInfo;

const UI_FULL_WIDTH: f32 = 220.;

#[derive(PartialEq, Eq)]
pub(crate) enum OrbOption {
    Orb1,
    Orb2,
    Orb3,
}
impl std::fmt::Display for OrbOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrbOption::Orb1 => write!(f, "Orb1"),
            OrbOption::Orb2 => write!(f, "Orb2"),
            OrbOption::Orb3 => write!(f, "Orb3"),
        }
    }
}

pub(crate) struct Store {
    close_window: bool,
    minimize: bool,
    selected_orb: Option<OrbOption>,
    pub fixed_delta_time: f32,
}

impl Store {
    pub fn new() -> Self {
        Self {
            close_window: false,
            minimize: true,
            selected_orb: None,
            fixed_delta_time: 0.015,
        }
    }

    /// returns 0 | 1 | 2 | 3
    pub fn get_selected_orb_number(&self) -> u8 {
        match &self.selected_orb {
            Some(o) => match o {
                OrbOption::Orb1 => 1,
                OrbOption::Orb2 => 2,
                OrbOption::Orb3 => 3,
            },
            None => 0,
        }
    }
    /// return: if mouse_over_ui { true } else { false }
    pub fn draw(&mut self, orbs: (&OrbInfo, &OrbInfo, &OrbInfo)) -> bool {
        if self.close_window {
            return false;
        }

        let window_size = vec2(UI_FULL_WIDTH, if self.minimize { 30. } else { 310. });
        let window_pos = vec2(0., 0.);
        // fix damn scroll bar and add orb highlight
        root_ui().window(hash!(), window_pos, window_size, |ui| {
            // HEADER
            ui.label(None, "THREE BODIES");
            widgets::Group::new(hash!(), vec2(90., 25.))
                .position(vec2(126., 2.))
                .ui(ui, |ui| {
                    if ui.button(None, if self.minimize { "   +   " } else { "   -   " }) {
                        self.minimize = !self.minimize
                    }

                    if ui.button(vec2(62., 2.), " X ") {
                        self.close_window = true;
                    }
                });

            if self.minimize {
                return;
            }

            // Time Delta Slider
            ui.label(None, "Delta Time");
            ui.slider(
                hash!(),
                "[.00001 .. .1]",
                0.00001f32..0.1f32,
                &mut self.fixed_delta_time,
            );

            // Orb Selector
            ui.label(None, "Select Orb To View");
            widgets::Group::new(hash!(), vec2(UI_FULL_WIDTH - 6., 25.))
                .position(vec2(2., 85.))
                .ui(ui, |ui| {
                    if ui.button(vec2(2., 2.), "  Orb 1  ") {
                        self.selected_orb = Some(OrbOption::Orb1);
                    }
                    if ui.button(vec2(72., 2.), "  Orb 2  ") {
                        self.selected_orb = Some(OrbOption::Orb3);
                    }
                    if ui.button(vec2(144., 2.), "  Orb 3  ") {
                        self.selected_orb = Some(OrbOption::Orb2);
                    }
                });

            let Some(selected_orb) = &self.selected_orb else {
                return;
            };

            // Orb Properties
            let orb_info = match selected_orb {
                OrbOption::Orb1 => orbs.0,
                OrbOption::Orb2 => orbs.1,
                OrbOption::Orb3 => orbs.2,
            };

            widgets::Group::new(hash!(), vec2(214., 190.))
                .position(vec2(2., 115.))
                .ui(ui, |ui| {
                    ui.label(None, &format!("Position({}):", selected_orb));
                    ui.label(None, &format!("x: {}", orb_info.pos.x,));
                    ui.label(None, &format!("y: {}", orb_info.pos.y,));
                    ui.label(None, &format!("z: {}", orb_info.pos.z,));

                    ui.label(None, &format!("Velocity({}):", selected_orb));
                    ui.label(None, &format!("x: {}", orb_info.velocity.x,));
                    ui.label(None, &format!("y: {}", orb_info.velocity.y,));
                    ui.label(None, &format!("z: {}", orb_info.velocity.z,));
                });
        });
        self.check_mouse_over_window(window_pos, window_size)
    }

    fn check_mouse_over_window(&self, pos: Vec2, size: Vec2) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        (pos.x..=size.x).contains(&mouse_x) && (pos.y..=size.y).contains(&mouse_y)
    }
}
