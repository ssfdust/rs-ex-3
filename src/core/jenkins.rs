extern crate dirs;

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;
use url::Url;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JenkinsInfo {
    jenkins_domain: String,
    username: String,
    token: String,
    is_https: bool,
    path: String,
}

impl JenkinsInfo {
    pub fn get_url(&self) -> String {
        if self.is_https {
            format!(
                "https://{}:{}@{}{}/buildWithParameters",
                self.username, self.token, self.jenkins_domain, self.path
            )
        } else {
            format!(
                "http://{}:{}@{}{}/buildWithParameters",
                self.username, self.token, self.jenkins_domain, self.path
            )
        }
    }
}

pub fn get_jenkins_config() -> JenkinsInfo {
    let mut jk_info = JenkinsInfo::default();
    config_dir().and_then(|conf_dir| {
        let jk_info_toml = conf_dir.join("jenkins_info").join("jenkins_info.toml");
        if jk_info_toml.exists() {
            jk_info = read_config_from_file(&jk_info_toml);
            Some(0)
        } else {
            jk_info = create_config_to_file(&jk_info_toml);
            Some(1)
        }
    });
    jk_info
}

fn read_config_from_file(toml_conf: &PathBuf) -> JenkinsInfo {
    let mut toml_str = String::new();
    let mut jenkins_info = JenkinsInfo::default();
    File::open(toml_conf)
        .and_then(|mut file| {
            file.read_to_string(&mut toml_str)
                .map_err(|_err| panic!("Failed to read file content."))
                .unwrap();
            jenkins_info = toml::from_str(&toml_str).unwrap();
            Ok(())
        })
        .unwrap();
    jenkins_info
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

fn create_config_to_file(toml_conf: &PathBuf) -> JenkinsInfo {
    println!("It's seems that your first time to run this program. Please input the necessary information.");
    let jenkins_url = Url::parse(&get_input("Please enter the jenkins url:")).unwrap();
    let username = get_input("Please enter username for jenkins server:");
    let token = get_input("Please enter token for jenkins server:");
    let jenkins_info = JenkinsInfo {
        jenkins_domain: jenkins_url.host_str().unwrap().to_string(),
        username: username,
        token: token,
        is_https: jenkins_url.scheme() == "https",
        path: jenkins_url.path().to_string(),
    };
    toml::to_string(&jenkins_info)
        .and_then(|toml_string| {
            toml_conf.parent().and_then(|toml_conf_parent| {
                if !toml_conf_parent.exists() {
                    fs::create_dir(toml_conf_parent).expect("Failed to create dir");
                }
                Some(toml_conf_parent)
            });
            fs::write(toml_conf, &toml_string).unwrap();
            Ok(toml_string)
        })
        .unwrap();
    jenkins_info
}
