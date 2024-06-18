use ck3oop_core_rs::mods::mod_from_file_content;
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn get_tests_absolute_path() -> PathBuf {
    // Get the current directory
    let dir1 = current_dir().unwrap();
    let dir1 = dir1.parent().unwrap();
    // Get the file path relative to the current file
    let dir2 = Path::new(file!());
    // Get the parent directory of the file path
    let dir3 = dir2.parent().unwrap();
    // Join the current directory with the parent directory of the file path

    dir1.join(dir3)
}

// make sure path works on linux and windows
pub fn normalize_path(path: &PathBuf) -> std::io::Result<PathBuf> {
    std::fs::canonicalize(path)
}

const CONTENT: &str = r#"
version="1.0.0"
tags={
	"Alternative History"
}
name="My Awesome Mod 1"
supported_version="1.12.5"
path="C:/Users/User1/Documents/Paradox Interactive/Crusader Kings III/mod/test_mod"
remote_file_id="123456789"
    "#;

pub fn assert_mod_equal_to_content(mod_: &ck3oop_core_rs::mods::Mod) {
    assert_eq!(mod_.name, "My Awesome Mod 1");
    assert_eq!(
        mod_.path,
        "C:/Users/User1/Documents/Paradox Interactive/Crusader Kings III/mod/test_mod"
    );
    assert_eq!(mod_.version, "1.0.0");
    assert_eq!(mod_.supported_version, "1.12.5");
    assert_eq!(mod_.remote_file_id, "123456789");
}

#[test]
pub fn test_parse_mod_file_content() {
    let content = String::from(CONTENT);
    let mod_ = mod_from_file_content(content.as_str()).unwrap();
    assert_mod_equal_to_content(&mod_);
}

#[test]
pub fn test_mod_new_from_path() {
    let tests_absolute_path = get_tests_absolute_path();
    println!("{:?}", tests_absolute_path.to_str());

    let fixture_path = "fixtures/windows/game_data/mod/_mod1.mod";
    let fixture_absolute_path = tests_absolute_path.join(fixture_path);
    println!("{:?}", fixture_absolute_path.to_str());

    let fixture_normalized_path = normalize_path(&fixture_absolute_path);
    println!("{:?}", fixture_normalized_path);

    let fixture_normalized_path_str = fixture_normalized_path.as_ref().unwrap().to_str();
    println!("{:?}", fixture_normalized_path_str);

    let mod_ = ck3oop_core_rs::mods::Mod::new_from_path(&fixture_normalized_path.unwrap()).unwrap();
    assert_mod_equal_to_content(&mod_);
}
