extern crate core;

use std::ops::{Add, RangeInclusive};
use std::os::unix::process::CommandExt;
use std::os::unix::raw::pid_t;
use std::process::{Child, Command};
use std::thread::sleep;
use eframe::egui;
use egui::{Color32, Ui, Widget, WidgetText, WidgetType};
use egui::emath::Numeric;
use egui::WidgetType::Button;
use fork::Fork;
use crate::MonadoRunning::{False, True};
use crate::monado_env_var::MonadoEnvVar;
use crate::monado_env_var::xrt::{WindowType, XcbScreenNumber, XcbScreenType};
use crate::monado_env_var::xrt::WindowType::{Auto, NvidiaDirect, RandrDirect, Wayland, WaylandDirect};
use crate::WindowType::{Vk, Xcb};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    pub monado_running: MonadoRunning,
    pub monado_env_var: MonadoEnvVar,
}

enum MonadoRunning {
    True(Child),
    False
}
impl MonadoRunning {
    pub fn running(&self) -> bool {
        match self {
            True(_) => {true}
            False => {false}
        }
    }
}
impl Default for MonadoRunning {
    fn default() -> Self {
        Self::False
    }
}

pub enum Incomplete {
    Incomplete
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

pub mod monado_env_var {
    use crate::monado_env_var::xrt::Xrt;

