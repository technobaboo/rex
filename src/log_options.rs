use crate::RexApp;
use egui::{Color32, ComboBox, Context, RichText, Ui};
use serde::{Deserialize, Serialize};
use std::default::Default;
use subprocess::Exec;

pub fn update(state: &mut RexApp, ctx: &Context) {
    egui::Window::new("Logging Options")
        .default_pos([512.0, 0.0])
        .collapsible(true)
        .show(ctx, |ui| {
            log_level_dropdown(state, ui, "Compositor", |s| {
                &mut s.logging_env_vars.compositor_log
            });
        });
}

fn log_level_dropdown<F: FnOnce(&mut RexApp) -> &mut LoggingLevel>(
    state: &mut RexApp,
    ui: &mut Ui,
    name: &str,
    log_level_fn: F,
) {
    let log_level = log_level_fn(state);
    let old_value = log_level.clone();
    ComboBox::from_label(name)
        .selected_text(format!("{:?}", log_level))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                log_level,
                LoggingLevel::Trace,
                RichText::new("Trace").color(Color32::LIGHT_GRAY),
            );
            ui.selectable_value(
                log_level,
                LoggingLevel::Debug,
                RichText::new("Debug").color(Color32::DEBUG_COLOR),
            );
            ui.selectable_value(
                log_level,
                LoggingLevel::Info,
                RichText::new("Info").color(Color32::LIGHT_BLUE),
            );
            ui.selectable_value(
                log_level,
                LoggingLevel::Warn,
                RichText::new("Warn").color(Color32::YELLOW),
            );
            ui.selectable_value(
                log_level,
                LoggingLevel::Error,
                RichText::new("Error").color(Color32::LIGHT_RED),
            );
        });
    if *log_level != old_value {
        state.save_global();
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Copy, Clone)]
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
            LoggingLevel::Trace => String::from("trace"),
            LoggingLevel::Debug => String::from("debug"),
            LoggingLevel::Info => String::from("info"),
            LoggingLevel::Warn => String::from("warn"),
            LoggingLevel::Error => String::from("error"),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct LoggingEnvVars {
    pub compositor_log: LoggingLevel,
    pub egl_swap_chain_log: LoggingLevel,
    pub d3d_compositor_log: LoggingLevel,
    pub ht_log: LoggingLevel,
    pub calibration_log: LoggingLevel,
    pub global_log: LoggingLevel,
    pub aeg_log: LoggingLevel,
    pub egl_log: LoggingLevel,
    pub mercury_log: LoggingLevel,
    pub slam_log: LoggingLevel,
    pub simple_imu_log: LoggingLevel,
    pub psvr_tracking_log: LoggingLevel,
    pub d3d11_log: LoggingLevel,
    pub u_pacing_app_log: LoggingLevel,
    pub u_pacing_compositor_log: LoggingLevel,
    pub json_log: LoggingLevel,
    pub ahardwarebuffer_log: LoggingLevel,
    pub lh_log: LoggingLevel,
    pub svr_log: LoggingLevel,
    pub ns_log: LoggingLevel,
    pub qwerty_log: LoggingLevel,
    pub arduino_log: LoggingLevel,
    pub hydra_log: LoggingLevel,
    pub survive_log: LoggingLevel,
    pub vive_log: LoggingLevel,
}
impl Default for LoggingEnvVars {
    fn default() -> Self {
        LoggingEnvVars {
            compositor_log: LoggingLevel::Debug,
            egl_swap_chain_log: LoggingLevel::Warn,
            d3d_compositor_log: LoggingLevel::Debug,
            ht_log: LoggingLevel::Warn,
            calibration_log: LoggingLevel::Debug,
            global_log: LoggingLevel::Warn,
            aeg_log: LoggingLevel::Warn,
            egl_log: LoggingLevel::Info,
            mercury_log: LoggingLevel::Warn,
            slam_log: LoggingLevel::Debug,
            simple_imu_log: LoggingLevel::Warn,
            psvr_tracking_log: LoggingLevel::Warn,
            d3d11_log: LoggingLevel::Warn,
            u_pacing_app_log: LoggingLevel::Warn,
            u_pacing_compositor_log: LoggingLevel::Warn,
            json_log: LoggingLevel::Warn,
            ahardwarebuffer_log: LoggingLevel::Warn,
            lh_log: LoggingLevel::Warn,
            svr_log: LoggingLevel::Warn,
            ns_log: LoggingLevel::Warn,
            qwerty_log: LoggingLevel::Info,
            arduino_log: LoggingLevel::Warn,
            hydra_log: LoggingLevel::Warn,
            survive_log: LoggingLevel::Warn,
            vive_log: LoggingLevel::Warn,
        }
    }
}
impl LoggingEnvVars {
    pub fn set_vars(&self, mut command: Exec) -> Exec {
        command = command.env("XRT_COMPOSITOR_LOG", self.compositor_log.to_string());
        // command = command.env("EGL_SWAPCHAIN_LOG", self.egl_swap_chain_log.to_string());
        // command = command.env("D3D_COMPOSITOR_LOG", self.d3d_compositor_log.to_string());
        // command = command.env("HT_LOG", self.ht_log.to_string());
        // command = command.env("CALIB_LOG", self.calibration_log.to_string());
        // command = command.env("XRT_LOG", self.global_log.to_string());
        // command = command.env("AEG_LOG", self.aeg_log.to_string());
        // command = command.env("EGL_LOG", self.egl_log.to_string());
        // command = command.env("MERCURY_LOG", self.mercury_log.to_string());
        // command = command.env("SLAM_LOG", self.slam_log.to_string());
        // command = command.env("SIMPLE_IMU_LOG", self.simple_imu_log.to_string());
        // command = command.env("PSVR_TRACKING_LOG", self.psvr_tracking_log.to_string());
        // command = command.env("DXGI_LOG", self.d3d11_log.to_string());
        // command = command.env("U_PACING_APP_LOG", self.u_pacing_app_log.to_string());
        // command = command.env("U_PACING_COMPOSITOR_LOG", self.u_pacing_compositor_log.to_string());
        // command = command.env("JSON_LOG", self.json_log.to_string());
        // command = command.env("AHARDWAREBUFFER_LOG", self.json_log.to_string());
        // command = command.env("LH_LOG", self.lh_log.to_string());
        // command = command.env("SVR_LOG", self.svr_log.to_string());
        // command = command.env("NS_LOG", self.ns_log.to_string());
        // command = command.env("QWERTY_LOG", self.qwerty_log.to_string());
        // command = command.env("ARDUINO_LOG", self.arduino_log.to_string());
        // command = command.env("HYDRA_LOG", self.hydra_log.to_string());
        // command = command.env("SURVIVE_LOG", self.survive_log.to_string());
        // command = command.env("VIVE_LOG", self.vive_log.to_string());
        command
    }
}
