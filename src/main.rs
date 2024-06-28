//बिजि७७<bandesh@gmail.com>


use iced::widget::{button, container, text, text_editor, Column, Row, Container};
use iced::theme::Theme;
use iced::{Alignment, Element, Length, Sandbox, Settings, Size};
use iced::widget::text_editor::{Content, Action};
use NepaliTransliterate::NepaliTransliterator;
use copypasta::{ClipboardContext, ClipboardProvider};

struct MultilineTextEditor {
    input_content: Content,
    output_content: Content,
    clipboard: ClipboardContext,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(Action),
    OutputChanged(Action),
    ToRomanButtonClicked,
    ToNepaliButtonClicked,
    CopyRequested,
    PasteRequested,
}

impl Sandbox for MultilineTextEditor {
    type Message = Message;

    fn new() -> Self {
        MultilineTextEditor {
            input_content: Content::new(),
            output_content: Content::new(),
            clipboard: ClipboardContext::new().unwrap(),
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
            Message::CopyRequested => {
                let text_to_copy = self.output_content.text();
                self.clipboard.set_contents(text_to_copy.to_string()).unwrap();
            }
            Message::PasteRequested => {
                if let Ok(contents) = self.clipboard.get_contents() {
                    self.input_content = Content::with_text(&contents);
                }
            }
        }
    }

fn view(&self) -> Element<Message> {
    let input_label = text("Input").size(20);
    let paste_button = button(Row::with_children(vec![
        text("Paste").into(),
        text("📋").size(16).into(), // Small paste logo
    ]).spacing(5))
    .on_press(Message::PasteRequested)
    .style(iced::theme::Button::Secondary);

    let input_container = Row::new()
        .push(
            Column::new()
                .push(
                    Row::new()
                        .push(input_label)
                        .push(iced::widget::Space::with_width(Length::Fill))
                        .push(paste_button)
                )
                .push(
                    text_editor(&self.input_content)
                        .on_action(Message::InputChanged)
                        .height(Length::Fixed(150.0))
                )
        );

    let button_row = Row::with_children(vec![
        button("To Nepali").on_press(Message::ToNepaliButtonClicked).into(),
        button("To Roman").on_press(Message::ToRomanButtonClicked).into(),
    ])
    .spacing(10);

    let output_label = text("Output").size(20);
    let copy_button = button(Row::with_children(vec![
        text("Copy").into(),
        text("📄").size(16).into(), // Small copy logo
    ]).spacing(5))
    .on_press(Message::CopyRequested)
    .style(iced::theme::Button::Secondary);

    let output_container = Row::new()
        .push(
            Column::new()
                .push(
                    Row::new()
                        .push(output_label)
                        .push(iced::widget::Space::with_width(Length::Fill))
                        .push(copy_button)
                )
                .push(
                    text_editor(&self.output_content)
                        .on_action(Message::OutputChanged)
                        .height(Length::Fixed(150.0))
                        
                )
        );

    let content = Column::with_children(vec![
        input_container.into(),
        button_row.into(),
        output_container.into(),
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