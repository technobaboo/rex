mod compositor;
mod console;
mod control_panel;
mod env_var;
pub mod instance;
mod log_options;

use confy::ConfyError;
use eframe::{App, Frame};
use egui::{Color32, Context, Style, Visuals};
use instance::MonadoInstance;
use log_options::LoggingEnvVars;
use rustc_hash::FxHashMap;
use std::{
    path::PathBuf,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
};

pub fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.min_window_size = Some(egui::Vec2 { x: 512.0, y: 512.0 });
    eframe::run_native(
        "Malek's Monado Gui Utility",
        native_options,
        Box::new(|cc| Box::new(RexApp::new(cc))),
    );
}
pub struct RexApp {
    pub monado_instance_dir: PathBuf,
    pub logging_env_vars: LoggingEnvVars,
    current_instance: String,
    instances: FxHashMap<String, MonadoInstance>,
    pub console: String,
    pub stdout_sender: Arc<Mutex<SyncSender<String>>>,
    pub stdout_receiver: Receiver<String>,
}
impl RexApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let visuals = Visuals::dark();
        cc.egui_ctx.set_visuals(visuals);

        let monado_instance_dir = dirs::config_dir().unwrap().join("monado").join("instances");
        std::fs::create_dir_all(&monado_instance_dir).unwrap();
        let (stdout_sender, stdout_receiver) = sync_channel(64000);
        RexApp {
            monado_instance_dir,
            logging_env_vars: confy::load("monado", "logging").unwrap(),
            console: String::default(),
            stdout_sender: Arc::new(Mutex::new(stdout_sender)),
            stdout_receiver,
            current_instance: String::default(),
            instances: FxHashMap::default(),
        }
    }
    pub fn save_global(&self) -> Result<(), ConfyError> {
        confy::store("monado", "logging", &self.logging_env_vars)
    }
    pub fn current_instance(&mut self) -> Option<&mut MonadoInstance> {
        self.instances.get_mut(&self.current_instance)
    }
}
impl App for RexApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        control_panel::update(self, ctx);
        log_options::update(self, ctx);

        if let Some(instance) = self.current_instance() {
            instance.update(ctx);
        }

        let style = Style {
            visuals: Visuals {
                panel_fill: Color32::TRANSPARENT,
                ..Default::default()
            },
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&style))
            .show(ctx, |ui| {
                console::update(self, ui);
            });
    }
}
