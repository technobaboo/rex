use crate::compositor::WindowType;
use crate::log_options::LoggingEnvVars;

use subprocess::Exec;

#[derive(Default)]
pub struct EnvVar {
    pub logging_env_vars: LoggingEnvVars,
    pub window_type: WindowType,
}
impl EnvVar {
    pub fn set_vars(&self, mut command: Exec) -> Exec {
        command = self.logging_env_vars.set_vars(command);
        command = self.window_type.set_vars(command);
        command
    }
}
