// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::path::{Component, Path};
use std::time::{Duration, Instant};
use jwalk::{DirEntry, WalkDir};
use serde::Serialize;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
    let (result, duration) = measure_execution_time(|| _recursive_walk(path));
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


fn _recursive_walk(path: String) -> FileTree {


    let mut fileTree: FileTree = HashMap::new();

    let rootPath = Path::new(path.as_str());
    let rootLastComponentIndex = rootPath.components().count() - 1;
    let rootLastComponent = rootPath.components().nth(rootLastComponentIndex).unwrap();
    let rootName = (rootLastComponent.as_os_str().to_str().unwrap().to_string());

    let rootFileNode = FileNode {
        path: rootPath.display().to_string(),
        is_dir: true,
        name: rootName.clone(),
        children: HashMap::new(),
    };

    fileTree.insert(rootName, rootFileNode);
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
                    .skip(rootLastComponentIndex)
                    .map(|c| c.as_os_str().to_str().unwrap().to_string()).collect();


                let totalComponents = components.len();

                match traverse_tree_by_keys(&mut fileTree, &components) {
                    Some((node, index)) => {
                        let name = &components[index+1];

                        node.children.insert(name.clone(), FileNode {
                            path: path.display().to_string(),
                            is_dir: entry.file_type().is_dir(),
                            name: name.clone(),
                            children: HashMap::new(),
                        });
                    },
                    None => {
                        eprintln!("Error: Could not find parent node for path: {:?}", path);
                    }
                }

            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    fileTree
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_extra::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![recursive_walk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::*;

    #[test]
    fn test_recursive_walk() {
        let mut result = recursive_walk(r"C:\Program Files (x86)\Steam\steamapps\common\Crusader Kings III\game".to_string());
        println!("{:?}", result.len());
    }
}