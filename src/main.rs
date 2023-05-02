mod compositor;
mod console;
mod control_panel;
mod env_var;
pub mod instance;
mod log_options;


use eframe::{App, Frame};
use egui::{Color32, Context, Style, Visuals};
use instance::MonadoInstance;
use log_options::LoggingEnvVars;
use native_dialog::MessageDialog;
use rustc_hash::FxHashMap;
use std::{
    iter::FromIterator,
    path::PathBuf,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    error::Error,
};
use expect_dialog::ExpectDialog;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.min_window_size = Some(egui::Vec2 { x: 512.0, y: 512.0 });
    eframe::run_native(
        "Malek's Monado Gui Utility",
        native_options,
        Box::new(|cc| Box::new(RexApp::new(cc))),
    );
    Ok(())
}
pub struct RexApp {
    pub monado_instance_dir: PathBuf,
    pub logging_env_vars: LoggingEnvVars,
    current_instance: Option<String>,
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

        let monado_instance_dir = dirs::config_dir().expect_dialog("System does not have a configured config directory.").join("monado").join("instances");
        std::fs::create_dir_all(&monado_instance_dir).expect_dialog("Unable to create config directory folders.");
        let (stdout_sender, stdout_receiver) = sync_channel(64000);

        let log_env_vars: LoggingEnvVars = RexApp::load_tolerant("monado", "logging");

        let mut app = RexApp {
            monado_instance_dir,
            logging_env_vars: log_env_vars,
            console: String::default(),
            stdout_sender: Arc::new(Mutex::new(stdout_sender)),
            stdout_receiver,
            current_instance: None,
            instances: FxHashMap::default(),
        };
        let _ = app.load_instances();
        app
    }

    pub fn load_instances(&mut self) -> Result<(), Box<dyn Error>> {
        self.instances = FxHashMap::from_iter(
            std::fs::read_dir(&self.monado_instance_dir)?
                .filter_map(|d| Some(d.ok()?.file_name().to_str()?.to_string()))
                .filter_map(|n| Some((n.clone(), MonadoInstance::create_load(self, n).ok()?))),
        );   

        Ok(())
    }

    pub fn save_global(&self) {
        match confy::store("monado", "logging", self.logging_env_vars) {
            Err(err) => {
                println!("Error saving global logging config: {}", err);

                MessageDialog::new()
                    .set_title("Config Error")
                    .set_text(&format!("Error saving global logging config.\nError:\n{}", err))
                    .set_type(native_dialog::MessageType::Error)
                    .show_confirm()
                    .expect("Error creating dialog window for logging options error");
            }   

            _ => {}
        }
    }

    pub fn current_instance(&mut self) -> Option<&mut MonadoInstance> {
        self.instances.get_mut(self.current_instance.as_ref()?)
    }

    fn load_tolerant<'a, T>(app_name: &str, config_name: impl Into<Option<&'a str>>) -> T where T: serde::Serialize + serde::de::DeserializeOwned + Default {
        let config: T;

        let config_name_owned = config_name.into().unwrap_or("None");

        match confy::load(app_name, config_name_owned) {
            Ok(load_config) => config = load_config,
            Err(err) => {
                println!("Error loading config: {}", err);

                let regen_saved_config = MessageDialog::new()
                    .set_title("Config Error")
                    .set_text(&format!("Error loading config '{}', continuing with default options for config. \nWould you like to regenerate the config file? \n\nError: \n{}", config_name_owned, err))
                    .set_type(native_dialog::MessageType::Error)
                    .show_confirm()
                    .expect("Error creating dialog window for logging options error");
                
                config = T::default();

                if regen_saved_config && confy::store(app_name, config_name_owned, &config).is_err() {
                    MessageDialog::new()
                        .set_title("Config Error")
                        .set_text(&format!("Error saving config, is monado-gui missing necessary file permissions? \n Error: \n {}", err))
                        .set_type(native_dialog::MessageType::Error)
                        .show_confirm()
                        .expect("Error creating dialog window for logging options error");
                }
            }   
        }

        config
    }

}
impl App for RexApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
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
                console::update(self, ui, frame);
            });
    }
}