    #[derive(Default)]
    pub struct MonadoEnvVar {
        pub xrt: Xrt
    }
    pub mod xrt {
        use crate::{Incomplete, LoggingLevel};
        pub struct Compositor {
            pub log: LoggingLevel,
            pub print_modes: bool,
            pub window_type: WindowType,
            pub xcb_fullscreen: bool,
            pub xcb_screen: u32,
            pub nvidia_str: String,
            pub nvidia_str_enabled: bool,
            pub force_gpu_index: i32,
            pub force_client_gpu_index: i32,
            pub desired_mode: i32,
            pub scale_percentage: u32,
            pub default_framerate: u32,
            pub compute: bool,
        }
        impl Default for Compositor {
            fn default() -> Self {
                Compositor {
                    log: LoggingLevel::Info,
                    print_modes: false,
                    window_type: WindowType::default(),
                    nvidia_str: String::default(),
                    nvidia_str_enabled: false,
                    xcb_fullscreen: false,
                    xcb_screen: 0,
                    force_gpu_index: -1,
                    force_client_gpu_index: -1,
                    desired_mode: -1,
                    scale_percentage: 140,
                    default_framerate: 60,
                    compute: false
                }
            }
        }
        #[derive(PartialEq)]
        pub enum WindowType {
            Auto,
            NvidiaDirect(Option<String>),
            Vk(u32),
            RandrDirect,
            WaylandDirect,
            Xcb(XcbScreenType, XcbScreenNumber),
            Wayland,
        }
        impl Default for WindowType {
            fn default() -> Self {
                WindowType::Auto
            }
        }
        #[derive(PartialEq)]
        pub enum XcbScreenType {
            Fullscreen,
            Windowed
        }
        impl Default for XcbScreenType {
            fn default() -> Self {
                XcbScreenType::Windowed
            }
        }
        #[derive(PartialEq)]
        pub struct XcbScreenNumber(pub u32);
        pub struct Xrt {
            pub compositor: Compositor,
            pub window_peek: Incomplete,
        }
        impl Default for Xrt {
            fn default() -> Self {
                Xrt {
                    compositor: Compositor::default(),
                    window_peek: Incomplete::Incomplete
                }
            }
        }
    }
}


impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let start_button = ui.add_enabled(!self.monado_running.running(), egui::Button::new("Start Monado").fill(Color32::from_rgb(0, 40, 0)));
            let stop_button = ui.add_enabled(self.monado_running.running(), egui::Button::new("Stop Monado").fill(Color32::from_rgb(40, 0, 0)));
            if start_button.clicked() {
                self.monado_running = True(start_monado(&self.monado_env_var));
            }
            if stop_button.clicked() {
                let mut running = &mut self.monado_running;
                match &mut running {
                    True(pid) => { kill_monado(pid);}
                    False => { panic!("monado wasn't running!"); }
                }
                self.monado_running = False;
            }
            egui::CollapsingHeader::new("XRT").show(ui, |ui| {
                egui::CollapsingHeader::new("Compositor").show(ui, |ui| {
                    let compositor = &mut self.monado_env_var.xrt.compositor;
                    egui::CollapsingHeader::new("logging").show(ui, |ui| {
                        ui.radio_value(&mut compositor.log, LoggingLevel::Trace, "Trace")
                            .on_hover_ui(|ui| {ui.label("Trace messages, highly verbose");});
                        ui.radio_value(&mut compositor.log, LoggingLevel::Debug, "Debug")
                            .on_hover_ui(|ui| {ui.label("Debug messages, verbose.");});
                        ui.radio_value(&mut compositor.log, LoggingLevel::Info, "Info")
                            .on_hover_ui(|ui| {ui.label("Info messages: not very verbose, not indicating a problem.");});
                        ui.radio_value(&mut compositor.log, LoggingLevel::Warn, "Warn")
                            .on_hover_ui(|ui| {ui.label("Warning messages: indicating a potential problem");});
                        ui.radio_value(&mut compositor.log, LoggingLevel::Error, "Error")
                            .on_hover_ui(|ui| {ui.label("Error messages: indicating a problem");});
                    });
                    egui::CollapsingHeader::new("compositor forcing").show(ui, |ui| {
                        ui.radio_value(&mut compositor.window_type, Auto, "Auto");
                        ui.radio_value(&mut compositor.window_type, Wayland, "Wayland");
                        ui.radio_value(&mut compositor.window_type, WaylandDirect, "Wayland Direct");
                        ui.radio_value(&mut compositor.window_type, RandrDirect, "Randr Direct");
                        {
                            ui.horizontal_wrapped(|ui| {
                                let radio = ui.add(egui::RadioButton::new( matches!(&mut compositor.window_type, NvidiaDirect(_)), "Nvidia Direct"));
                                if radio.clicked() {
                                    *&mut compositor.window_type = NvidiaDirect(Some(String::new()));
                                }
                                let is_enabled = matches!(&mut compositor.window_type, NvidiaDirect(_));
                                ui.add_enabled_ui(is_enabled, |ui| ui.checkbox(&mut compositor.nvidia_str_enabled, "Enable Custom Display String"));
                                let enable_display_str = compositor.nvidia_str_enabled;
                                ui.add_enabled_ui((enable_display_str && is_enabled), |ui| ui.text_edit_singleline(&mut compositor.nvidia_str) );
                            });
                        }
                        {

                                let radio = ui.add(egui::RadioButton::new( matches!(&mut compositor.window_type, Xcb(_,_)), "Xcb")).on_hover_ui(|ui| { ui.label("Do not use direct mode and run the Monado compositor in a window."); });
                                if radio.clicked() {
                                    *&mut compositor.window_type = Xcb(XcbScreenType::Windowed, XcbScreenNumber(0));
                                }
                                egui::CollapsingHeader::new(WidgetText::default()).open(Some(matches!(&mut compositor.window_type, Xcb(_,_)))).enabled(matches!(&mut compositor.window_type, Xcb(_,_))).show(ui, |ui| {
                                    ui.checkbox(&mut compositor.xcb_fullscreen, "Fullscreen");
                                    ui.add(egui::DragValue::new(&mut compositor.xcb_screen).clamp_range(RangeInclusive::new(0, 10)));
                                });

                        }
                    });
                }).header_response.on_hover_ui(|ui| { ui.hyperlink("https://monado.pages.freedesktop.org/monado/comp__settings_8h_source.html"); });
            });
            //xrt
            // egui::CollapsingHeader::new("XRT").show(ui, |ui| {
            //    egui::CollapsingHeader::new("Compositor").show(ui, |ui| {
            //        let compositor = &mut self.monado_env_var.xrt.compositor;
            //
            //        egui::CollapsingHeader::new("Force").show(ui, |ui| {
            //            ui.checkbox(&mut compositor.force_randr, "Force Randr");
            //            ui.checkbox(&mut compositor.force_wayland, "Force Wayland");
            //            ui.checkbox(&mut compositor.force_wayland_direct, "Force Wayland Direct");
            //            ui.checkbox(&mut compositor.force_nvidia, "Force Nvidia");
            //            ui.label("Force Nvidia Display <INCOMPLETE>");
            //            ui.checkbox(&mut compositor.force_xcb, "Force Xcb");
            //            wrap_label_range(ui, &mut compositor.force_vk_display, "Force VK Display", RangeInclusive::new(-1, 4));
            //            wrap_label_range(ui, &mut compositor.force_gpu_index, "Force GPU Index", RangeInclusive::new(-1, 4));
            //            wrap_label_range(ui, &mut compositor.force_client_gpu_index, "Force Client GPU Index", RangeInclusive::new(-1, 4));
            //        });
            //        ui.checkbox(&mut compositor.print_modes, "Print Modes");
            //        ui.checkbox(&mut compositor.compute, "Compute");
            //        ui.checkbox(&mut compositor.xcb_full_screen, "XCB Fullscreen");
            //        wrap_label_range(ui, &mut compositor.xcb_display, "XCB Display", RangeInclusive::new(-1, 4));
            //        wrap_label_range(ui, &mut compositor.desired_mode, "Desired Mode", RangeInclusive::new(-1, 4));
            //        wrap_label_range(ui, &mut compositor.scale_percentage, "Scale Percentage", RangeInclusive::new(5, 500));
            //        wrap_label_range(ui, &mut compositor.default_framerate, "Default Framerate", RangeInclusive::new(0, 500));
            //        ui.label("Log <INCOMPLETE>");
            //    });
            //     ui.label("Window Peek <INCOMPLETE>");
            // });
        });
    }
}

