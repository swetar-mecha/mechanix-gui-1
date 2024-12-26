use std::hash::Hash;

use super::wireless_model::WirelessModel;
use crate::components::{detail_row, DetailRow};
use crate::footer_node;
use crate::gui::{Message, NetworkScreenRoutes, Routes};
use crate::utils::truncate;

use mctk_core::widgets::{Button, HDivider};
use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{self, Div, IconButton, IconType, Text},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};

use mechanix_system_dbus_client::wireless::WirelessInfoResponse;

enum NetworkDetailsMessage {
    openModel(bool),
    ForgetNetwork,
}

#[derive(Debug, Clone)]
pub struct NetworkDetailsState {
    pub is_model_open: bool,
}

#[derive(Debug)]
#[component(State = "NetworkDetailsState")]
pub struct NetworkDetails {}

impl NetworkDetails {
    pub fn new() -> Self {
        NetworkDetails {
            dirty: false,
            state: Some(NetworkDetailsState {
                is_model_open: false,
            }),
        }
    }

    fn get_ip_address(&self) -> Option<String> {
        let networks = sysinfo::Networks::new_with_refreshed_list();
        for (interface, info) in &networks {
            if interface.starts_with("wl") {
                for network in info.ip_networks().iter() {
                    if network.addr.is_ipv4() {
                        return Some(network.addr.to_string());
                    }
                }
            }
        }
        None
    }
}

