use crate::traits::CtxSect;
use crate::{EnvVar, MonadoGuiApp};
use eframe::Frame;
use egui::panel::TopBottomSide;
use egui::{Color32, Context, Widget};

use libc::pid_t;
use nix::sys::wait::{WaitPidFlag, WaitStatus};
use nix::unistd::Pid;

use std::io::{BufRead, BufReader};

use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};
use std::thread;
use subprocess::{Exec, Popen, Redirection};

pub struct MonadoControl {
    stdout_sender: Arc<Mutex<SyncSender<String>>>,
}

impl MonadoControl {
    pub fn new(sender: SyncSender<String>) -> Self {
        MonadoControl {
            stdout_sender: Arc::new(Mutex::new(sender)),
        }
    }
}

impl CtxSect for MonadoControl {
    fn update(state: &mut MonadoGuiApp, ctx: &Context, _frame: &Frame) {
        egui::TopBottomPanel::new(TopBottomSide::Bottom, "control_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let start_button = ui.add_enabled_ui(state.child.is_none(), |ui| {
                    egui::Button::new("Start")
                        .fill(Color32::from_rgb(0, 40, 0))
                        .ui(ui)
                });
                let stop_button = ui.add_enabled_ui(state.child.is_some(), |ui| {
                    egui::Button::new("Stop")
                        .fill(Color32::from_rgb(40, 0, 0))
                        .ui(ui)
                });
                let restart_button = ui.add_enabled_ui(state.child.is_some(), |ui| {
                    egui::Button::new("Restart")
                        .fill(Color32::from_rgb(20, 20, 0))
                        .ui(ui)
                });
                if start_button.inner.clicked() {
                    let env_vars = &mut state.env_vars;
                    state.child.replace(start_monado(
                        env_vars,
                        state.monado_control.stdout_sender.clone(),
                    ));
                }
                if stop_button.inner.clicked() {
                    match &mut state.child {
                        None => {}
                        Some(child) => {
                            kill_monado(child);
                            state.child.take();
                        }
                    }
                }
                if restart_button.inner.clicked() {
                    match &mut state.child {
                        None => {}
                        Some(child) => {
                            kill_monado(child);
                            state.child.take();
                        }
                    }
                    let env_vars = &mut state.env_vars;
                    state.child.replace(start_monado(
                        env_vars,
                        state.monado_control.stdout_sender.clone(),
                    ));
                }
            });
        });
    }
}

fn kill_monado(child: &mut Popen) {
    println!("killing: {}", child.pid().unwrap());
    child.kill().unwrap();
    let _ = nix::sys::wait::wait();
    //We don't need this because we wait in the thread.
    //nix::sys::wait::wait().unwrap();
}
fn start_monado(
    monado_env_var: &mut EnvVar,
    stdout_sender: Arc<Mutex<SyncSender<String>>>,
) -> Popen {
    let mut command = Exec::cmd("monado-service");
    command = monado_env_var.set_vars(command);
    command = command.stderr(Redirection::Merge);
    command = command.stdout(Redirection::Pipe);
    command = command.stdin(Redirection::None);
    let mut child = command.popen().unwrap();
    let pid = child.pid().unwrap();
    let stdout = child.stdout.take().unwrap();
    let sender = stdout_sender;
    thread::spawn(move || {
        let b = stdout;
        let child_pid = pid;
        let sender = sender.lock().unwrap().clone();
        loop {
            match nix::sys::wait::waitpid(
                Pid::from_raw(child_pid as pid_t),
                Some(WaitPidFlag::WNOHANG),
            )
            .unwrap()
            {
                WaitStatus::StillAlive => {}
                _ => {
                    println!("monado is dead");
                    return;
                }
            }
            match b.try_clone() {
                Ok(b) => {
                    let mut reader = BufReader::new(b);
                    let mut my_string = String::new();
                    match reader.read_line(&mut my_string) {
                        Ok(_) => {}
                        Err(_) => {
                            return;
                        }
                    }
                    // Don't know why this needs to be here, just added it on a hunch and now shit works again, idfk lmao
                    if my_string.is_empty() {
                        continue;
                    }
                    match sender.send(my_string) {
                        Ok(_) => {}
                        Err(_) => {
                            return;
                        }
                    }
                }
                Err(_) => {
                    return;
                }
            }
        }
    });

    child
}
