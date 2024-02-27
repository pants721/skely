use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub editor: Option<String>,
    pub placeholder: Option<String>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            editor: None,
            placeholder: None,
        }
    }

    pub fn default() -> Self {
        Settings {
            editor: Some("".to_string()),
            placeholder: Some("PLACEHOLDER".to_string()),
        }
    }

    pub fn load(cfg_path: PathBuf) -> Result<Self> {
        let mut cfg_file = File::open(cfg_path)?;
        let mut contents = String::new();
        cfg_file.read_to_string(&mut contents)?;
        let mut s: Settings = toml::from_str(&contents)?;
        // Make this loop over struct fields
        if let Some(editor) = &s.editor {
            if editor.is_empty() {
                s.editor = None;
            }
        }

        if let Some(placeholder) = &s.placeholder {
            if placeholder.is_empty() {
                s.placeholder = None;
            }
        }
        Ok(s)
    }

    pub fn create_default_cfg_file(path: PathBuf) -> Result<()> {
        let settings = Settings::default();

        let serialized = toml::to_string_pretty(&settings)?;

        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }
}
