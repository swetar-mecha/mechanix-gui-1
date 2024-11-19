use super::{component::NetworkRowComponent, handler::WirelessDetailsItem};
use crate::footer_node;
use crate::{
    components::text_node,
    gui::{Message, Routes},
    shared::h_divider::HDivider,
};
use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, IconButton, IconType, Text, Toggle},
    Color, Node,
};
use mechanix_system_dbus_client::wireless::WirelessInfoResponse;

#[derive(Debug)]
pub struct NetworkScreen {
    pub connected_network: Option<WirelessInfoResponse>,
    pub available_networks_list: Vec<WirelessDetailsItem>,
    pub status: bool,
}

impl Component for NetworkScreen {
    fn view(&self) -> Option<Node> {
        let mut text_color = Color::WHITE;

        let connected_network_name: String = match self.connected_network.clone() {
            Some(resp) => resp.name,
            None => {
                text_color = Color::rgb(197., 200., 207.);
                "".to_string()
            }
        };

        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );
        let mut main_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 80],
                direction: Direction::Column,
            ]
        );

        let mut c_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 80],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
            ],
        );

        //Title
        let mut header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                axis_alignment: Alignment::Start,
                direction: Direction::Column
            ]
        );

        let mut header = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                padding: [5.0, 5.0, 10.0, 10.0],
                margin: [0., 0., 20., 0.],
            ]
        );
        let header_text = node!(
            Text::new(txt!("Wireless"))
                .style("font", "Space Grotesk")
                .style("size", 28.)
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("font_weight", FontWeight::Normal),
            lay![
                margin:[2.0, 5.0, 2.0, 5.0],
                size: size!(20.0, 50.0),
                axis_alignment: Alignment::Start
            ]
        );
        let toggle = node!(
            Toggle::new(self.status.clone())
                .on_change(Box::new(|value| msg!(Message::UpdateWirelessStatus(value)))),
            lay![
                margin:[0., 0., 0., 28.],
                axis_alignment: Alignment::End
            ]
        );
        header = header.push(header_text);
        header = header.push(toggle);
        header_node = header_node.push(header);

        let mut network_div_check: Option<Node> = None;

        if connected_network_name.clone().len() > 0 {
            let mut network_div = node!(
                Div::new(),
                lay![
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch,
                    margin: [15., 0, 25., 0],
                ]
            );
            let network_row = node!(
                NetworkRowComponent {
                    title: connected_network_name.clone(),
                    value: "".to_string(),
                    icon_1: "connected_icon".to_string(),
                    icon_2: "right_arrow_icon".to_string(),
                    color: text_color,
                    on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                        route: Routes::NetworkDetails
                    }))),
                },
                lay![
                    padding: [5., 3., 5., 5.],
                ]
            );
            network_div = network_div
                .push(node!(HDivider { size: 1. }))
                .push(network_row)
                .push(node!(HDivider { size: 1. }));

            network_div_check = Some(network_div);
        }

        let mut manage_networks_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let manage_networks_row = node!(
            NetworkRowComponent {
                title: "Manage Networks".to_string(),
                value: "".to_string(),
                icon_1: "".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: None,
            },
            lay![
                padding: [5., 3., 0., 5.],
            ]
        );
        manage_networks_div = manage_networks_div
            .push(manage_networks_row)
            .push(node!(HDivider { size: 1. }));

        let mut available_networks_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                margin: [0., 0, 25., 0],
            ]
        );
        let available_networks_row = node!(
            NetworkRowComponent {
                title: "Available Networks".to_string(),
                value: "".to_string(),
                icon_1: "".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::AvailableNetworksScreen
                }))),
                // on_click: None,
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        available_networks_div = available_networks_div
            .push(available_networks_row)
            .push(node!(HDivider { size: 1. }));

        // // c_node = c_node.push(network_div);
        // // c_node = c_node.push(manage_networks_div);
        // // c_node = c_node.push(available_networks_div);
        base = base.push(footer_node!(Routes::SettingsList));

        if connected_network_name.clone().len() < 1 {
            c_node = c_node.push(manage_networks_div);
            c_node = c_node.push(available_networks_div);
            main_node = main_node.push(c_node);
        } else {
            let mut c_node = node!(
                Div::new(),
                lay![
                    size_pct: [100, 70],
                    cross_alignment: Alignment::Stretch,
                    direction: Direction::Column,
                    padding: [0.0, 10.0, 0.0, 10.0],
                ],
            );
            c_node = c_node.push(network_div_check?);
            c_node = c_node.push(manage_networks_div);
            c_node = c_node.push(available_networks_div);
            c_node = c_node.push(text_node("Other"));
            main_node = main_node.push(c_node);
        }

        base = base.push(header_node);
        base = base.push(main_node);

        Some(base)
    }
}
