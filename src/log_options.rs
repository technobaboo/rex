use egui::Ui;
use crate::MonadoGuiApp;
use crate::traits::UiSect;

pub struct LogOptions { }
impl UiSect for LogOptions {
    fn update(state: &mut MonadoGuiApp, ui: &mut Ui) {
        ui.collapsing("Logging Options", |ui| {
           CompositorLog::update(state, ui);
        });
    }
}

pub struct CompositorLog { }
impl UiSect for CompositorLog {
    fn update(state: &mut MonadoGuiApp, ui: &mut Ui) {
        ui.collapsing("Compositor Log", |ui| {
            ui.radio_value(&mut state.env_vars.compositor_log_level,LoggingLevel::Info, "Info");
            ui.radio_value(&mut state.env_vars.compositor_log_level,LoggingLevel::Debug, "Debug");
            ui.radio_value(&mut state.env_vars.compositor_log_level,LoggingLevel::Trace, "Trace");
            ui.radio_value(&mut state.env_vars.compositor_log_level,LoggingLevel::Warn, "Warn");
            ui.radio_value(&mut state.env_vars.compositor_log_level,LoggingLevel::Error, "Error");
        });
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
impl ToString for LoggingLevel {
    fn to_string(&self) -> String {
        match self {
            LoggingLevel::Trace => {String::from("trace")}
            LoggingLevel::Debug => {String::from("debug")}
            LoggingLevel::Info => {String::from("info")}
            LoggingLevel::Warn => {String::from("warn")}
            LoggingLevel::Error => {String::from("error")}
        }
    }
}