mod components;

use iced::Settings;
use iced::widget::row;

fn main() -> iced::Result {
    return HelixGUI::run(Settings::default());
}

#[derive(default)]
struct HelixGUI {
    running: bool,
}

enum HelixGUIMessage {
    Shutdown,
}

impl HelixGUI {
    pub fn is_running(&self) -> bool {
        return self.running;
    }
}

impl iced::Application for HelixGUI {
    type Executor = iced::executor::Default;
    type Message = HelixGUIMessage;
    type Theme = iced::theme::Application;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        return (Self::default(), iced::Command::none());
    }

    fn title(&self) -> String {
        return String::from("Helix");
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            HelixGUIMessage::Shutdown => self.running = false,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        return row![].padding(0).spacing(0).into();
    }
}