fn wrap_label_range<Num: Numeric>(ui: &mut Ui, value: &mut Num, label: &str, range: RangeInclusive<Num>) {
    ui.horizontal_wrapped(|ui| {
       ui.label(label);
        ui.add(egui::DragValue::new(value).clamp_range(range));
    });
}

fn start_monado(monado_env_var: &MonadoEnvVar) -> Child {
    let mut monado_service = Command::new("monado-service");
    {
        let xrt = &monado_env_var.xrt;
        {
            let compositor = &xrt.compositor;
            {
                monado_service.env("XRT_COMPOSITOR_LOG", compositor.log.to_string());
                match &compositor.window_type {
                    Auto => {}
                    NvidiaDirect(opt_display_string) => {
                        monado_service.env("XRT_COMPOSITOR_FORCE_NVIDIA", "true");
                        match opt_display_string {
                            None => {}
                            Some(display_str) => {monado_service.env("XRT_COMPOSITOR_FORCE_NVIDIA_DISPLAY", display_str);}
                        }
                    }
                    Vk(vk) => { monado_service.env("XRT_COMPOSITOR_FORCE_VK_DISPLAY", vk.to_string()); }
                    RandrDirect => { monado_service.env("XRT_COMPOSITOR_FORCE_RANDR", "true"); }
                    WaylandDirect => { monado_service.env("XRT_COMPOSITOR_FORCE_WAYLAND_DIRECT", "true"); }
                    Xcb(screen_type, screen_number) => {
                        monado_service.env("XRT_COMPOSITOR_FORCE_XCB", "true");
                        monado_service.env("XRT_COMPOSITOR_XCB_FULLSCREEN", matches!(screen_type, XcbScreenType::Fullscreen).to_string());
                        monado_service.env("XRT_COMPOSITOR_XCB_DISPLAY", screen_number.0.to_string());
                    }
                    Wayland => { monado_service.env("XRT_COMPOSITOR_FORCE_WAYLAND", "true"); }
                };
                // monado_service.env("XRT_COMPOSITOR_PRINT_MODES", compositor.print_modes.to_string());
                // monado_service.env("XRT_COMPOSITOR_FORCE_RANDR", compositor.force_randr.to_string());
                // monado_service.env("XRT_COMPOSITOR_FORCE_WAYLAND_DIRECT", compositor.force_wayland_direct.to_string());
                // monado_service.env("XRT_COMPOSITOR_FORCE_NVIDIA", compositor.force_nvidia.to_string());
                //monado_service.env(head_str.add("FORCE_NVIDIA_DISPLAY"), compositor.force_nvidia_display);
                // monado_service.env("XRT_COMPOSITOR_FORCE_VK_DISPLAY", compositor.force_vk_display.to_string());
                // monado_service.env("XRT_COMPOSITOR_FORCE_XCB", compositor.force_xcb.to_string());
                // monado_service.env("XRT_COMPOSITOR_FORCE_WAYLAND", compositor.force_wayland.to_string());
                // monado_service.env("XRT_COMPOSITOR_FORCE_GPU_INDEX", compositor.force_gpu_index.to_string());
                // monado_service.env("XRT_COMPOSITOR_FORCE_CLIENT_GPU_INDEX", compositor.force_client_gpu_index.to_string());
                // monado_service.env("XRT_COMPOSITOR_DESIRED_MODE", compositor.desired_mode.to_string());
                // monado_service.env("XRT_COMPOSITOR_SCALE_PERCENTAGE", compositor.scale_percentage.to_string());
                // monado_service.env("XRT_COMPOSITOR_XCB_FULLSCREEN", compositor.xcb_full_screen.to_string());
                // monado_service.env("XRT_COMPOSITOR_XCB_DISPLAY", compositor.xcb_display.to_string());
                // monado_service.env("XRT_COMPOSITOR_DEFAULT_FRAMERATE", compositor.default_framerate.to_string());
                // monado_service.env("XRT_COMPOSITOR_COMPUTE", compositor.compute.to_string());
            }
            //monado_service.env("XRT_WINDOW_PEAK", xrt.window_peek.to_string());
        }
    }

    monado_service.spawn().unwrap()
}
fn kill_monado(pid: &mut Child) {
    pid.kill().unwrap();
    nix::sys::wait::wait().unwrap();
}