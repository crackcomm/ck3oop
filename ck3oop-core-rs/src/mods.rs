use std::path::PathBuf;

pub fn mod_from_file_content(content: &str) -> std::io::Result<Mod> {
    let mut name = String::from("");
    let mut path = String::from("");
    let mut version = String::from("");
    let mut supported_version = String::from("");
    let mut remote_file_id = String::from("");

    for line in content.lines() {
        let mut parts = line.split('=');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").trim_matches('"');

        match key {
            "name" => name = String::from(value),
            "path" => path = String::from(value),
            "version" => version = String::from(value),
            "supported_version" => supported_version = String::from(value),
            "remote_file_id" => remote_file_id = String::from(value),
            _ => (),
        }
    }

    Ok(Mod::new(
        name,
        path,
        version,
        supported_version,
        remote_file_id,
    ))
}

pub struct Mod {
    pub name: String,
    pub path: String,
    pub version: String,
    pub supported_version: String,
    pub remote_file_id: String,
}

impl Mod {
    pub fn new_from_path(path: &PathBuf) -> std::io::Result<Mod> {
        let content = std::fs::read_to_string(path)?;
        Mod::new_from_file_content(&content)
    }

    fn new_from_file_content(content: &str) -> std::io::Result<Mod> {
        mod_from_file_content(content)
    }

    fn new(
        name: String,
        path: String,
        version: String,
        supported_version: String,
        remote_file_id: String,
    ) -> Mod {
        Mod {
            name,
            path,
            version,
            supported_version,
            remote_file_id,
        }
    }
}
