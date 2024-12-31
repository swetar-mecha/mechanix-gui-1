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
use simple_base64::engine::general_purpose::NO_PAD;

use super::component::{SettingsRowComponent, SettingsRowParams};
use crate::header_node;
use crate::{
    components::*, screens::network::wireless_model::WirelessModel,
    shared::style_constants::DISABLED_TEXT, utils::truncate,
};
use crate::{
    gui::{Message, NetworkScreenRoutes, Routes},
    screens::battery::battery_model::BatteryModel,
};

#[derive(Debug)]
pub struct SettingsScreen {}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct SettingsItem {
    name: String,
    icon: String,
    on_click: Routes,
}
impl Component for SettingsScreen {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
            ]
        );

        let mut connected_network_name = "    ".to_string();
        if let Some(connected_network) = WirelessModel::get().connected_network.get().clone() {
            connected_network_name = connected_network.clone().name.clone();
            connected_network_name = truncate(connected_network_name, 12);
        }

        // let network_row = node!(SettingsRowComponent {
        //     title: "Network".to_string(),
        //     value: connected_network_name,
        //     icon_1: "wireless_good".to_string(),
        //     icon_1_type: IconType::Png,
        //     icon_2: "".to_string(),
        //     color: Color::WHITE,
        //     on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
        //         route: Routes::Network {
        //             screen: NetworkScreenRoutes::Networking
        //         }
        //     }))),
        // },);
        let network_row = node!(SettingsRowComponent::new(SettingsRowParams {
            title: "Network".to_string(),
            value: connected_network_name,
            icon_1: "wireless_good".to_string(),
            icon_1_type: IconType::Png,
            icon_2: "".to_string(),
            color: Color::WHITE,
            on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                route: Routes::Network {
                    screen: NetworkScreenRoutes::Networking
                }
            }))),
        }));
        let network_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(network_row)
        .push(
            node!(
                Div::new(),
                lay![
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(HDivider {
                size: 0.8,
                color: Color::rgba(83.0, 83.0, 83.0, 1.)
            })),
        );

        // let bluetooth_row = node!(SettingsRowComponent {
        //     title: "Bluetooth".to_string(),
        //     value: "".to_string(),
        //     icon_1: "bluetooth_icon".to_string(),
        //     icon_1_type: IconType::Svg,
        //     icon_2: "grey_right_arrow".to_string(),
        //     color: DISABLED_TEXT.to_owned(),
        //     on_click: None,
        //     // on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
        //     //     route: Routes::BluetoothScreen
        //     // }))),
        // },);
        // let bluetooth_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(bluetooth_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let display_row = node!(SettingsRowComponent {
        //     title: "Display".to_string(),
        //     value: "".to_string(),
        //     icon_1: "display_icon".to_string(),
        //     icon_1_type: IconType::Png,
        //     icon_2: "".to_string(),
        //     color: Color::WHITE,
        //     on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
        //         route: Routes::DisplayScreen
        //     }))),
        // },);
        // let display_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(display_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let battery_percentage = if *BatteryModel::get().battery_percentage.get() > 0.0 {
        //     format!(" {}% ", *BatteryModel::get().battery_percentage.get() as u8)
        // } else {
        //     "".to_string()
        // };

        // let battery_row = node!(SettingsRowComponent {
        //     title: "Battery".to_string(),
        //     value: battery_percentage,
        //     icon_1: "battery_icon".to_string(),
        //     icon_1_type: IconType::Png,
        //     icon_2: "".to_string(),
        //     color: Color::WHITE,
        //     on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
        //         route: Routes::BatteryScreen
        //     }))),
        // },);
        // let battery_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(battery_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let appearance_row = node!(SettingsRowComponent {
        //     title: "Appearance".to_string(),
        //     value: "".to_string(),
        //     icon_1: "appearance_icon".to_string(),
        //     icon_1_type: IconType::Svg,
        //     icon_2: "grey_right_arrow".to_string(),
        //     color: DISABLED_TEXT.to_owned(),
        //     on_click: None,
        // },);
        // let appearance_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(appearance_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let sound_row = node!(SettingsRowComponent {
        //     title: "Sound".to_string(),
        //     value: "".to_string(),
        //     icon_1: "sound_icon".to_string(),
        //     icon_1_type: IconType::Png,
        //     icon_2: "".to_string(),
        //     color: Color::WHITE,
        //     // on_click: None,
        //     on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
        //         route: Routes::SoundScreen
        //     }))),
        // },);
        // let sound_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(sound_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let lock_row = node!(SettingsRowComponent {
        //     title: "Lock".to_string(),
        //     value: "".to_string(),
        //     icon_1: "lock_icon".to_string(),
        //     icon_1_type: IconType::Svg,
        //     icon_2: "grey_right_arrow".to_string(),
        //     color: DISABLED_TEXT.to_owned(),
        //     on_click: None,
        // },);
        // let lock_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(lock_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let date_time_row = node!(SettingsRowComponent {
        //     title: "Date & Time".to_string(),
        //     value: "".to_string(),
        //     icon_1: "date_time_icon".to_string(),
        //     icon_1_type: IconType::Svg,
        //     icon_2: "grey_right_arrow".to_string(),
        //     color: DISABLED_TEXT.to_owned(),
        //     on_click: None,
        // },);
        // let date_time_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(date_time_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let language_row = node!(SettingsRowComponent {
        //     title: "Language".to_string(),
        //     value: "".to_string(),
        //     icon_1: "language_icon".to_string(),
        //     icon_1_type: IconType::Svg,
        //     icon_2: "grey_right_arrow".to_string(),
        //     color: DISABLED_TEXT.to_owned(),
        //     on_click: None,
        //     // on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
        //     //     route: Routes::LanguageScreen
        //     // }))),
        // },);
        // let language_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(language_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let update_row = node!(SettingsRowComponent {
        //     title: "Update".to_string(),
        //     value: "".to_string(),
        //     icon_1: "update_icon".to_string(),
        //     icon_1_type: IconType::Svg,
        //     icon_2: "grey_right_arrow".to_string(),
        //     color: DISABLED_TEXT.to_owned(),
        //     on_click: None,
        // },);
        // let update_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(update_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        // let about_row = node!(SettingsRowComponent {
        //     title: "About".to_string(),
        //     value: "".to_string(),
        //     icon_1: "about_icon".to_string(),
        //     icon_1_type: IconType::Png,
        //     icon_2: "".to_string(),
        //     color: Color::WHITE,
        //     on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
        //         route: Routes::AboutScreen
        //     }))),
        // },);
        // let about_div = node!(
        //     Div::new(),
        //     lay![
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(about_row)
        // .push(node!(HDivider {
        //     size: 0.8,
        //     color: Color::rgba(83.0, 83.0, 83.0, 1.)
        // }));

        let mut scrollable = node!(
            Scrollable::new(size!(440, 360)),
            lay![
                size: [440, 360],
            ]
        );

        let mut list_items = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                size: [440, Auto],
            ]
        );

        list_items = list_items.push(network_div);
        // // list_items = list_items.push(bluetooth_div);
        // list_items = list_items.push(display_div);
        // // list_items = list_items.push(appearance_div);
        // list_items = list_items.push(battery_div);
        // list_items = list_items.push(sound_div);
        // // list_items = list_items.push(lock_div);
        // // list_items = list_items.push(date_time_div);
        // // list_items = list_items.push(language_div);
        // // list_items = list_items.push(update_div);
        // list_items = list_items.push(about_div);

        scrollable = scrollable.push(list_items);

        let content_node = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                size: [440, Auto],
                position: [Auto, 0., 0., 0.],
            ]
        )
        .push(scrollable);

        base = base.push(header_node!("Settings"));
        base = base.push(content_node);

        Some(base)
    }
}
