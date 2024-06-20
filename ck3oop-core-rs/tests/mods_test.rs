use ck3oop_core_rs::mods::mod_from_file_content;
use std::fs::canonicalize;
use std::path::Path;
use test_utils::path::dir_of_this_file;

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
    let fixture_path = "fixtures/windows/game_data/mod/_mod1.mod";
    let fixture_abs_path = Path::new(dir_of_this_file().as_path()).join(fixture_path);
    println!("{:?}", fixture_abs_path.to_str());

    let fixture_normalized_path = canonicalize(&fixture_abs_path);
    println!("{:?}", fixture_normalized_path);

    let fixture_normalized_path_str = fixture_normalized_path.as_ref().unwrap().to_str();
    println!("{:?}", fixture_normalized_path_str);

    let mod_ = ck3oop_core_rs::mods::Mod::new_from_path(&fixture_normalized_path.unwrap()).unwrap();
    assert_mod_equal_to_content(&mod_);
}

#[test]
pub fn test_add_mods_from_dir() {
    let fixture_path = "fixtures/windows/game_data/mod";
    let fixture_abs_path = Path::new(dir_of_this_file().as_path()).join(fixture_path);
    println!("{:?}", fixture_abs_path.to_str());

    let fixture_normalized_path = canonicalize(&fixture_abs_path);
    println!("{:?}", fixture_normalized_path);

    let mut mod_list = ck3oop_core_rs::mods::ModList::new();
    mod_list
        .add_mods_from_dir(&fixture_normalized_path.unwrap())
        .unwrap();
    assert_eq!(mod_list.mods.len(), 3);

    let mod_1 = mod_list.mods.get(0).unwrap();
    assert_eq!(mod_1.name, "My Awesome Mod 1");

    let mod_2 = mod_list.mods.get(1).unwrap();
    assert_eq!(mod_2.name, "My Awesome Mod 2");

    let mod_3 = mod_list.mods.get(2).unwrap();
    assert_eq!(mod_3.name, "My Awesome Mod 3");
}
