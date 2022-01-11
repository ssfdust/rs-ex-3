mod elements;
mod core;
mod ui;
use iced::{Sandbox, Settings};
use ui::{SvnUpgrader, UserInfo};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    match core::get_jenkins_config() {
        Some(_) => {
            settings.window.size = (400, 300);
            SvnUpgrader::run(settings)
        },
        None => {
            settings.window.size = (350, 210);
            UserInfo::run(settings)
        }
    }
}
