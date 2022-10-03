use std::process::Command;
use subprocess::Exec;
use crate::compositor::WindowType;
use crate::log_options::{LoggingEnvVars, LoggingLevel};

pub struct EnvVar {
    pub logging_env_vars: LoggingEnvVars,
    pub window_type: WindowType,
}
impl Default for EnvVar {
    fn default() -> Self {
        EnvVar {
            logging_env_vars: LoggingEnvVars::default(),
            window_type: WindowType::default(),
        }
    }
}
impl EnvVar {
    pub fn set_vars(&self, mut command: Exec) -> Exec {
        command = self.logging_env_vars.set_vars(command);
        command = self.window_type.set_vars(command);
        command
    }
}