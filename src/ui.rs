use crate::{
    constants::PHI,
    flower::{ResetFlowerSeeds, SeedSettings},
    petal::ResetFlowerPetals,
    Callback,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContexts, EguiSettings,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_systems(Update, settings_ui);
    }
}

#[derive(Resource)]
pub struct UiState {
    pub fraction_content: String,
    pub step_size: f32,
    pub animate: bool,
    pub flower_mode: FlowerMode,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FlowerMode {
    Seed,
    Petal,
}

impl FlowerMode {
    fn max_density(self) -> f32 {
        match self {
            FlowerMode::Seed => 30.0,
            FlowerMode::Petal => 100.0,
        }
    }

    fn max_amount(self) -> i32 {
        match self {
            FlowerMode::Seed => 500,
            FlowerMode::Petal => 40,
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            fraction_content: String::from(""),
            step_size: 0.0025,
            animate: false,
            flower_mode: FlowerMode::Seed,
        }
    }
}

pub fn settings_ui(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut seed_settings: ResMut<SeedSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    reset_seeds: Query<&Callback, With<ResetFlowerSeeds>>,
    reset_petals: Query<&Callback, With<ResetFlowerPetals>>,
) {
    let mut changed = false;
    let mode = ui_state.flower_mode;
    egui::SidePanel::left("settings_ui")
        // .default_width(250.0)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                ui.heading("Settings");
                // egui::ComboBox::from_label("Flower mode")
                //     .selected_text(format!("{mode:?}"))
                //     .show_ui(ui, |ui| {
                //         ui.style_mut().wrap = Some(false);
                //         ui.set_min_width(60.0);
                //         ui.selectable_value(&mut ui_state.flower_mode, FlowerMode::Seed, "Seed");
                //         ui.selectable_value(&mut ui_state.flower_mode, FlowerMode::Petal, "Petal");
                //     });
                ui.label("Rotation");
                if ui
                    .add(
                        egui::DragValue::new(&mut seed_settings.rotation)
                            .speed(ui_state.step_size)
                            .fixed_decimals(14),
                    )
                    .changed()
                {
                    changed = true;
                }
                ui.label("Input");
                let fr = ui.text_edit_singleline(&mut ui_state.fraction_content);
                if fr.lost_focus() && fr.ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Ok(value) = exmex::eval_str::<f32>(&ui_state.fraction_content) {
                        changed = true;
                        seed_settings.rotation = value;
                    }
                }
                ui.horizontal(|ui| {
                    if ui.button("φ").clicked() {
                        changed = true;
                        seed_settings.rotation = PHI.fract();
                    }
                    if ui.button("π").clicked() {
                        changed = true;
                        seed_settings.rotation = std::f32::consts::PI.fract();
                    }
                    if ui.button("e").clicked() {
                        changed = true;
                        seed_settings.rotation = std::f32::consts::E.fract();
                    }
                });
                if mode == FlowerMode::Seed {
                    ui.label("Animation");
                    ui.add(
                        egui::Slider::new(&mut ui_state.step_size, 0.0..=0.1)
                            .drag_value_speed(0.001)
                            .logarithmic(true)
                            .text("Step size"),
                    )
                    .changed();
                    let button_text = if ui_state.animate {
                        "Stop animation"
                    } else {
                        "Start animation"
                    };
                    if ui.button(button_text).clicked() {
                        ui_state.animate = !ui_state.animate;
                    }
                }
                // ui.label("Misc Settings");
                // if ui
                //     .add(
                //         egui::Slider::new(&mut seed_settings.distance, 0.0..=mode.max_density())
                //             .text("Density"),
                //     )
                //     .changed()
                // {
                //     changed = true
                // }
                // if ui
                //     .add(
                //         egui::Slider::new(&mut seed_settings.radius, f32::EPSILON..=200.0)
                //             .text("Size"),
                //     )
                //     .changed()
                // {
                //     changed = true;
                // }
                if ui
                    .add(
                        egui::Slider::new(&mut seed_settings.amount, 0..=mode.max_amount())
                            .text("Amount"),
                    )
                    .changed()
                {
                    changed = true
                }
                ui.horizontal(|ui| {
                    if ui.button("-1").clicked() {
                        changed = true;
                        seed_settings.amount = i32::max(seed_settings.amount - 1, 1);
                    }
                    if ui.button("+1").clicked() {
                        changed = true;
                        seed_settings.amount =
                            i32::min(seed_settings.amount + 1, mode.max_amount());
                    }
                });
                if ui.button("Reset").clicked() {
                    match mode {
                        FlowerMode::Seed => {
                            *seed_settings = SeedSettings::default();
                        }
                        FlowerMode::Petal => {
                            *seed_settings = SeedSettings::default_petal();
                        }
                    }
                    changed = true;
                }
            });
        });
    if mode != ui_state.flower_mode {
        ui_state.animate = false;
        match ui_state.flower_mode {
            FlowerMode::Seed => {
                *seed_settings = SeedSettings::default();
            }
            FlowerMode::Petal => {
                *seed_settings = SeedSettings::default_petal();
            }
        }
    }
    if changed || mode != ui_state.flower_mode {
        match ui_state.flower_mode {
            FlowerMode::Seed => {
                seed_settings.mesh_handle = None;
                commands.run_system(reset_seeds.single().0);
            }
            FlowerMode::Petal => {
                seed_settings.mesh_handle = None;
                commands.run_system(reset_petals.single().0);
            }
        }
    }
}
