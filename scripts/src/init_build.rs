use anyhow::Result;
use ck3oop_scripts::build;
fn main() -> Result<()> {
    let dir = build::get_full_path_to_build_dir()?;
    if !dir.exists() {
        std::fs::create_dir_all(dir.clone())?;
        return Ok(println!(
            "Created build directory at: {:?}",
            dir.clone().into_os_string()
        ));
    }
    let file = build::get_full_path_to_build_file()?;
    if !file.exists() {
        let build = build::Build::default();
        let build_content = serde_json::to_string_pretty(&build)?;
        std::fs::write(file.clone(), build_content)?;
        return Ok(println!(
            "Initialized build file at: {:?}",
            dir.clone().into_os_string()
        ));
    }
    Ok(println!("Build directory and file already exist"))
}
