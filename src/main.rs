mod elements;
mod ui;
use iced::{Sandbox, Settings};
use ui::{SvnUpgrader, UserInfo};

pub fn main() -> iced::Result {
    let a = 4;
    let mut settings = Settings::default();
    if a == 4 {
        settings.window.size = (350, 180);
        UserInfo::run(settings)
    } else {
        settings.window.size = (400, 280);
        SvnUpgrader::run(settings)
    }
}
