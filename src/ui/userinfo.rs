use super::super::core::{create_default_config_file, JenkinsInput};
use super::{INPUT_LENGTH, LABEL_FONT_SIZE, LABEL_WIDTH};
use iced::{
    button, text_input, Align, Button, Column, Container, Element, Length, Row, Sandbox, Text,
};
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

impl Sandbox for UserInfo {
    type Message = UserInfoMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Upgrade Helper - Set User Info For First Time. Need restart then.")
    }

    fn update(&mut self, message: UserInfoMessage) {
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
                create_default_config_file(&jenkins_input);
                process::exit(0)
            }
        }
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
                        .width(Length::from(INPUT_LENGTH)),
                    ),
            )
            .push(
                Row::new().padding(10).align_items(Align::Center).push(
                    Button::new(&mut self.submit, Text::new("Ok"))
                        .on_press(UserInfoMessage::SubmitPressed),
                ),
            );
        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}
