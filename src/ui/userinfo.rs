use super::super::core::{create_default_config_file, JenkinsInput};
use super::{INPUT_LENGTH, INPUT_PADDING, LABEL_FONT_SIZE, LABEL_WIDTH};
use iced::{
    button, text_input, Align, Button, Column, Container, Element, Length, Row, Application, Text, self
};
use native_dialog::{MessageDialog, MessageType};
use std::process;

#[derive(Default)]
pub struct UserInfo {
    url: String,
    username: String,
    token: String,
    url_input: text_input::State,
    username_input: text_input::State,
    token_input: text_input::State,
    submit: button::State,
}

#[derive(Debug, Clone)]
pub enum UserInfoMessage {
    UserNameChange(String),
    UrlChange(String),
    UserTokenChange(String),
    SubmitPressed,
}

impl Application for UserInfo {
    type Message = UserInfoMessage;
    type Flags = ();
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (UserInfo, iced::Command<UserInfoMessage>) {
        (UserInfo {
            url: "http://jenkins.juminfo.org/job/tool-make-upgrade".to_string(),
            ..Self::default()
        }, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Upgrade Helper - Set User Info For First Time. Need restart then.")
    }

    fn update(&mut self, message: UserInfoMessage, _clipboard: &mut iced::Clipboard) -> iced::Command<Self::Message> {
        match message {
            UserInfoMessage::UserTokenChange(token) => self.token = token,
            UserInfoMessage::UserNameChange(username) => self.username = username,
            UserInfoMessage::UrlChange(url) => self.url = url,
            UserInfoMessage::SubmitPressed => {
                let jenkins_input = JenkinsInput {
                    url: self.url.clone(),
                    username: self.username.clone(),
                    token: self.token.clone(),
                };
                if jenkins_input.url.len() == 0
                    || jenkins_input.username.len() == 0
                    || jenkins_input.token.len() == 0
                {
                    MessageDialog::new()
                        .set_type(MessageType::Warning)
                        .set_title("warning")
                        .set_text("There are empty fileds.")
                        .show_alert()
                        .unwrap();
                } else {
                    create_default_config_file(&jenkins_input);
                    MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Info")
                        .set_text("You need to restart the program.")
                        .show_alert()
                        .unwrap();
                    process::exit(0);
                }
            }
        };
        iced::Command::none()
    }

    fn view(&mut self) -> Element<UserInfoMessage> {
        let content = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .padding(10)
                    .align_items(Align::Center)
                    .push(
                        Text::new("Jenkins Url:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.url_input,
                            ".",
                            &self.url,
                            UserInfoMessage::UrlChange,
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
                        Text::new("Username:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.username_input,
                            ".",
                            &self.username,
                            UserInfoMessage::UserNameChange,
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
                        Text::new("Token:")
                            .width(Length::from(LABEL_WIDTH))
                            .size(LABEL_FONT_SIZE),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.token_input,
                            "",
                            &self.token,
                            UserInfoMessage::UserTokenChange,
                        )
                        .padding(INPUT_PADDING)
                        .width(Length::from(INPUT_LENGTH)),
                    ),
            )
            .push(
                Button::new(&mut self.submit, Text::new("Ok"))
                    .on_press(UserInfoMessage::SubmitPressed),
            );
        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}
