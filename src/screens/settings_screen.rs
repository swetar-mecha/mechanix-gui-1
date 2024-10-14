use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, row, scrollable, svg, text,
    vertical_space, Column, Row, Svg,
};
use iced::{Center, Element};

use crate::resources::route_configs::Routes;
use crate::server::network_client::WirelessService;
use crate::Message;

#[derive(Debug, Clone)]
pub struct SettingsScreen {}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) {
        // match message {
        //     Message::Increment => {
        //         self.value += 1;
        //     }
        //     Message::Decrement => {
        //         self.value -= 1;
        //     }
        // }
    }

    pub fn view(&self) -> Element<Message> {
        let header: iced::widget::Row<'_, Message> =
            row![text("Settings").size(20), horizontal_space(),]
                .height(50)
                .spacing(20)
                .align_y(Center);

        fn get_svg(path: &str, w: u16, h: u16) -> Svg<'static> {
            let svg_handle = svg::Handle::from_path(format!(
                "{}/src/resources/svgs/{}",
                env!("CARGO_MANIFEST_DIR"),
                path
            ));
            svg(svg_handle).width(w).height(h)
        }

        let data_row_type1 = |main_icon, label, value| -> Row<'_, Message> {
            row![
                main_icon,
                horizontal_space().width(10),
                text(label),
                horizontal_space(),
                text(value).size(14),
                horizontal_space().width(10),
                button(get_svg("right_arrow.svg", 20, 20))
                    .padding([5, 5])
                    .on_press(match label {
                        "Network" => Message::ChangeRoute(Routes::Network),
                        "Bluetooth" => Message::ChangeRoute(Routes::Bluetooth),
                        _ => Message::ChangeRoute(Routes::Network),
                    })
                    .style(button::text),
                horizontal_space().width(10),
            ]
            .height(40)
            .align_y(Center)
        };

        let data_row_type2 = |main_icon, label_1| -> Row<'_, Message> {
            row![
                main_icon,
                horizontal_space().width(10),
                text(label_1),
                horizontal_space(),
                button(get_svg("right_arrow.svg", 20, 20))
                    .padding([5, 5])
                    .on_press(Message::ChangeRoute(Routes::Network))
                    .style(button::text),
                horizontal_space().width(10),
                // get_svg("right_arrow.svg", 20, 20)
            ]
            .height(40)
            .align_y(Center)
        };

        let content = container(column![
            header,
            vertical_space().height(10),
            column![row![data_row_type1(
                get_svg("wifi_icon.svg", 24, 24),
                "Network",
                "Mecha_11"
            )]],
            horizontal_rule(20),
            column![data_row_type1(
                get_svg("bluetooth_icon.svg", 24, 24),
                "Bluetooth",
                "Machine1"
            )],
            horizontal_rule(20),
            column![data_row_type2(
                get_svg("display_icon.svg", 24, 24),
                "Display"
            )],
            horizontal_rule(20),
            column![data_row_type2(
                get_svg("appearance_icon.svg", 24, 24),
                "Appearance"
            )],
            horizontal_rule(20),
            column![row![data_row_type1(
                get_svg("battery_icon.svg", 24, 24),
                "Battery",
                "80%"
            )]],
            horizontal_rule(20),
            column![data_row_type2(get_svg("sound_icon.svg", 24, 24), "Sound")],
            horizontal_rule(20),
            column![data_row_type2(get_svg("lock_icon.svg", 24, 24), "Lock")],
            horizontal_rule(20),
            column![data_row_type2(
                get_svg("date_time_icon.svg", 24, 24),
                "Date & Time"
            )],
            horizontal_rule(20),
            column![data_row_type2(
                get_svg("language_icon.svg", 24, 24),
                "Language"
            )],
            horizontal_rule(20),
            column![data_row_type2(get_svg("update_icon.svg", 24, 24), "Update")],
            horizontal_rule(20),
            column![data_row_type2(get_svg("about_icon.svg", 24, 24), "About")],
            horizontal_rule(20),
        ])
        .padding(10);

        scrollable(content).into()
    }
}

// fn network_screen<'a>() -> Element<'a, Message> {
//     column!["This is another reply", horizontal_rule(1),]
//         .spacing(10)
//         .into()
// }
