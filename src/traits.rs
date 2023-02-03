use crate::MonadoGuiApp;
use eframe::Frame;
use egui::{Context, Ui};

pub trait UiSect {
    fn update(state: &mut MonadoGuiApp, ui: &mut Ui);
}

pub trait CtxSect {
    fn update(state: &mut MonadoGuiApp, ctx: &Context, frame: &Frame);
}
