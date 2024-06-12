// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{HashMap};
use std::path::{Path};
use std::time::{Duration, Instant};
use jwalk::{WalkDir};
use serde::Serialize;
use regex::Regex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn measure_execution_time<F, R>(function: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
{
    let start_time = Instant::now();
    let result = function();
    let end_time = Instant::now();
    let duration = end_time - start_time;
    (result, duration)
}

#[tauri::command(async)]
fn recursive_walk(path: String) -> FileTree {
    let (result, duration) = measure_execution_time(|| _build_file_tree(path));
    println!("Execution time: {:?}", duration);
    result
}


#[derive(Serialize, Clone)]
struct FileNode {
    path: String,
    name: String,
    is_dir: bool,
    children: FileTree,
}

type FileTree = HashMap<String, FileNode>;

fn traverse_tree_by_keys<'a>(tree: &'a mut FileTree, keys: &[String]) -> Option<(&'a mut FileNode, usize)> {
    // Get a mutable reference to the root node
    let mut current_node = tree.get_mut(&keys[0])?;

    // Index variable to keep track of the index
    let mut index = 0;

    for (i, key) in keys[1..].iter().enumerate() {
        // Attempt to get the mutable child node for the current key

        match current_node.children.get_mut(key) {
            Some(_) => {
                // If child node found, update current_node and index]
                current_node = current_node.children.get_mut(key).unwrap();
                index = i + 1; // Increment index by 1 to account for skipping the first key
            }
            None => {
                // If child node not found, return the last successfully found node and its index
                return Some((current_node, index));
            }
        }
    }

    // Return the last node found and its index
    Some((current_node, index))
}

