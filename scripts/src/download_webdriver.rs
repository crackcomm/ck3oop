use anyhow::{anyhow, Context, Result};
use cargo_run_bin::{binary, metadata};
use reqwest::blocking::get;
use serde::Serialize;
use std::path::Path;
use std::process::ExitCode;
use std::{io, process};

fn download_tauri_driver() -> Result<String> {
    let binary_package = metadata::get_binary_packages()?
        .iter()
        .find(|e| e.package == "tauri-driver")
        .unwrap()
        .to_owned();
    let bin_path = binary::install(binary_package)?;
    Ok(bin_path)
}

const REG_PATH: &str =
    r#"SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}"#;

#[cfg(windows)]
fn get_microsoft_edge_version() -> io::Result<String> {
    use winreg::enums::*;
    use winreg::RegKey;
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(REG_PATH)?
        .get_value("pv")
}

fn download_microsoft_edge_driver(version: &str) -> Result<String> {
    let url = format!(
        "https://msedgedriver.azureedge.net/{}/edgedriver_win64.zip",
        version
    );

    let project_root =
        std::env::var_os("CARGO_EXTRA_BIN_DIR").context("CARGO_EXTRA_BIN_DIR not set")?;
    let project_root = Path::new(&project_root);
    let bin_path = &project_root.join(".bin");
    let webdriver_path = &bin_path.join(format!("msedgedriver-{}.exe", version));

    if Path::new(webdriver_path).exists() {
        return Ok(webdriver_path.to_str().unwrap().to_string());
    }

    if !Path::new(bin_path).exists() {
        std::fs::create_dir_all(bin_path).unwrap();
    }

    let mut temp_file = tempfile::tempfile().unwrap();
    get(url).unwrap().copy_to(&mut temp_file)?;
    let mut archive = zip::ZipArchive::new(temp_file).unwrap();
    match archive.by_name("msedgedriver.exe") {
        Ok(mut file) => {
            let mut file2 = std::fs::File::create_new(webdriver_path).unwrap();
            std::io::copy(&mut file, &mut file2).unwrap();
        }
        Err(_) => panic!("File not found in zip"),
    };
    Ok(webdriver_path.to_str().unwrap().to_string())
}

#[derive(Debug, Serialize)]
struct Output {
    tauri_driver_binary: String,
    webdriver_binary: String,
}

impl std::process::Termination for Output {
    fn report(self) -> ExitCode {
        println!("{}", serde_json::to_string(&self).unwrap());
        ExitCode::SUCCESS
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<Output> {
    let version = get_microsoft_edge_version()?;
    let edge_path = download_microsoft_edge_driver(&version)?;
    let driver_path = download_tauri_driver()?;
    Ok(Output {
        tauri_driver_binary: driver_path,
        webdriver_binary: edge_path,
    })
}

#[cfg(target_os = "linux")]
fn check_webkit2gtk_driver() -> Result<String> {
    let webdriver = process::Command::new("WebKitWebDriver");

    if webdriver.get_program().is_empty() {
        return Err(anyhow!(
            "webkit2gtk-driver not found: sudo apt install webkit2gtk-driver"
        ));
    }

    Ok(which::which("WebKitWebDriver")?
        .to_str()
        .context("WebKitWebDriver not found")?
        .to_string())
}

#[cfg(target_os = "linux")]
fn main() -> Result<Output> {
    let driver_path = download_tauri_driver()?;
    let webkit2gtk_driver = check_webkit2gtk_driver()?;
    Ok(Output {
        tauri_driver_binary: driver_path,
        webdriver_binary: webkit2gtk_driver,
    })
}

#[allow(dead_code)]
fn run_command(bin_path: &str, args: Vec<String>) -> Result<()> {
    let spawn = process::Command::new(bin_path)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .args(&args)
        .spawn();

    if let Ok(mut spawn) = spawn {
        let status = spawn
            .wait()?
            .code()
            .ok_or_else(|| anyhow!("Failed to get spawn exit code"))?;

        process::exit(status);
    }

    Ok(())
}
