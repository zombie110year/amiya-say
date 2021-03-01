mod dialog;

use std::{fs::read_to_string, path::PathBuf};

use dirs;

pub fn build(text: &str, name: &str) -> String {
    let dialog_box = dialog::dialog(text.to_string()).unwrap();
    let role = find_role(name);
    return format!("{}{}", dialog_box, role);
}

fn find_role_dir() -> PathBuf {
    let role_dir1 = std::env::current_dir().unwrap().join("resources/roles");
    let role_dir2 = dirs::data_dir().unwrap().join("amiya-say/roles");
    if role_dir1.metadata().unwrap().is_dir() {
        role_dir1
    } else if role_dir2.metadata().unwrap().is_dir() {
        role_dir2
    } else {
        panic!("找不到资源目录 amiya-say/roles");
    }
}

fn find_role(name: &str) -> String {
    let role_dir = find_role_dir();
    let role_path = role_dir.join(format!("{}.txt", &name));
    let role = read_to_string(role_path).unwrap();
    return role;
}
