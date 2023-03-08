use serde::Deserialize;
use std::fs::File;
use anyhow::Result;
use std::io::Read;
use crate::common::{check_cfg, sk_cfg_dir};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub editor: Option<String>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            editor: None,
        }
    }

    pub fn default() -> Result<Self> {
        let mut cfg_file_path = sk_cfg_dir()?;
        cfg_file_path.push("config.toml");
        check_cfg()?;

        let mut cfg_file = File::open(cfg_file_path)?;
        let mut contents = String::new();
        cfg_file.read_to_string(&mut contents)?;
        let mut s: Settings = toml::from_str(&contents)?;
        if let Some(editor) = &s.editor {
            if editor.is_empty() {
                s.editor = None;
            }
        }
        Ok(s)
    }
}
