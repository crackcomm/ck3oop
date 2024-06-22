use anyhow::{anyhow, Result};
use cargo_run_bin::{binary, metadata};
use reqwest::blocking::get;
use std::path::Path;
use std::{io, process};
use winreg::enums::*;
use winreg::RegKey;

fn download_tauri_driver() -> Result<()> {
    let binary_package = metadata::get_binary_packages()?
        .iter()
        .find(|e| e.package == "tauri-driver")
        .unwrap()
        .to_owned();
    binary::install(binary_package)?;
    Ok(())
}

const REG_PATH: &str =
    r#"SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}"#;
fn get_microsoft_edge_version() -> io::Result<String> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(REG_PATH)?
        .get_value("pv")
}

fn download_microsoft_edge_driver(version: &str) -> Result<()> {
    let url = format!(
        "https://msedgedriver.azureedge.net/{}/edgedriver_win64.zip",
        version
    );

    let project_root = std::env::var_os("CARGO_EXTRA_BIN_DIR").unwrap();
    let project_root = Path::new(&project_root);
    let bin_path = &project_root.join(".bin");
    let webdriver_path = &bin_path.join(format!("msedgedriver-{}.exe", version));

    if Path::new(webdriver_path).exists() {
        return Ok(());
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
    Ok(())
}

#[cfg(windows)]
fn main() -> Result<()> {
    let version = get_microsoft_edge_version()?;
    download_tauri_driver()?;
    download_microsoft_edge_driver(&version)?;
    Ok(())
}

#[cfg(unix)]
fn main() {
    download_tauri_driver()?;
    // on debian distros
    // sudo apt update
    // sudo apt install webkit2gtk-driver
    println!("Please install webkit2gtk-driver on your system");
    Ok(())
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
