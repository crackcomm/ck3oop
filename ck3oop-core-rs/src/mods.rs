use std::path::PathBuf;

pub struct ModLoadOrder {
    pub mod_list: ModList,
}

impl Default for ModLoadOrder {
    fn default() -> Self {
        ModLoadOrder::new()
    }
}

impl ModLoadOrder {
    pub fn new() -> ModLoadOrder {
        ModLoadOrder {
            mod_list: ModList::new(),
        }
    }
    pub fn change_order_by_index(&mut self, from: usize, to: usize) {
        if from >= self.mod_list.mods.len() || to >= self.mod_list.mods.len() {
            return;
        }
        self.mod_list.mods.swap(from, to);
    }

    pub fn change_order_by_name(&mut self, from: &str, to: &str) {
        let from_index = self.mod_list.mods.iter().position(|x| x.name == from);
        let to_index = self.mod_list.mods.iter().position(|x| x.name == to);
        if from_index.is_none() || to_index.is_none() {
            return;
        }
        self.change_order_by_index(from_index.unwrap(), to_index.unwrap());
    }
}

pub struct ModList {
    pub mods: Vec<Mod>,
}

impl ModList {
    pub fn new() -> ModList {
        ModList { mods: Vec::new() }
    }

    pub fn add_mod(&mut self, mod_: Mod) {
        self.mods.push(mod_);
    }

    pub fn add_mod_from_path(&mut self, path: &PathBuf) -> std::io::Result<()> {
        let mod_ = Mod::new_from_path(path)?;
        self.add_mod(mod_);
        Ok(())
    }

    pub fn add_mods_from_dir(&mut self, dir: &PathBuf) -> std::io::Result<()> {
        let mut entries = std::fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        entries.sort();

        for entry in entries {
            let as_path = entry.as_path();
            if as_path.is_file() {
                self.add_mod_from_path(&entry.to_path_buf())?;
            }
        }
        Ok(())
    }
}

impl Default for ModList {
    fn default() -> Self {
        ModList::new()
    }
}

pub struct Mod {
    pub name: String,
    pub path: String,
    pub version: String,
    pub remote_file_id: String,
    pub supported_version: String,
}

impl Default for Mod {
    fn default() -> Self {
        Mod::new(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        )
    }
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
