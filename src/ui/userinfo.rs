use super::{INPUT_LENGTH, LABEL_FONT_SIZE, LABEL_WIDTH};
use iced::{
    button, text_input, Align, Button, Column, Container, Element, Length, Row, Sandbox, Text,
};

#[derive(Default)]
pub struct UserInfo {
    username: String,
    token: String,
    username_input: text_input::State,
    token_input: text_input::State,
    submit: button::State,
}

#[derive(Debug, Clone)]
pub enum UserInfoMessage {
    UserNameChange(String),
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
            UserInfoMessage::SubmitPressed => {}
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
