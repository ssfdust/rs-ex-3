use std::fs;
use std::path::PathBuf;

pub fn get_sub_packages(repo_path: &PathBuf) -> Vec<String> {
    let allowed_packages = vec![
        "core_upgrade",
        "vul_upgrade",
        "evt_upgrade",
        "collector_upgrade",
        "web_upgrade",
        "scb_upgrade",
        "ivc_upgrade",
        "data_upgrade",
    ];
    let mut sub_packages: Vec<String> = Vec::new();
    for subdir in fs::read_dir(repo_path).unwrap() {
        let subdir = subdir.unwrap();
        let subdir_path = subdir.path();
        if let Some(file_name) = subdir_path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if subdir_path.is_dir() && allowed_packages.contains(&&file_name_str) {
                    sub_packages.push(file_name_str.to_owned());
                }
            }
        }
    }
    sub_packages
}
