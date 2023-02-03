use crate::compositor::WindowType::{
    Auto, NvidiaDirect, RandrDirect, Vk, Wayland, WaylandDirect, Xcb,
};
use crate::traits::UiSect;
use crate::{CtxSect, MonadoGuiApp};
use eframe::Frame;
use egui::{Context, Ui, WidgetText};
use std::ops::RangeInclusive;
use subprocess::Exec;

pub struct CompositorWindow {
    pub forcing: Forcing,
}
impl CompositorWindow {
    pub fn new() -> Self {
        CompositorWindow {
            forcing: Forcing::default(),
        }
    }
}
impl CtxSect for CompositorWindow {
    fn update(state: &mut MonadoGuiApp, ctx: &Context, _frame: &Frame) {
        egui::Window::new("Compositor")
            .collapsible(true)
            .show(ctx, |ui| {
                Forcing::update(state, ui);
            });
    }
}
pub struct Forcing {
    pub nvidia_str_enabled: bool,
    pub nvidia_str: String,
    pub xcb_fullscreen: bool,
    pub xcb_screen: u32,
}
impl Default for Forcing {
    fn default() -> Self {
        Forcing {
            nvidia_str_enabled: false,
            nvidia_str: "".to_string(),
            xcb_fullscreen: false,
            xcb_screen: 0,
        }
    }
}
impl UiSect for Forcing {
    fn update(state: &mut MonadoGuiApp, ui: &mut Ui) {
        ui.collapsing("Forcing", |ui| {
            ui.radio_value(&mut state.env_vars.window_type, Auto, "Auto");
            ui.radio_value(&mut state.env_vars.window_type, Wayland, "Wayland");
            ui.radio_value(
                &mut state.env_vars.window_type,
                WaylandDirect,
                "Wayland Direct",
            );
            ui.radio_value(&mut state.env_vars.window_type, RandrDirect, "Randr Direct");
            {
                ui.horizontal_wrapped(|ui| {
                    let radio = ui.add(egui::RadioButton::new(
                        matches!(&mut state.env_vars.window_type, NvidiaDirect(_)),
                        "Nvidia Direct",
                    ));
                    if radio.clicked() {
                        state.env_vars.window_type = NvidiaDirect(Some(String::new()));
                    }
                    let is_enabled = matches!(&mut state.env_vars.window_type, NvidiaDirect(_));
                    ui.add_enabled_ui(is_enabled, |ui| {
                        ui.checkbox(
                            &mut state.compositor_window.forcing.nvidia_str_enabled,
                            "Enable Custom Display String",
                        )
                    });
                    let enable_display_str = state.compositor_window.forcing.nvidia_str_enabled;
                    ui.add_enabled_ui(enable_display_str && is_enabled, |ui| {
                        ui.text_edit_singleline(&mut state.compositor_window.forcing.nvidia_str)
                    });
                });
            }
            {
                let radio = ui
                    .add(egui::RadioButton::new(
                        matches!(&mut state.env_vars.window_type, Xcb(_, _)),
                        "Xcb",
                    ))
                    .on_hover_ui(|ui| {
                        ui.label(
                            "Do not use direct mode and run the Monado compositor in a window.",
                        );
                    });
                if radio.clicked() {
                    state.env_vars.window_type =
                        Xcb(XcbScreenType::Windowed, XcbScreenNumber(0));
                }
                egui::CollapsingHeader::new(WidgetText::default())
                    .open(Some(matches!(&mut state.env_vars.window_type, Xcb(_, _))))
                    .enabled(matches!(&mut state.env_vars.window_type, Xcb(_, _)))
                    .show(ui, |ui| {
                        ui.checkbox(
                            &mut state.compositor_window.forcing.xcb_fullscreen,
                            "Fullscreen",
                        );
                        ui.add(
                            egui::DragValue::new(&mut state.compositor_window.forcing.xcb_screen)
                                .clamp_range(RangeInclusive::new(0, 10)),
                        );
                    });
            }
        });
    }
}
#[derive(PartialEq, Eq)]
pub enum WindowType {
    Auto,
    NvidiaDirect(Option<String>),
    Vk(u32),
    RandrDirect,
    WaylandDirect,
    Xcb(XcbScreenType, XcbScreenNumber),
    Wayland,
}
impl WindowType {
    pub fn set_vars(&self, mut command: Exec) -> Exec {
        match &self {
            Auto => {}
            NvidiaDirect(opt_display_string) => {
                command = command.env("XRT_COMPOSITOR_FORCE_NVIDIA", "true");
                match opt_display_string {
                    None => {}
                    Some(display_str) => {
                        command = command.env("XRT_COMPOSITOR_FORCE_NVIDIA_DISPLAY", display_str);
                    }
                }
            }
            Vk(vk) => {
                command = command.env("XRT_COMPOSITOR_FORCE_VK_DISPLAY", vk.to_string());
            }
            RandrDirect => {
                command = command.env("XRT_COMPOSITOR_FORCE_RANDR", "true");
            }
            WaylandDirect => {
                command = command.env("XRT_COMPOSITOR_FORCE_WAYLAND_DIRECT", "true");
            }
            Xcb(screen_type, screen_number) => {
                command = command.env("XRT_COMPOSITOR_FORCE_XCB", "true");
                command = command.env(
                    "XRT_COMPOSITOR_XCB_FULLSCREEN",
                    matches!(screen_type, XcbScreenType::Fullscreen).to_string(),
                );
                command = command.env("XRT_COMPOSITOR_XCB_DISPLAY", screen_number.0.to_string());
            }
            Wayland => {
                command = command.env("XRT_COMPOSITOR_FORCE_WAYLAND", "true");
            }
        }
        command
    }
}
impl Default for WindowType {
    fn default() -> Self {
        WindowType::Auto
    }
}
#[derive(PartialEq, Eq)]
pub enum XcbScreenType {
    Fullscreen,
    Windowed,
}
impl Default for XcbScreenType {
    fn default() -> Self {
        XcbScreenType::Windowed
    }
}
#[derive(PartialEq, Eq)]
pub struct XcbScreenNumber(pub u32);
