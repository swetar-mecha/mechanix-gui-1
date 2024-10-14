use iced::alignment::Vertical::Bottom;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, row, scrollable, text, toggler,
    vertical_rule, vertical_space,
};
use iced::Length::Fill;
use iced::{Center, Element};

use crate::commons::components::get_svg;
use crate::resources::route_configs::Routes;
use crate::Message;

#[derive(Debug, Clone, Copy)]
pub enum BluetoothMessage {
    BluetoothToggled(bool),
}

#[derive(Default, Debug, Clone)]
pub struct BluetoothScreen {
    is_connected: bool,
}

impl BluetoothScreen {
    pub fn new() -> Self {
        Self { is_connected: true }
    }

    pub fn update(&mut self, message: BluetoothMessage) {
        match message {
            BluetoothMessage::BluetoothToggled(value) => self.is_connected = value,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let toggler = toggler(self.is_connected)
            // .label("Toggle me!")
            .size(30)
            .on_toggle(Message::BluetoothToggled)
            .spacing(10);

        let header: iced::widget::Row<'_, Message> =
            row![text("Bluetooth").size(20), horizontal_space(), toggler]
                .height(50)
                .spacing(20)
                .align_y(Center);

        let footer: iced::widget::Column<'_, Message> = column![
            column![horizontal_rule(20)],
            row![
                button(get_svg("back_icon.svg", 40, 40))
                    .padding([5, 5])
                    .on_press(Message::BackButtonPressed)
                    .style(button::text),
                horizontal_space(),
                button(get_svg("add_icon.svg", 40, 40))
                    .padding([5, 5])
                    .on_press(Message::ChangeRoute(Routes::Settings))
                    .style(button::text),
            ]
            .height(50)
            .spacing(20)
            .align_y(Bottom),
        ];

        let connected_device = "Mecha-1";

        let main_content = column![
            column![horizontal_rule(20)],
            row![
                connected_device,
                horizontal_space(),
                get_svg("connected_icon.svg", 20, 20),
                get_svg("right_arrow.svg", 20, 20),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center),
            column![horizontal_rule(20)],
        ];

        let content = container(column![
            header,
            vertical_space().height(10),
            container(main_content).height(Fill),
            footer
        ])
        .padding(10);

        content.into()
    }
}
