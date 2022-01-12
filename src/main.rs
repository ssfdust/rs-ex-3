#![windows_subsystem = "windows"]
mod elements;
mod core;
mod ui;
use iced::{Application, Settings};
use ui::{SvnUpgrader, UserInfo};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    match core::get_jenkins_config() {
        Some(_) => {
            settings.window.size = (450, 330);
            SvnUpgrader::run(settings)
        },
        None => {
            settings.window.size = (400, 240);
            UserInfo::run(settings)
        }
    }
}
