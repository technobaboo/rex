use crate::compositor::WindowType;
use serde::{Deserialize, Serialize};
use subprocess::Exec;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EnvVars {
    pub window_type: WindowType,
}
impl EnvVars {
    pub fn set_vars(&self, mut command: Exec) -> Exec {
        command = self.window_type.set_vars(command);
        command
    }
}
