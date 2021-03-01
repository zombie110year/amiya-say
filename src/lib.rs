mod dialog;

use std::{
    fs::{read_dir, read_to_string},
    io::{Error, ErrorKind},
    path::PathBuf,
};

use dirs;

pub fn build(text: &str, name: &str) -> String {
    let dialog_box = dialog::dialog(text.to_string()).unwrap();
    let role = find_role(name);
    return format!("{}{}", dialog_box, role);
}

fn find_role_dir() -> PathBuf {
    let role_dir1 = std::env::current_dir().unwrap().join("resources/roles");
    let role_dir2 = dirs::data_dir().unwrap().join("amiya-say/roles");
    if role_dir1
        .metadata()
        .and_then(|x| {
            if x.is_dir() {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::InvalidData, "not a directory"))
            }
        })
        .is_ok()
    {
        role_dir1
    } else if role_dir2
        .metadata()
        .and_then(|x| {
            if x.is_dir() {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::InvalidData, "not a directory"))
            }
        })
        .is_ok()
    {
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

pub fn list_roles() -> Vec<String> {
    let role_dir = find_role_dir();
    let roles: Vec<String> = read_dir(role_dir)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| x.path().file_stem().unwrap().to_str().unwrap().to_string())
        .collect();
    roles
}
