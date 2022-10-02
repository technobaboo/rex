use std::process::Command;
use subprocess::Exec;
use crate::log_options::LoggingLevel;

pub struct EnvVar {
    pub compositor_log_level: LoggingLevel,
}
impl Default for EnvVar {
    fn default() -> Self {
        EnvVar {
            compositor_log_level: LoggingLevel::Debug
        }
    }
}
impl EnvVar {
    pub fn set_vars(&self, mut command: Exec) -> Exec {
        command = command.env("XRT_COMPOSITOR_LOG", self.compositor_log_level.to_string());

        command
    }
}