fn _build_file_tree(path: String) -> FileTree {
    let mut file_tree: FileTree = HashMap::new();

    let root_path = Path::new(path.as_str());
    let root_last_component_index = root_path.components().count() - 1;
    let root_last_component = root_path.components().nth(root_last_component_index).unwrap();
    let root_name = root_last_component.as_os_str().to_str().unwrap().to_string();

    let root_file_node = FileNode {
        path: root_path.display().to_string(),
        is_dir: true,
        name: root_name.clone(),
        children: HashMap::new(),
    };

    file_tree.insert(root_name, root_file_node);
    // get count of components of the root path
    // including the last one so we know the tree starting point index
    // if this is windows the first one will be disk prefix so we can probably merge

    for (index, entry) in WalkDir::new(path).sort(true).into_iter().enumerate() {
        match entry {
            Ok(entry) => {

                // skip root
                if index == 0 {
                    continue;
                }

                // get the path
                let path = entry.path();

                // build components starting from the root index
                let components: Vec<String> = path.components()
                    .skip(root_last_component_index)
                    .map(|c| c.as_os_str().to_str().unwrap().to_string()).collect();

                match traverse_tree_by_keys(&mut file_tree, &components) {
                    Some((node, index)) => {
                        let name = &components[index + 1];

                        node.children.insert(name.clone(), FileNode {
                            path: path.display().to_string(),
                            is_dir: entry.file_type().is_dir(),
                            name: name.clone(),
                            children: HashMap::new(),
                        });
                    }
                    None => {
                        eprintln!("Error: Could not find parent node for path: {:?}", path);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    file_tree
}

fn resolve_mod_path(mods_dir: String, mod_path: String) -> String {

    // if path is absolute, normalize it
    if Path::new(&mod_path).is_absolute() {

        // naive normalization
        Path::new(&mod_path).display().to_string();
    }

    // if path is relative, resolve it
    Path::new(&mods_dir).join(&mod_path).display().to_string()
}

#[test]
fn test_resolve_mod_dir() {
    let mods_dir = "C:\\Users\\Some\\Documents\\Paradox Interactive\\Crusader Kings III\\mod";
    let mod_path = "mod1";
    let result = resolve_mod_path(mods_dir.to_string(), mod_path.to_string());
    assert_eq!(result, "C:\\Users\\Some\\Documents\\Paradox Interactive\\Crusader Kings III\\mod\\mod1");

    let mods_dir = "C:\\Users\\Some\\Documents\\Paradox Interactive\\Crusader Kings III\\mod";
    let mod_path = "C:\\Users\\Some\\Documents\\Paradox Interactive\\Crusader Kings III\\mod\\mod2";
    let result = resolve_mod_path(mods_dir.to_string(), mod_path.to_string());
    assert_eq!(result, "C:\\Users\\Some\\Documents\\Paradox Interactive\\Crusader Kings III\\mod\\mod2");
}

// define Mod structure
#[derive(Serialize)]
struct Mod {
    file_name: String,
    file_path: String,

    version: String,
    name: String,
    picture: String,
    supported_version: String,
    path: String,
    remote_file_id: String,
    archive: String,
}

impl Mod {
    fn new(
        mods_dir_path: String,  // C:\Users\User\Documents\Paradox Interactive\Crusader Kings III\mod
        mod_filename: String,  // test.mod
    ) -> Mod {
        // build file path
        let file_path = Path::new(&mods_dir_path.clone()).join(mod_filename.clone());
        // read mod file
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        // resolve mod path
        Mod {
            file_name: mod_filename,
            file_path: file_path.display().to_string(),
            name: extract_value_from_modfile("name", &file_content).unwrap_or("".to_string()),
            path: extract_value_from_modfile("path", &file_content).unwrap_or("".to_string()),
            picture: extract_value_from_modfile("picture", &file_content).unwrap_or("".to_string()),
            version: extract_value_from_modfile("version", &file_content).unwrap_or("".to_string()),
            remote_file_id: extract_value_from_modfile("remote_file_id", &file_content).unwrap_or("".to_string()),
            supported_version: extract_value_from_modfile("supported_version", &file_content).unwrap_or("".to_string()),
            archive: extract_value_from_modfile("archive", &file_content).unwrap_or("".to_string()),
        }
    }
}

#[test]
fn test_mod_construtor() {
    let current_dir_path = std::env::current_dir().unwrap();
    let mods_path = current_dir_path.join("test\\paradocdir\\mod");
    let mod_file_name = "test.mod";

    let my_mod = Mod::new(
        mods_path.display().to_string(),
        mod_file_name.to_string(),
    );

    assert_eq!(my_mod.file_name, "test.mod");
    assert_eq!(my_mod.file_path, Path::new(&mods_path).join(mod_file_name).display().to_string());
    assert_eq!(my_mod.name, "test");
    assert_eq!(my_mod.path, "C:/Users/User/Documents/Paradox Interactive/Crusader Kings III/mod/test");
    assert_eq!(my_mod.picture, "");
    assert_eq!(my_mod.version, "1.0.0");
    assert_eq!(my_mod.remote_file_id, "");
    assert_eq!(my_mod.supported_version, "1.12.5");
}

fn extract_value_from_modfile(key: &str, text: &str) -> Option<String> {
    let pattern = format!(r#"\b{}\s*=\s*("[^"]*"|\{{([^}}]*)\}}|\[([^\]]*)\]|(\S[^"]*))"#, key);
    let re = Regex::new(&pattern).unwrap();

    if let Some(caps) = re.captures(text) {
        for i in 1..=4 {
            if let Some(value) = caps.get(i) {
                return Some(value.as_str().trim_matches('"').to_string());
            }
        }
    }
    None
}

#[test]
fn test_extract_value_from_modfile() {
    let text =
        r#"
        version="1.0.0"
        tags={
            "Alternative History"
        }
        name="test"
        supported_version="1.12.5"
        path="C:/Users/buk/Documents/Paradox Interactive/Crusader Kings III/mod/test"#;

    assert_eq!(extract_value_from_modfile("version", text).unwrap(), "1.0.0");
    assert_eq!(extract_value_from_modfile("name", text).unwrap(), "test");
    assert_eq!(extract_value_from_modfile("supported_version", text).unwrap(), "1.12.5");
    assert_eq!(extract_value_from_modfile("path", text).unwrap(), "C:/Users/buk/Documents/Paradox Interactive/Crusader Kings III/mod/test");
}

fn build_mod_list(mods_path: String) -> Vec<Mod> {
    let mods_dir = Path::new(&mods_path);

    let mut mods: Vec<Mod> = Vec::new();

    // iterate mods path looking for .mod files
    for entry in std::fs::read_dir(mods_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        // we don't care about directories
        if path.is_dir() {
            continue;
        }

        if file_name.ends_with(".mod") {
            let new_mod = Mod::new(mods_path.clone(), file_name);

            // if mod has no path, skip it
            // probably archived paradox mod, we dont care
            if new_mod.path.is_empty() {
                continue;
            }
            
            mods.push(new_mod);
        }
    }
    mods
}

#[tauri::command(async)]
fn get_mod_list(path: String) -> Vec<Mod> {
    let (result, duration) = measure_execution_time(|| build_mod_list(path));
    println!("Execution time : {:?}", duration);
    result
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, recursive_walk, get_mod_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_walk() {
        let result = recursive_walk(r"C:\Program Files (x86)\Steam\steamapps\common\Crusader Kings III\game".to_string());
        println!("{:?}", result.len());
    }
}