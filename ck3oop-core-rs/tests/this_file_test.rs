extern crate test_utils;

// make sure that `path_to_this_file` returns the correct path
#[test]
fn test_this_file() {
    let this_file_path = test_utils::path::path_to_this_file();
    let components = this_file_path
        .components()
        .map(|c| c.as_os_str().to_str().unwrap())
        .collect::<Vec<&str>>();
    let len = components.len();
    assert_eq!(components[len - 1], "this_file_test.rs");
    assert_eq!(components[len - 2], "tests");
    assert_eq!(components[len - 3], "ck3oop-core-rs");
}

// make sure that `dir_of_this_file` returns the correct path
#[test]
fn test_dir_of_this_file() {
    let dir_of_this_file_path = test_utils::path::dir_of_this_file();
    let components = dir_of_this_file_path
        .components()
        .map(|c| c.as_os_str().to_str().unwrap())
        .collect::<Vec<&str>>();
    let len = components.len();
    assert_eq!(components[len - 1], "tests");
    assert_eq!(components[len - 2], "ck3oop-core-rs");
}
