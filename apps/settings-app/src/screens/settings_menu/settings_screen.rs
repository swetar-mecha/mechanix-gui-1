use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, Text},
    Color, Node,
};

use crate::{
    gui::{Message, Routes},
    shared::h_divider::HDivider,
};

use super::component::SettingsRowComponent;

#[derive(Debug)]
pub struct SettingsScreen {
    pub connected_network_name: String,
}

impl Component for SettingsScreen {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            // .scroll_y()
            // .style("bar_width", 0.)
            // .style("bar_color", Color::TRANSPARENT)
            // .style("bar_background_color", Color::TRANSPARENT),
            lay![
                size_pct: [100],
                padding: [5.0, 10.0, 5.0, 10.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        //Title
        let mut header = node!(
            Div::new(),
            // Div::new().bg(Color::MID_GREY),
            lay![
                size_pct: [100, 15],
                direction: Direction::Row,
                cross_alignment: Alignment::Center,
                margin: [0., 0., 5., 0.]
            ]
        );

        let header_text = node!(
            Text::new(txt!("Settings"))
                .style("font", "Space Grotesk")
                .style("size", 28.)
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("font_weight", FontWeight::Normal),
            lay![
                margin:[2.0, 5.0, 2.0, 5.0],
                size: size!(20.0, 50.0),
            ]
        );

        header = header.push(header_text);

        let network_row = node!(
            SettingsRowComponent {
                title: "Network".to_string(),
                value: self.connected_network_name.to_string(),
                icon_1: "wifi_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::NetworkScreen
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let network_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(network_row)
        .push(node!(HDivider { size: 1. }));

        let bluetooth_row = node!(
            SettingsRowComponent {
                title: "Bluetooth".to_string(),
                value: "".to_string(), // TODO: api integration
                icon_1: "bluetooth_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::BluetoothScreen
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let bluetooth_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(bluetooth_row)
        .push(node!(HDivider { size: 1. }));

        let display_row = node!(
            SettingsRowComponent {
                title: "Display".to_string(),
                value: "".to_string(),
                icon_1: "display_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::DisplayScreen
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let display_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(display_row)
        .push(node!(HDivider { size: 1. }));

        let battery_row = node!(
            SettingsRowComponent {
                title: "Battery".to_string(),
                value: "".to_string(),
                icon_1: "battery_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::BatteryScreen
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let battery_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(battery_row)
        .push(node!(HDivider { size: 1. }));

        let appearance_row = node!(
            SettingsRowComponent {
                title: "Appearance".to_string(),
                value: "".to_string(),
                icon_1: "appearance_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: None,
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let appearance_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(appearance_row)
        .push(node!(HDivider { size: 1. }));

        let sound_row = node!(
            SettingsRowComponent {
                title: "Sound".to_string(),
                value: "".to_string(),
                icon_1: "sound_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::SoundScreen
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let sound_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(sound_row)
        .push(node!(HDivider { size: 1. }));

        let lock_row = node!(
            SettingsRowComponent {
                title: "Lock".to_string(),
                value: "".to_string(),
                icon_1: "lock_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: None,
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let lock_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(lock_row)
        .push(node!(HDivider { size: 1. }));

        let date_time_row = node!(
            SettingsRowComponent {
                title: "Date & Time".to_string(),
                value: "".to_string(),
                icon_1: "date_time_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: None,
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let date_time_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(date_time_row)
        .push(node!(HDivider { size: 1. }));

        let language_row = node!(
            SettingsRowComponent {
                title: "Language".to_string(),
                value: "".to_string(),
                icon_1: "language_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::LanguageScreen
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let language_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(language_row)
        .push(node!(HDivider { size: 1. }));

        let update_row = node!(
            SettingsRowComponent {
                title: "Update".to_string(),
                value: "".to_string(),
                icon_1: "update_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: None,
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let update_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(update_row)
        .push(node!(HDivider { size: 1. }));

        let about_row = node!(
            SettingsRowComponent {
                title: "About".to_string(),
                value: "".to_string(),
                icon_1: "about_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: None,
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        let about_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(about_row)
        .push(node!(HDivider { size: 1. }));

        base = base.push(header);
        base = base.push(network_div);
        base = base.push(bluetooth_div);
        base = base.push(display_div);
        base = base.push(appearance_div);
        base = base.push(battery_div);
        base = base.push(sound_div);
        base = base.push(lock_div);
        base = base.push(date_time_div);
        base = base.push(language_div);
        base = base.push(update_div);
        base = base.push(about_div);

        Some(base)
    }
}
