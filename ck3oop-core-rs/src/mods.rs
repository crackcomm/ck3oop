use std::path::PathBuf;

#[derive(Default)]
pub struct ModLoadOrder {
    pub mod_list: ModList,
}

impl ModLoadOrder {
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

#[derive(Default)]
pub struct ModList {
    pub mods: Vec<Mod>,
}

impl ModList {
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

#[derive(Default)]
pub struct Mod {
    pub name: String,
    pub path: String,
    pub version: String,
    pub remote_file_id: String,
    pub supported_version: String,
}

impl Mod {
    pub fn new_from_path(path: &PathBuf) -> std::io::Result<Mod> {
        let content = std::fs::read_to_string(path)?;
        Ok(Mod::new_from_file_content(&content))
    }

    fn new_from_file_content(content: &str) -> Mod {
        mod_from_file_content(content)
    }
}
pub fn mod_from_file_content(content: &str) -> Mod {
    let mut mod_ = Mod::default();

    for line in content.lines() {
        let mut parts = line.split('=');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").trim_matches('"');

        match key {
            "name" => value.clone_into(&mut mod_.name),
            "path" => value.clone_into(&mut mod_.path),
            "version" => value.clone_into(&mut mod_.version),
            "supported_version" => value.clone_into(&mut mod_.supported_version),
            "remote_file_id" => value.clone_into(&mut mod_.remote_file_id),
            _ => (),
        }
    }

    mod_
}
