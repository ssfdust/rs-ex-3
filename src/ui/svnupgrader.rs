use super::super::elements::{FirstName, PackType};
use super::{INPUT_LENGTH, LABEL_FONT_SIZE, LABEL_WIDTH};
use iced::{
    button, pick_list, text_input, Align, Button, Column, Container, Element, Length, PickList,
    Row, Sandbox, Text,
};

#[derive(Default)]
pub struct SvnUpgrader {
    repo_path: String,
    repo_path_input: text_input::State,
    pro_id: String,
    pro_id_input: text_input::State,
    mailto: String,
    mailto_input: text_input::State,
    first_name: String,
    first_name_selected: Option<FirstName>,
    first_name_pick: pick_list::State<FirstName>,
    pack_type: String,
    pack_type_selected: Option<PackType>,
    pack_type_pick: pick_list::State<PackType>,
    submit: button::State,
}

#[derive(Debug, Clone)]
pub enum SvnUpgraderMessage {
    RepoPathChange(String),
    ProidChange(String),
    MailToChange(String),
    FirstNameSelected(FirstName),
    PackTypeSelected(PackType),
    SubmitPressed,
}

impl Sandbox for SvnUpgrader {
    type Message = SvnUpgraderMessage;

    fn new() -> Self {
        let mut preset_default = Self::default();
        preset_default.first_name_selected = Some(FirstName::LAS);
        preset_default.pack_type_selected = Some(PackType::SP);
        preset_default
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
            SvnUpgraderMessage::PackTypeSelected(pack_type) => {
                let pack_type_val = match pack_type {
                    PackType::KB => "KB",
                    PackType::SP => "SP",
                };
                self.pack_type_selected = Some(pack_type);
                self.pack_type = pack_type_val.to_owned()
            }
            SvnUpgraderMessage::SubmitPressed => {}
        }
    }

    fn view(&mut self) -> Element<SvnUpgraderMessage> {
        let content = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .padding(10)
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
                        .width(Length::from(INPUT_LENGTH)),
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
                        Text::new("Pack Type:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        PickList::new(
                            &mut self.pack_type_pick,
                            &PackType::ALL[..],
                            self.pack_type_selected,
                            SvnUpgraderMessage::PackTypeSelected,
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
