use crate::RexApp;
use ansi_parser::{AnsiParser, Output};
use egui::panel::TopBottomSide;
use egui::{Color32, ComboBox, Context, Layout, Ui, Widget};

pub fn update(state: &mut RexApp, ctx: &Context) {
    egui::TopBottomPanel::new(TopBottomSide::Bottom, "control_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            monado_control_buttons(state, ui);
            instance_selector(state, ui);
            log_buttons(state, ui);
        });
    });
}

fn monado_control_buttons(state: &mut RexApp, ui: &mut Ui) {
    let current_instance = state.current_instance();
    let instance_active = current_instance.is_some();
    let monado_active = instance_active && current_instance.as_ref().unwrap().child.is_some();
    let start_button = ui.add_enabled_ui(instance_active, |ui| {
        if !monado_active {
            egui::Button::new(" Start ").fill(Color32::from_rgb(0, 40, 0))
        } else {
            egui::Button::new("Restart").fill(Color32::from_rgb(20, 20, 0))
        }
        .ui(ui)
    });
    let stop_button = ui.add_enabled_ui(monado_active, |ui| {
        egui::Button::new("Stop")
            .fill(Color32::from_rgb(40, 0, 0))
            .ui(ui)
    });

    if state.current_instance().is_none() {
        return;
    };
    if start_button.inner.clicked() {
        let logging_env_vars = state.logging_env_vars;
        let stdout_sender = state.stdout_sender.clone();
        state.console.clear();
        let Some(instance) = state.current_instance() else {return};
        let _ = instance.kill_monado();
        instance.start_monado(&logging_env_vars, stdout_sender);
    }
    if stop_button.inner.clicked() {
        let Some(instance) = state.current_instance() else {return};
        let _ = instance.kill_monado();
    }
}

fn instance_selector(state: &mut RexApp, ui: &mut Ui) {
    ComboBox::from_id_source(0)
        .selected_text(
            state
                .current_instance
                .as_ref()
                .map(String::as_str)
                .unwrap_or("Select Instance"),
        )
        .show_ui(ui, |ui| {
            for name in state.instances.keys() {
                ui.selectable_value(&mut state.current_instance, Some(name.clone()), name);
            }
        });
}

fn log_buttons(state: &mut RexApp, ui: &mut Ui) {
    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
        if ui.button("Clear").clicked() {
            state.console.clear();
        }
        if ui.button("Copy To Clipboard").clicked() {
            let mut output_string = String::new();
            state.console.ansi_parse().for_each(|ansi| match ansi {
                Output::TextBlock(block) => {
                    output_string.push_str(block);
                }
                Output::Escape(_) => {}
            });
            ui.output().copied_text = output_string;
        };
        ui.label("Log:");
    });
}