#[state_component_impl(NetworkDetailsState)]
impl Component for NetworkDetails {
    fn init(&mut self) {
        WirelessModel::update();
    }

    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.state_ref().is_model_open.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let ip_address = if let Some(ip_address) = self.get_ip_address() {
            ip_address
        } else {
            "-".to_string()
        };
        let mut text_color = Color::WHITE;
        let connected_network_option = WirelessModel::get().connected_network.get().clone();
        let mut network_status = "Connected";
        let mut security = "-".to_string();
        let connected_network = if let Some(connected_network_option) = connected_network_option {
            security = connected_network_option.flags.clone();
            connected_network_option
        } else {
            network_status = "Not Connected";
            WirelessInfoResponse {
                name: "-".to_string(),
                mac: "-".to_string(),
                flags: "-".to_string(),
                frequency: "-".to_string(),
                signal: "-".to_string(),
            }
        };
        let is_model_open = self.state_ref().is_model_open;

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let text_node = node!(
            Text::new(txt!("Network Information"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 17.5)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, Auto],
            ]
        );

        let header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 10],
                direction: Direction::Row,
                cross_alignment: Alignment::Center,
                axis_alignment: Alignment::Stretch,
                position: [0., 0., Auto, 0.],
                margin: [0., 0., 10., 0.]
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            // .push(node!(
            //     IconButton::new("back_icon")
            //         .on_click(Box::new(|| msg!(Message::ChangeRoute {
            //             route: Routes::Network {
            //                 screen: NetworkScreenRoutes::Networking
            //             }
            //         })))
            //         .icon_type(IconType::Png)
            //         .style(
            //             "size",
            //             Size {
            //                 width: Dimension::Px(34.0),
            //                 height: Dimension::Px(34.0),
            //             }
            //         )
            //         .style("background_color", Color::TRANSPARENT)
            //         .style("border_color", Color::TRANSPARENT)
            //         .style("active_color", Color::rgba(85., 85., 85., 0.50))
            //         .style("radius", 10.),
            //     lay![
            //         size: [42, 42],
            //         padding: [0, 0, 0, 2.],
            //         axis_alignment: Alignment::Start,
            //         cross_alignment: Alignment::Center,
            //     ]
            // ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, Auto],
                        direction: Direction::Column,
                        axis_alignment: Alignment::Start,
                    ]
                )
                .push(text_node),
            ),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End,
                    margin: [0., 0., 0., 10.]
                ]
            ), // .push(node!(
               //     IconButton::new("delete_icon")
               //         .on_click(Box::new(move || msg!(NetworkDetailsMessage::openModel(
               //             !is_model_open
               //         ))))
               //         .icon_type(IconType::Png)
               //         .style(
               //             "size",
               //             Size {
               //                 width: Dimension::Px(34.0),
               //                 height: Dimension::Px(34.0),
               //             }
               //         )
               //         .style("background_color", Color::TRANSPARENT)
               //         .style("border_color", Color::TRANSPARENT)
               //         .style("active_color", Color::rgba(85., 85., 85., 0.50))
               //         .style("radius", 10.),
               //     lay![
               //         size: [52, 52],
               //         axis_alignment: Alignment::End,
               //         cross_alignment: Alignment::Center,
               //         padding: [0., 0., 0., 2.]
               //     ]
               // )),
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 90],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                margin: [12., 0., 0., 0.],
            ]
        );

        let selected_network_row = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                padding: [5., 0., 15., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                widgets::Image::new("wifi_icon"),
                lay![
                    size: [24, 24],
                    margin:[0., 0., 0., 20.],
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, Auto],
                        direction: Direction::Column,
                        axis_alignment: Alignment::Stretch,
                    ]
                )
                .push(node!(
                    Text::new(txt!("Status"))
                        .style("color", Color::WHITE)
                        .style("size", 15.0)
                        .style("line_height", 17.50)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                ))
                .push(node!(
                    // mini status
                    Text::new(txt!(network_status))
                        .style("color", Color::WHITE)
                        .style("size", 14.0)
                        .style("line_height", 20.0)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Bold),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            ),
        );

        let selected_network_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(selected_network_row);

        let details_row_1 = detail_row(
            DetailRow {
                key: "NAME".to_uppercase(),
                value: truncate(connected_network.name.clone(), 17),
            },
            DetailRow {
                key: "STATUS".to_uppercase(),
                value: network_status.to_string(),
            },
        );

        let details_row_2 = detail_row(
            DetailRow {
                key: "Frequency".to_uppercase(),
                value: if connected_network.frequency.starts_with("2") {
                    "2.4 GHz"
                } else {
                    "5 GHz"
                }
                .to_string(),
            },
            DetailRow {
                key: "IP Address".to_uppercase(),
                value: ip_address.to_string(),
            },
        );

        let details_row_3 = detail_row(
            DetailRow {
                key: "MAC Address".to_uppercase(),
                value: connected_network.mac.to_string(),
            },
            DetailRow {
                key: "Security".to_uppercase(),
                value: security.to_string(),
            },
        );

        // content_node = content_node.push(selected_network_node);
        // content_node = content_node.push(node!(HDivider { size: 1. }, lay![
        //     margin: [0.0, 0.0, 30.0, 0.0],
        // ]));

        // content_node = content_node.push(node!(
        //     HDivider {
        //         size: 0.8,
        //         color: Color::rgba(83., 83., 83., 1.)
        //     },
        //     lay![
        //         margin: [0., 0., 10., 0.]
        //     ]
        // ));
        content_node = content_node.push(details_row_1);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.6,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));
        content_node = content_node.push(details_row_2);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.6,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));
        content_node = content_node.push(details_row_3);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));

        // note : in border with width, does not match with radius  - 1. is the border width
        let modal = node!(
            Div::new().bg(Color::rgba(29., 29., 29., 1.)).border(
                Color::rgba(127., 127., 135., 1.),
                0.,
                (10., 10., 10., 10.)
            ),
            lay![
                size: [320, 160],
                direction: Direction::Column,
                position_type: Absolute,
                position: [140., 60., 0., 0.],
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [15., 15., 15., 10.]

            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, 72],
                    cross_alignment: Alignment::Start,
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                Text::new(txt!("Forget this network?"))
                    .style("color", Color::WHITE)
                    .style("size", 18.)
                    .style("line_height", 20.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    size: [Auto],
                ]
            )),
        )
        .push(
            // BUTTONS
            node!(
                Div::new(),
                lay![
                    size_pct: [100, 28],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
                Div::new(),
                lay![
                    size_pct: [28, 100]
                    axis_alignment: Alignment::Start,
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [72, 100]
                        axis_alignment: Alignment::Stretch,
                    ]
                )
                .push(node!(
                    Button::new(txt!("Cancel"))
                        .style("text_color", Color::WHITE)
                        // .style("background_color", Color::rgba(29., 29., 29., 1.))
                        .style("background_color", Color::rgba(68., 68., 68., 1.))
                        .style("active_color", Color::rgba(82., 81., 81., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        // .style("border_color", Color::rgba(127., 127., 135., 1.))
                        // .style("border_width", 1.)
                        .on_click(Box::new(move || msg!(NetworkDetailsMessage::openModel(
                            !is_model_open
                        )))),
                    lay![
                        size_pct: [48, 100],
                        padding: [0., 0., 0., 8.],
                        axis_alignment: Alignment::Start,

                    ]
                ))
                .push(node!(
                    Button::new(txt!("Forget"))
                        // .style("text_color", Color::BLACK)
                        // .style("background_color", Color::rgba(29., 29., 29., 1.))
                        // .style("active_color", Color::rgba(82., 81., 81., 1.))
                        .style("text_color", Color::BLACK)
                        .style("background_color", Color::WHITE)
                        .style("active_color", Color::rgba(194., 184., 184., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        // .style("border_color", Color::rgba(127., 127., 135., 1.))
                        // .style("border_width", 1.)
                        .on_click(Box::new(move || {
                            WirelessModel::disconnect();
                            msg!(Message::ChangeRoute {
                                route: Routes::Network {
                                    screen: NetworkScreenRoutes::Networking
                                }
                            })
                        })),
                    lay![
                        size_pct: [48, 100],
                        padding: [0., 12., 0., 0.],
                        axis_alignment: Alignment::End,
                    ]
                )),
            ),
        );

        if is_model_open.clone() == true {
            base = base.push(modal);
        }
        base = base.push(header_node);
        base = base.push(content_node);

        base = base.push(footer_node!(
            "back_icon",
            Box::new(|| msg!(Message::ChangeRoute {
                route: Routes::Network {
                    screen: NetworkScreenRoutes::Networking
                }
            })),
            "delete_icon",
            IconType::Png,
            Box::new(move || msg!(NetworkDetailsMessage::openModel(!is_model_open)))
        ));

        Some(base)
    }

    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<mctk_core::component::Message> {
        if let Some(message) = msg.downcast_ref::<NetworkDetailsMessage>() {
            match message {
                NetworkDetailsMessage::openModel(value) => {
                    self.state_mut().is_model_open = *value;
                }
                NetworkDetailsMessage::ForgetNetwork => {
                    self.state_mut().is_model_open = false;
                }
            }
        }
        vec![msg]
    }
}
