use eframe::Frame;
use egui::{Context, Ui};
use crate::MonadoGuiApp;

pub trait UiSect {
    fn update(state: &mut MonadoGuiApp, ui: &mut Ui);
}

pub trait CtxSect {
    fn update(state: &mut MonadoGuiApp, ctx: &Context, frame: &Frame);
}