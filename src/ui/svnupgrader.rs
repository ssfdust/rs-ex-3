use super::super::core::{dump_jenkins_toml, get_jenkins_toml, JenkinsToml, post_with};
use super::super::elements::{FirstName, PreName};
use super::{INPUT_LENGTH, INPUT_PADDING, LABEL_FONT_SIZE, LABEL_WIDTH};
use iced::{
    button, pick_list, text_input, Align, Button, Column, Container, Element, Length, PickList,
    Row, Sandbox, Text,
};
use std::io;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::path::PathBuf;

#[derive(Default)]
pub struct SvnUpgrader {
    pub repo_path: String,
    repo_path_input: text_input::State,
    pub pro_id: String,
    pro_id_input: text_input::State,
    pub mailto: String,
    mailto_input: text_input::State,
    pub first_name: String,
    first_name_selected: Option<FirstName>,
    first_name_pick: pick_list::State<FirstName>,
    pub pre_name: String,
    pre_name_selected: Option<PreName>,
    pre_name_pick: pick_list::State<PreName>,
    open_file_dialog: button::State,
    submit: button::State,
}

#[derive(Debug, Clone)]
pub enum SvnUpgraderMessage {
    RepoPathChange(String),
    ProidChange(String),
    MailToChange(String),
    FirstNameSelected(FirstName),
    PreNameSelected(PreName),
    OpenFileDialogPressed,
    SubmitPressed,
}

impl Sandbox for SvnUpgrader {
    type Message = SvnUpgraderMessage;

    fn new() -> Self {
        SvnUpgrader {
            first_name_selected: Some(FirstName::LAS),
            pre_name_selected: Some(PreName::SP),
            first_name: String::from("001: BDSEC"),
            pre_name: String::from("SP"),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Upgrade Helper")
    }

    fn update(&mut self, message: SvnUpgraderMessage) {
        match message {
            SvnUpgraderMessage::RepoPathChange(repo_path) => {
                self.repo_path = repo_path;
            }
            SvnUpgraderMessage::ProidChange(pro_id) => self.pro_id = pro_id,
            SvnUpgraderMessage::MailToChange(mailto) => self.mailto = mailto,
            SvnUpgraderMessage::FirstNameSelected(first_name) => {
                let first_name_val = match first_name {
                    FirstName::LAS => "LAS:  深信服LAS",
                    FirstName::BVT => "BVT: 深信服BVT",
                    FirstName::BDSEC => "001: BDSEC",
                    FirstName::SAS => "002: SAS",
                    FirstName::CSV => "003: CSV",
                    FirstName::BDLOG => "004: BDLOG",
                    FirstName::NFA => "005: NFA",
                };
                self.first_name_selected = Some(first_name);
                self.first_name = first_name_val.to_owned()
            }
            SvnUpgraderMessage::PreNameSelected(pre_name) => {
                let pre_name_val = match pre_name {
                    PreName::KB => "KB",
                    PreName::SP => "SP",
                };
                self.pre_name_selected = Some(pre_name);
                self.pre_name = pre_name_val.to_owned()
            }
            SvnUpgraderMessage::OpenFileDialogPressed => {
                let path = FileDialog::new()
                    .set_location("~/Desktop")
                    .set_filename("")
                    .show_open_single_dir()
                    .unwrap();

                let path = match path {
                    Some(path) => path,
                    None => return,
                };
                self.repo_path = path.to_string_lossy().to_string();
                match get_jenkins_toml(&path){
                    Ok(jenkins_toml) => {
                        self.pre_name = jenkins_toml.pre_name();
                        self.first_name = jenkins_toml.first_name();
                        self.pro_id = jenkins_toml.proid();
                        self.mailto = jenkins_toml.mailto();
                        self.first_name_selected = jenkins_toml.first_name_selected();
                        self.pre_name_selected = jenkins_toml.pre_name_selected();
                    }
                    _ => {}
                }
            }
            SvnUpgraderMessage::SubmitPressed => {
                let path = PathBuf::from(self.repo_path.as_str());
                let jenkins_toml = JenkinsToml::from(self);
                match dump_jenkins_toml(&path, &jenkins_toml) {
                    Err(_) => {
                        MessageDialog::new()
                        .set_title("Error")
                        .set_type(MessageType::Error)
                        .set_text("Failed to create toml file.")
                        .show_alert().or_else(|_| {Err(io::Error::last_os_error())}).unwrap();
                    }
                    _ => {
                        post_with(&path, jenkins_toml);
                        MessageDialog::new()
                        .set_title("Info")
                        .set_type(MessageType::Info)
                        .set_text("Request send.")
                        .show_alert().or_else(|_| {Err(io::Error::last_os_error())}).unwrap();
                    },
                }
            }
        }
    }

    fn view(&mut self) -> Element<SvnUpgraderMessage> {
        let content = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .padding(10)
                    .spacing(5)
                    .align_items(Align::Center)
                    .push(
                        Text::new("Repo Path:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.repo_path_input,
                            ".",
                            &self.repo_path,
                            SvnUpgraderMessage::RepoPathChange,
                        )
                        .padding(INPUT_PADDING)
                        .width(Length::from(INPUT_LENGTH - 20)),
                    )
                    .push(
                        Button::new(&mut self.open_file_dialog, Text::new(".."))
                            .on_press(SvnUpgraderMessage::OpenFileDialogPressed),
                    ),
            )
            .push(
                Row::new()
                    .padding(10)
                    .align_items(Align::Center)
                    .push(
                        Text::new("Proid:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.pro_id_input,
                            "504_Pro.20210114.003800",
                            &self.pro_id,
                            SvnUpgraderMessage::ProidChange,
                        )
                        .padding(INPUT_PADDING)
                        .width(Length::from(INPUT_LENGTH)),
                    ),
            )
            .push(
                Row::new()
                    .padding(10)
                    .align_items(Align::Center)
                    .push(
                        Text::new("Mail To:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.mailto_input,
                            "",
                            &self.mailto,
                            SvnUpgraderMessage::MailToChange,
                        )
                        .padding(INPUT_PADDING)
                        .width(Length::from(INPUT_LENGTH)),
                    ),
            )
            .push(
                Row::new()
                    .padding(10)
                    .align_items(Align::Center)
                    .push(
                        Text::new("Frist Name:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        PickList::new(
                            &mut self.first_name_pick,
                            &FirstName::ALL[..],
                            self.first_name_selected,
                            SvnUpgraderMessage::FirstNameSelected,
                        )
                        .width(Length::from(INPUT_LENGTH)),
                    ),
            )
            .push(
                Row::new()
                    .padding(10)
                    .align_items(Align::Center)
                    .push(
                        Text::new("Pre Name:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        PickList::new(
                            &mut self.pre_name_pick,
                            &PreName::ALL[..],
                            self.pre_name_selected,
                            SvnUpgraderMessage::PreNameSelected,
                        )
                        .width(Length::from(INPUT_LENGTH)),
                    ),
            )
            .push(
                Row::new().align_items(Align::End).push(
                    Button::new(&mut self.submit, Text::new("Submit"))
                        .on_press(SvnUpgraderMessage::SubmitPressed),
                ),
            );
        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}
