use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::Size;

const DEFAULT_SHELL: &str = "/bin/bash";

#[derive(Debug, Clone)]
pub struct BackendSettings {
    pub shell: String,
    pub args: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub env: HashMap<String, String>,
    /// Initial terminal layout size (width, height in pixels). When provided
    /// together with [`initial_cell_metrics`] the PTY is spawned at the correct
    /// column/row count from the very beginning, avoiding a resize on the first
    /// rendered frame.
    pub initial_layout_size: Option<Size>,
    /// Initial font cell metrics (cell width, cell height in pixels) used to
    /// compute the initial column/row count. See [`initial_layout_size`].
    pub initial_cell_metrics: Option<Size>,
}

impl Default for BackendSettings {
    fn default() -> Self {
        let mut env = HashMap::new();
        env.insert("TERM".to_string(), "xterm-256color".to_string());
        env.insert("COLORTERM".to_string(), "truecolor".to_string());

        Self {
            shell: DEFAULT_SHELL.to_string(),
            args: vec![],
            working_directory: None,
            env,
            initial_layout_size: None,
            initial_cell_metrics: None,
        }
    }
}
