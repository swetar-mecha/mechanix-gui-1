use iced::alignment::Vertical::Bottom;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, row, scrollable, text, toggler,
    vertical_rule, vertical_space,
};
use iced::Length::Fill;
use iced::{Center, Element, Task};

use crate::commons::components::get_svg;
use crate::resources::route_configs::Routes;
use crate::server::network_client::WirelessService;
use crate::{CustomError, Message};

#[derive(Debug, Clone, Copy)]
pub enum NetworkMessage {
    UpdateToggle(bool),
    NetworkToggled(bool),
}

#[derive(Default, Debug, Clone)]
pub struct NetworkScreen {
    is_connected: bool,
}

impl NetworkScreen {
    pub fn new() -> Self {
        Self {
            is_connected: false,
        }
    }

    pub fn update(&mut self, message: NetworkMessage) {
        match message {
            NetworkMessage::NetworkToggled(value) => {
                if value {
                    println!(
                        "IN NETWORK SCREEN toggle--------------> {:?} : ",
                        value.clone()
                    );
                    // Task::perform(
                    //     enable_wifi,
                    //     Message::PullRequestFetched,
                    // );
                } else {
                    println!(
                        "IN NETWORK SCREEN toggle--------------> {:?} : ",
                        value.clone()
                    );
                }

                self.is_connected = value
            }
            NetworkMessage::UpdateToggle(value) => {
                println!(
                    "IN NETWORK SCREEN update --------------> {:?} : ",
                    value.clone()
                );
                self.is_connected = value
            }
        }
    }

    // pub fn view(&self) -> Element<Message> {
    //     let header: iced::widget::Row<'_, Message> =
    //         row![text("Network").size(20), horizontal_space(),]
    //             .height(50)
    //             .spacing(30)
    //             .align_y(Center);

    //     let content: Column<'_, _, _, _> = column!["Main Content"];

    //     let footer: iced::widget::Row<'_, Message> = row![
    //         button(get_svg("back_icon.svg", 40, 40))
    //             .padding([5, 5])
    //             .on_press(Message::BackButtonPressed)
    //             .style(button::text),
    //         horizontal_space(),
    //         button(get_svg("add_icon.svg", 40, 40))
    //             .padding([5, 5])
    //             .on_press(Message::ChangeRoute(Routes::Settings))
    //             .style(button::text),
    //     ]
    //     .height(50)
    //     .spacing(20)
    //     .align_y(Center);

    //     let content = container(column![header, content, footer]).padding(10);

    //     scrollable(content).into()
    // }
    pub fn view(&self) -> Element<Message> {
        let toggler = toggler(self.is_connected)
            // .label("Toggle me!")
            .size(30)
            .on_toggle(Message::NetworkToggled)
            .spacing(10);

        let header: iced::widget::Row<'_, Message> =
            row![text("Network").size(20), horizontal_space(), toggler]
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
            row![horizontal_rule(20)],
            // ----- x -----
            row![vertical_space().height(20)],
            // ----- x -----
            row![
                "Manage Networks",
                horizontal_space(),
                button(get_svg("right_arrow.svg", 20, 20))
                    .padding([5, 5])
                    .on_press(Message::ChangeRoute(Routes::ManageNetworks))
                    .style(button::text),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center),
            row![horizontal_rule(20)],
            row![
                vertical_space().height(10),
                "Available Networks",
                horizontal_space(),
                get_svg("right_arrow.svg", 20, 20),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center),
            row![horizontal_rule(20)],
            // ----- x -----
            row![vertical_space().height(20)],
            row![text("Other").size(15)],
            // ----- x -----
            row![
                "IP Settings",
                horizontal_space(),
                get_svg("right_arrow.svg", 20, 20),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center),
            row![horizontal_rule(20)],
            row![
                vertical_space().height(10),
                "Ethernet",
                horizontal_space(),
                get_svg("right_arrow.svg", 20, 20),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center),
            row![horizontal_rule(20)],
            row![
                vertical_space().height(10),
                "DNS",
                horizontal_space(),
                get_svg("right_arrow.svg", 20, 20),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center),
            row![horizontal_rule(20)],
            // ----- x -----
        ];

        let content = container(column![
            header,
            vertical_space().height(10),
            container(scrollable(main_content)).height(Fill),
            // main_content,
            footer
        ])
        .padding(10);

        content.into()

        // scrollable(content).into()
        // container(column![scrollable(content), footer]).into()
    }
}

async fn enable_wifi() -> Result<bool, CustomError> {
    let response = zbus::block_on(async move {
        match WirelessService::enable_wifi().await {
            Ok(status) => Ok(status),
            Err(e) => {
                println!("Error enable wifi status: {}", e.to_string());
                Err(CustomError(e.to_string()))
            }
        }
    });
    response
}

async fn disable_wifi() -> Result<bool, CustomError> {
    let response = zbus::block_on(async move {
        match WirelessService::disable_wifi().await {
            Ok(status) => Ok(status),
            Err(e) => {
                println!("Error disable  wifi status: {}", e.to_string());
                Err(CustomError(e.to_string()))
            }
        }
    });
    response
}
