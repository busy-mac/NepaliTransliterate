// बिजि७७<bandesh@gmail.com>

use iced::widget::{button, container, text, text_editor, Column, Row};
use iced::theme::Theme;
use iced::{Alignment, Element, Length, Sandbox, Settings, Size};
use iced::widget::text_editor::{Content, Action};
use NepaliTransliterate::NepaliTransliterator;

struct MultilineTextEditor {
    input_content: Content,
    output_content: Content,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(Action),
    OutputChanged(Action),
    ToRomanButtonClicked,
    ToNepaliButtonClicked,
}

impl Sandbox for MultilineTextEditor {
    type Message = Message;

    fn new() -> Self {
        MultilineTextEditor {
            input_content: Content::new(),
            output_content: Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Nepali Transliterator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(action) => {
                self.input_content.perform(action);
            }
            Message::OutputChanged(action) => {
                self.output_content.perform(action);
            }
            Message::ToRomanButtonClicked => {
                let transliterator = NepaliTransliterator::new();
                let input_text = self.input_content.text();
                let output = transliterator.to_roman(&input_text);
                self.output_content = Content::with_text(&output);
            }
            Message::ToNepaliButtonClicked => {
                let transliterator = NepaliTransliterator::new();
                let input_text = self.input_content.text();
                let output = transliterator.to_nepali(&input_text);
                self.output_content = Content::with_text(&output);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input_label = text("Input").size(20);
        let input_editor = text_editor(&self.input_content)
            .on_action(Message::InputChanged)
  
            .height(Length::Fixed(150.0));

        let button_row = Row::with_children(vec![
            button("To Nepali").on_press(Message::ToNepaliButtonClicked).into(),
            button("To Roman").on_press(Message::ToRomanButtonClicked).into(),
        ])
        .spacing(10);

        let output_label = text("Output").size(20);
        let output_editor = text_editor(&self.output_content)
            .on_action(Message::OutputChanged)

            .height(Length::Fixed(150.0));

        let content = Column::with_children(vec![
            input_label.into(),
            input_editor.into(),
            button_row.into(),
            output_label.into(),
            output_editor.into(),
        ])
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> iced::Result {
    MultilineTextEditor::run(Settings {
        window: iced::window::Settings {
            size: Size::new(500.0, 500.0),
            ..Default::default()
        },
        ..Default::default()
    })
}