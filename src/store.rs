use macroquad::{
    math::vec2,
    ui::{hash, root_ui, widgets},
};

use crate::orb::OrbInfo;

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
}

impl Store {
    pub fn new() -> Self {
        Self {
            close_window: false,
            minimize: false,
            selected_orb: None,
        }
    }

    pub fn draw(&mut self, orbs: (&OrbInfo, &OrbInfo, &OrbInfo)) {
        if self.close_window {
            return;
        }
        // fix damn scroll bar and add orb highlight
        root_ui().window(
            hash!(),
            vec2(0., 0.),
            vec2(220., if self.minimize { 50. } else { 270. }),
            |ui| {
                // HEADER
                ui.label(None, "THREE BODIES");
                widgets::Group::new(hash!(), vec2(90., 25.))
                    .position(vec2(130., 2.))
                    .ui(ui, |ui| {
                        if ui.button(None, if self.minimize { "   +   " } else { "   -   " }) {
                            self.minimize = !self.minimize
                        }

                        if ui.button(vec2(60., 2.), " X ") {
                            self.close_window = true;
                        }
                    });

                if self.minimize {
                    return;
                }

                ui.label(None, "Select Orb To View");
                widgets::Group::new(hash!(), vec2(214., 25.))
                    .position(vec2(2., 45.))
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
                    .position(vec2(2., 75.))
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
            },
        );
    }
}
