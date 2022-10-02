use std::{io, thread};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::{DerefMut, Div};
use std::os::unix::io::{AsRawFd, RawFd};
use std::process::{Child, ChildStdout, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender, sync_channel, SyncSender, TryRecvError};
use ansi_parser::{AnsiParser, AnsiSequence, Output};
use eframe::Frame;
use egui::{Align2, Color32, Context, FontFamily, FontId, RichText, ScrollArea, Separator, TextFormat, Ui, Vec2, WidgetText};
use egui::emath::align;
use egui::panel::Side;
use egui::text::{Fonts, LayoutJob};
use egui::WidgetType::Label;
use crate::{MonadoGuiApp};
use crate::traits::{CtxSect, UiSect};
use std::default::Default;
use egui::widget_text::WidgetTextGalley;
use egui::WidgetText::Galley;

pub(crate) struct Io {
    pub main_console: MainConsole,
}

impl Io {
    pub fn new(input_receiver: Receiver<String>) -> Self {
        Io {
            main_console: MainConsole::new(input_receiver)
        }
    }
}

impl CtxSect for Io {
    fn update(state: &mut MonadoGuiApp, ctx: &Context, frame: &Frame) {
        egui::Window::new("I/O").collapsible(true).resizable(true).default_size(frame.info().window_info.size.div(Vec2::new(3.0, 3.0))).show(ctx, |ui| {
            ScrollArea::new([true, true]).stick_to_bottom(true).stick_to_right(true).max_width(ui.available_width()-30.0).max_height(ui.available_height()-30.0).show(ui, |ui| {
                let mut text_layout_job = LayoutJob::default();
                let mut output = None;
                &state.io.main_console.input.ansi_parse().for_each(|a| {
                   match a {
                       Output::TextBlock(text_block) => {
                           text_layout_job.append(text_block, 0.0,
                           match &output {
                               None => {
                                   TextFormat {
                                       font_id: FontId::new(14.0, FontFamily::Monospace),
                                       color: Color32::GRAY,
                                       ..Default::default()
                                   }
                               }
                               Some(output) => {
                                   match output {
                                       AnsiSequence::SetGraphicsMode(graphics) => {
                                           let mut color = Color32::GRAY;
                                           let my_vec = graphics.to_vec();
                                           if graphics.len() > 1 {
                                               match graphics.get(1).unwrap() {
                                                   33 => {color = Color32::YELLOW}
                                                   34 => {color = Color32::BLUE}
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
                                       _ => {
                                           TextFormat {
                                               font_id: FontId::new(14.0, FontFamily::Monospace),
                                               color: Color32::GRAY,
                                               ..Default::default()
                                           }
                                       }
                                   }
                               }
                           })
                       }
                       Output::Escape(ansi) => {
                           output = Some(ansi);
                       }
                   }
                });
                let galley = ctx.fonts().layout_job(text_layout_job);
                ui.label(Galley(galley));
                //ui.label(WidgetText::from(RichText::new(&state.io.main_console.input).monospace()));
            });
            MainConsole::update(state, ui);
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
            input_receiver
        }
    }
}
impl UiSect for MainConsole {
    fn update(state: &mut MonadoGuiApp, ui: &mut Ui) {
        let mut this = &mut state.io.main_console;
        ui.horizontal(|ui| {
            if ui.button("Copy To Clipboard").clicked() {
                let mut output_string = String::new();
                this.input.ansi_parse().for_each(|ansi| {
                   match ansi {
                       Output::TextBlock(block) => {
                           output_string.push_str(block);
                       }
                       Output::Escape(_) => {}
                   }
                });
                ui.output().copied_text = output_string;
            };
            if ui.button("Clear").clicked() {
                this.input = String::new();
            }
        });
        match this.input_receiver.try_recv() {
            Ok(input_str) => {
                this.input.push_str(input_str.as_str());
            }
            Err(_) => {}
        }
    }
}