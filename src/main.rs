mod env_var;
mod monado_control;
mod traits;
mod io;
mod log_options;
mod compositor;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::process::Child;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, sync_channel};
use std::thread::Thread;
use eframe::Frame;
use egui::{Context, Visuals};
use subprocess::Popen;
use crate::compositor::CompositorWindow;
use crate::env_var::EnvVar;
use crate::io::Io;
use crate::monado_control::MonadoControl;
use crate::traits::CtxSect;

pub fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Malek's Monado Gui Utility", native_options, Box::new(|cc| Box::new(MonadoGuiApp::new(cc))));
}
pub struct MonadoGuiApp {
    io: Io,
    monado_control: MonadoControl,
    compositor_window: CompositorWindow,
    child: Option<Popen>,
    pub env_vars: EnvVar,
}

impl Default for MonadoGuiApp {
    fn default() -> Self {
        let channel = sync_channel(64000);
        MonadoGuiApp {
            io: Io::new(channel.1),
            monado_control: MonadoControl::new(channel.0),
            compositor_window: CompositorWindow::new(),
            child: None,
            env_vars: EnvVar::default(),
        }
    }
}

impl eframe::App for MonadoGuiApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        Io::update( self, ctx, frame);
        MonadoControl::update( self, ctx, frame);
        CompositorWindow::update(self, ctx, frame);
    }
}

impl MonadoGuiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}
