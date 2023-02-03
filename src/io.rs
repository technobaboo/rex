use crate::log_options::LogOptions;
use crate::traits::{CtxSect, UiSect};
use crate::MonadoGuiApp;
use ansi_parser::{AnsiParser, AnsiSequence, Output};
use eframe::Frame;

use egui::text::LayoutJob;

use egui::WidgetText::Galley;

use egui::{Color32, Context, FontFamily, FontId, Pos2, ScrollArea, TextFormat, Ui, Vec2};

use std::default::Default;

use std::ops::Div;

use std::sync::mpsc::Receiver;

pub(crate) struct Io {
    pub main_console: MainConsole,
}

impl Io {
    pub fn new(input_receiver: Receiver<String>) -> Self {
        Io {
            main_console: MainConsole::new(input_receiver),
        }
    }
}

impl CtxSect for Io {
    fn update(state: &mut MonadoGuiApp, ctx: &Context, frame: &Frame) {
        egui::Window::new("I/O")
            .default_pos(Pos2::new(frame.info().window_info.size.x, 0.0))
            .collapsible(true)
            .resizable(true)
            .default_size(frame.info().window_info.size.div(Vec2::new(3.0, 3.0)))
            .show(ctx, |ui| {
                LogOptions::update(state, ui);
                MainConsole::update(state, ui);
                special_text_area(state, ctx, ui);
            });
    }
}

pub struct MainConsole {
    pub input: String,
    pub input_receiver: Receiver<String>,
}
impl MainConsole {
    pub fn new(input_receiver: Receiver<String>) -> Self {
        MainConsole {
            input: String::new(),
            input_receiver,
        }
    }
}
pub fn special_text_area(state: &mut MonadoGuiApp, ctx: &Context, ui: &mut Ui) {
    ScrollArea::new([true, true])
        .stick_to_bottom(true)
        .max_width(ui.available_width() - 30.0)
        .max_height(ui.available_height() - 30.0)
        .show(ui, |ui| {
            let mut text_layout_job = LayoutJob::default();
            let mut output = None;
            state
                .io
                .main_console
                .input
                .ansi_parse()
                .for_each(|a| match a {
                    Output::TextBlock(text_block) => text_layout_job.append(
                        text_block,
                        0.0,
                        match &output {
                            None => TextFormat {
                                font_id: FontId::new(14.0, FontFamily::Monospace),
                                color: Color32::GRAY,
                                ..Default::default()
                            },
                            Some(AnsiSequence::SetGraphicsMode(graphics)) => {
                                let mut color = Color32::GRAY;
                                let _my_vec = graphics.to_vec();
                                if graphics.len() > 1 {
                                    match graphics.get(1).unwrap() {
                                        31 => {
                                            color = Color32::RED;
                                        }
                                        32 => {
                                            color = Color32::GREEN;
                                        }
                                        33 => {
                                            color = Color32::YELLOW;
                                        }
                                        34 => {
                                            color = Color32::BLUE;
                                        }
                                        35 => {
                                            color = Color32::from_rgb(255, 0, 255);
                                        }
                                        36 => {
                                            color = Color32::from_rgb(0, 255, 255);
                                        }
                                        37 => {
                                            color = Color32::from_rgb(255, 255, 255);
                                        }
                                        &_ => {}
                                    }
                                } else {
                                    color = Color32::GRAY;
                                }
                                TextFormat {
                                    font_id: FontId::new(14.0, FontFamily::Monospace),
                                    color,
                                    ..Default::default()
                                }
                            }
                            Some(_) => TextFormat {
                                font_id: FontId::new(14.0, FontFamily::Monospace),
                                color: Color32::GRAY,
                                ..Default::default()
                            },
                        },
                    ),
                    Output::Escape(ansi) => {
                        output = Some(ansi);
                    }
                });
            let galley = ctx.fonts().layout_job(text_layout_job);
            ui.label(Galley(galley));
        });
}
impl UiSect for MainConsole {
    fn update(state: &mut MonadoGuiApp, ui: &mut Ui) {
        let this = &mut state.io.main_console;
        ui.horizontal(|ui| {
            if ui.button("Copy To Clipboard").clicked() {
                let mut output_string = String::new();
                this.input.ansi_parse().for_each(|ansi| match ansi {
                    Output::TextBlock(block) => {
                        output_string.push_str(block);
                    }
                    Output::Escape(_) => {}
                });
                ui.output().copied_text = output_string;
            };
            if ui.button("Clear").clicked() {
                this.input = String::new();
            }
        });
        if let Ok(input_str) = this.input_receiver.try_recv() {
            this.input.push_str(input_str.as_str());
        }
    }
}
