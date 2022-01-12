extern crate dirs;

use super::super::elements::{FirstName, PreName};
use super::super::ui::SvnUpgrader;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;
use url::Url;

const JENKINS_TOML_FILENAME: &str = "jenkins.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JenkinsInfo {
    jenkins_domain: String,
    username: String,
    token: String,
    is_https: bool,
    path: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JenkinsToml {
    first_name: String,
    pre_name: String,
    hyphen: String,
    readme: String,
    mailto: String,
    proid: String,
}

impl From<&mut SvnUpgrader> for JenkinsToml {
    fn from(svn_upgrader: &mut SvnUpgrader) -> Self {
        let readme = get_readme(&PathBuf::from(&svn_upgrader.repo_path));
        JenkinsToml {
            first_name: svn_upgrader.first_name.clone(),
            pre_name: svn_upgrader.pre_name.clone(),
            hyphen: "_".to_string(),
            mailto: svn_upgrader.mailto.clone(),
            proid: svn_upgrader.pro_id.clone(),
            readme: readme,
        }
    }
}

impl JenkinsToml {
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }
    pub fn mailto(&self) -> String {
        self.mailto.clone()
    }
    pub fn proid(&self) -> String {
        self.proid.clone()
    }
    pub fn readme(&self) -> String {
        self.readme.clone()
    }
    pub fn hyphen(&self) -> String {
        self.hyphen.clone()
    }
    pub fn pre_name(&self) -> String {
        self.pre_name.clone()
    }
    pub fn first_name_selected(&self) -> Option<FirstName> {
        match self.first_name.as_str() {
            name if name.contains("LAS") => Some(FirstName::LAS),
            name if name.contains("BVT") => Some(FirstName::BVT),
            name if name.contains("SAS") => Some(FirstName::BDSEC),
            name if name.contains("CSV") => Some(FirstName::CSV),
            name if name.contains("BDSEC") => Some(FirstName::BDSEC),
            name if name.contains("NFA") => Some(FirstName::NFA),
            name if name.contains("BDLOG") => Some(FirstName::BDLOG),
            _ => None,
        }
    }
    pub fn pre_name_selected(&self) -> Option<PreName> {
        match self.pre_name.as_str() {
            name if name.contains("SP") => Some(PreName::SP),
            name if name.contains("KB") => Some(PreName::KB),
            _ => None,
        }
    }
}

pub struct JenkinsInput {
    pub url: String,
    pub username: String,
    pub token: String,
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

pub fn get_jenkins_config() -> Option<JenkinsInfo> {
    let mut jk_info = JenkinsInfo::default();
    config_dir().and_then(|conf_dir| {
        let jk_info_toml = conf_dir.join("jenkins_info").join("jenkins_info.toml");
        if jk_info_toml.exists() {
            jk_info = read_config_from_file(&jk_info_toml);
            Some(jk_info)
        } else {
            None
        }
    })
}

pub fn dump_jenkins_toml(
    repo_path: &PathBuf,
    jenkins_toml: &JenkinsToml,
) -> Result<io::Result<()>, toml::ser::Error> {
    let repo_jenkins_toml = repo_path.join(JENKINS_TOML_FILENAME);
    toml::to_string(&jenkins_toml)
        .and_then(|toml_string| Ok(fs::write(repo_jenkins_toml, &toml_string)))
}

pub fn get_jenkins_toml(repo_path: &PathBuf) -> io::Result<JenkinsToml> {
    let mut toml_str = String::new();
    let repo_jenkins_toml = repo_path.join(JENKINS_TOML_FILENAME);
    File::open(repo_jenkins_toml).and_then(|mut file| {
        file.read_to_string(&mut toml_str)
            .map_err(|_err| panic!("Failed to read file content."))
            .unwrap();
        Ok(toml::from_str(&toml_str).unwrap())
    })
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

pub fn create_default_config_file(ui_input: &JenkinsInput) -> Option<JenkinsInfo> {
    config_dir().and_then(|conf_dir| {
        let jk_info_toml = conf_dir.join("jenkins_info").join("jenkins_info.toml");
        Some(create_config_to_file(&jk_info_toml, ui_input))
    })
}

fn create_config_to_file(toml_conf: &PathBuf, ui_input: &JenkinsInput) -> JenkinsInfo {
    println!("It's seems that your first time to run this program. Please input the necessary information.");
    let jenkins_url = Url::parse(&ui_input.url).unwrap();
    let username = ui_input.username.clone();
    let token = ui_input.token.clone();
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

fn get_readme(path: &PathBuf) -> String {
    let mut readme = String::from("readme");
    if path.is_dir() {
        let items = fs::read_dir(path).and_then(|e| {
            Ok(e.map(|res| {
                res.and_then(|entry| {
                    let entry_path = entry.path();
                    Ok((entry_path.clone(), entry_path.file_name().map(|filename| {
                        let lowcase_filename =
                            filename.to_string_lossy().to_string().to_lowercase();
                        lowcase_filename.eq("readme")
                            || lowcase_filename.eq("readme.txt")
                            || lowcase_filename.eq("readme.md")
                    })))
                })
            }))
        }).unwrap();
        for item in items {
            match item {
                Ok((path, Some(is_readme))) => {
                    if is_readme {
                        readme = fs::read_to_string(path).unwrap();
                    }
                }
                _ => (),
            }
        }
    }
    readme
}
