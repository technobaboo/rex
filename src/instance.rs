use egui::Context;
use libc::pid_t;
use nix::{
    sys::wait::{WaitPidFlag, WaitStatus},
    unistd::Pid,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, ErrorKind},
    path::PathBuf,
    sync::{mpsc::SyncSender, Arc, Mutex},
    thread,
};
use subprocess::{Exec, Popen, Redirection};

use crate::{
    compositor::CompositorSettings, env_var::EnvVars, log_options::LoggingEnvVars, RexApp,
};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct MonadoInstance {
    #[serde(skip)]
    instance_dir: PathBuf,
    pub env_vars: EnvVars,
    pub compositor_settings: CompositorSettings,
    #[serde(skip)]
    pub child: Option<Popen>,
}
impl MonadoInstance {
    pub fn create_load(app: &RexApp, name: String) -> Option<Self> {
        let instance_dir = app.monado_instance_dir.join(&name);
        let mut instance: MonadoInstance =
            confy::load_path(instance_dir.join("instance.toml")).ok()?;
        instance.instance_dir = instance_dir;
        Some(instance)
    }
    pub fn update(&mut self, ctx: &Context) {
        CompositorSettings::update(self, ctx);
    }

    pub fn start_monado(
        &mut self,
        logging_env_vars: &LoggingEnvVars,
        stdout_sender: Arc<Mutex<SyncSender<String>>>,
    ) {
        let mut command = Exec::cmd("monado-service");
        command = logging_env_vars.set_vars(command);
        command = self.env_vars.set_vars(command);
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

        self.child.replace(child);
    }

    pub fn kill_monado(&mut self) -> std::io::Result<()> {
        let Some(mut child) = self.child.take() else {return Err(ErrorKind::BrokenPipe.into())};
        println!("killing: {}", child.pid().unwrap());
        child.kill()?;
        let _ = nix::sys::wait::wait();
        //We don't need this because we wait in the thread.
        //nix::sys::wait::wait().unwrap();
        Ok(())
    }
}
