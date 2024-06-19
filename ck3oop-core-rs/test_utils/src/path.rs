use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[track_caller]
pub fn path_to_this_file() -> PathBuf {
    let root_dir: OsString;
    // if there's CARGO_WORKSPACE_DIR we are working in a workspace
    if let Some(workspace_dir) = std::env::var_os("CARGO_WORKSPACE_DIR") {
        println!("Using CARGO_WORKSPACE_DIR");
        root_dir = workspace_dir;
    } else {
        println!("Using CARGO_MANIFEST_DIR");
        root_dir = std::env::var_os("CARGO_MANIFEST_DIR").to_owned().unwrap();
    }
    println!("ROOT DIR: {:?}", root_dir);

    let caller_file = std::panic::Location::caller().file();
    println!("CALLER FILE: {:?}", caller_file);

    let root_dir_as_path = Path::new(root_dir.as_os_str());
    let this_file = root_dir_as_path.join(caller_file);
    println!("RESULT: {:?}", this_file);
    this_file
}

#[track_caller]
pub fn dir_of_this_file() -> PathBuf {
    let path = path_to_this_file();
    path.parent().unwrap().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path_to_this_file() {
        let path = path_to_this_file();
        println!("PATH: {:?}", path);
        let components = path
            .components()
            .map(|c| c.as_os_str().to_str().unwrap())
            .collect::<Vec<&str>>();

        let len = components.len();
        assert_eq!(components[len - 1], "path.rs");
        assert_eq!(components[len - 2], "src");
        assert_eq!(components[len - 3], "test_utils");
    }

    #[test]
    fn test_dir_of_this_file() {
        let path = dir_of_this_file();
        println!("PATH: {:?}", path);
        let components = path
            .components()
            .map(|c| c.as_os_str().to_str().unwrap())
            .collect::<Vec<&str>>();

        let len = components.len();
        assert_eq!(components[len - 1], "src");
        assert_eq!(components[len - 2], "test_utils");
    }
}
