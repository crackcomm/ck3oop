use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

const ENV_CARGO_EXTRA_BUILD_DIR: &str = "CARGO_EXTRA_BUILD_DIR";
const ENV_CARGO_EXTRA_BUILD_FILE: &str = "CARGO_EXTRA_BUILD_FILE";

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Build {
    pub tauri_app_binary: Option<String>,
    pub webdriver_binary: Option<String>,
    pub tauri_driver_binary: Option<String>,
}

impl Build {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize build file")
    }

    pub fn load() -> Result<Self> {
        let build_file = get_full_path_to_build_file()?;
        let content = std::fs::read_to_string(build_file).context("failed to read build file")?;
        serde_json::from_str(&content).context("failed to parse build file")
    }

    pub fn save(&self) -> Result<()> {
        let build_file = get_full_path_to_build_file()?;
        let content =
            serde_json::to_string_pretty(self).context("failed to serialize build file")?;
        std::fs::write(build_file, content).context("failed to write build file")?;
        Ok(())
    }

    pub fn update(&mut self, with: Self) {
        if let Some(tauri_app_binary) = with.tauri_app_binary {
            self.tauri_app_binary = Some(tauri_app_binary);
        }
        if let Some(webdriver_binary) = with.webdriver_binary {
            self.webdriver_binary = Some(webdriver_binary);
        }
        if let Some(tauri_driver_binary) = with.tauri_driver_binary {
            self.tauri_driver_binary = Some(tauri_driver_binary);
        }
    }
}

pub fn get_env(key: &str) -> Result<String> {
    Ok(std::env::var_os(key)
        .context(format!("env var :{} not set", key))?
        .to_str()
        .context(format!(":{} is not a valid UTF-8 string", key))?
        .to_string())
}

pub fn get_full_path_to_build_file() -> Result<PathBuf> {
    let build_dir = get_env(ENV_CARGO_EXTRA_BUILD_DIR)?;
    let build_file = get_env(ENV_CARGO_EXTRA_BUILD_FILE)?;
    let full_path = Path::new(&build_dir).join(build_file);
    Ok(full_path)
}

pub fn get_full_path_to_build_dir() -> Result<PathBuf> {
    let build_dir = get_env(ENV_CARGO_EXTRA_BUILD_DIR)?;
    let build_path = Path::new(&build_dir);
    Ok(build_path.to_path_buf())
}
