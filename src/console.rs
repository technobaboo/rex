use crate::RexApp;
use ansi_parser::{AnsiParser, AnsiSequence, Output};
use egui::text::LayoutJob;
use egui::WidgetText::Galley;
use egui::{Color32, FontFamily, FontId, ScrollArea, TextFormat, Ui};
use std::default::Default;

pub fn update(state: &mut RexApp, ui: &mut Ui) {
    ScrollArea::new([true, false])
        .stick_to_bottom(true)
        .max_width(f32::INFINITY)
        .max_height(f32::INFINITY)
        .show(ui, |ui| {
            if let Ok(input_str) = state.stdout_receiver.try_recv() {
                state.console.push_str(input_str.as_str());
            }
            let mut text_layout_job = LayoutJob::default();
            let mut output = None;
            state.console.ansi_parse().for_each(|a| match a {
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
            let galley = ui.ctx().fonts().layout_job(text_layout_job);
            ui.label(Galley(galley));
        });
}
