use iced::alignment::Vertical::Bottom;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, row, text, vertical_space,
};
use iced::Length::Fill;
use iced::{Center, Element};

use crate::commons::components::get_svg;
use crate::resources::route_configs::Routes;
use crate::Message;

#[derive(Debug, Clone, Copy)]
pub enum ManageNetworksMessage {}

#[derive(Default, Debug, Clone)]
pub struct ManageNetworksScreen {}

impl ManageNetworksScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: ManageNetworksMessage) {}

    pub fn view(&self) -> Element<Message> {
        let header: iced::widget::Row<'_, Message> =
            row![text("Manage Networks").size(20), horizontal_space(),]
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
            row![horizontal_rule(20)],
            row![
                connected_device,
                horizontal_space(),
                get_svg("connected_icon.svg", 20, 20),
                get_svg("right_arrow.svg", 20, 20),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center),
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
