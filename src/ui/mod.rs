pub use self::svnupgrader::SvnUpgrader;
pub use self::userinfo::UserInfo;
pub mod svnupgrader;
pub mod userinfo;
pub const INPUT_LENGTH: u16 = 248;
pub const LABEL_FONT_SIZE: u16 = 18;
pub const LABEL_WIDTH: u16 = 80;
pub const INPUT_PADDING: u16 = 3;
