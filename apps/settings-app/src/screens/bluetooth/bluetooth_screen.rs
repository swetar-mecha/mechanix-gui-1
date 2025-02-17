use crate::{
    components::ComponentHasher,
    gui::{Message, NetworkScreenRoutes, Routes},
    header_node,
    screens::bluetooth::contexts::bluetooth_structures::BluetoothDevice,
    utils::truncate,
};
use core::net;
use std::hash::Hash;

use mctk_core::widgets::Scrollable;
use mctk_core::{
    component::{self, Component},
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{self, Div, IconButton, IconType, Text, Toggle},
    Color, Node,
};
use mctk_core::{event, widgets::HDivider};
use upower::device;

use super::contexts::bluetooth_context::BluetoothStore;

pub struct ClicableIconComponent {
    pub on_click: Option<Box<dyn Fn() -> Box<Message> + Send + Sync>>,
}

impl std::fmt::Debug for ClicableIconComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClicableIconComponent").finish()
    }
}

impl Component for ClicableIconComponent {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0])
    }

    fn view(&self) -> Option<Node> {
        let base = node!(
            Div::new(),
            lay![
                size_pct: [80, Auto],
                axis_alignment: Alignment::Start,

            ]
        );
        Some(base)
    }
}

#[derive(Debug)]
pub struct BluetoothScreen {}

impl BluetoothScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for BluetoothScreen {
    fn init(&mut self) {
        BluetoothStore::get_enabled_status();
        // BluetoothStore::get_managed_objects();
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        self.props_hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let status = BluetoothStore::get().is_enabled.get().clone();

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        // toggle row
        let toggle_row = node!(
            Div::new(),
            lay![
                size: [Auto, 68],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment:Alignment::Center,
                padding: [5., 10., 5., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size: [350, 50],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Text::new(txt!("Bluetooth"))
                    .style("color", Color::rgba(250., 251., 252., 1.))
                    .style("font", "Inter")
                    .with_class("text-xl leading-6 font-medium"),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [20, 40],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                    padding: [0., 0., 0., 10.]
                ]
            )
            .push(node!(
                Toggle::new(status.to_owned())
                    .toggle_type(widgets::ToggleType::Type3)
                    .on_change(Box::new(|value| {
                        BluetoothStore::toggle_bluetooth();
                        Box::new(())
                    })),
                lay![]
            )),
        );

        let mut scrollable_section = node!(
            Scrollable::new(size!(440, 300)),
            lay![
                size: [440, 300],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        ));

        let saved_row_component = |device: BluetoothDevice| {
            node!(
                Div::new(),
                lay![
                    size: [440, 68],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(
                node!(ClicableIconComponent {
                    on_click: Some(Box::new(move || {
                        // WirelessModel::connect_to_saved_network(ssid.clone());
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::Networking
                            }
                        })
                    }))
                })
                // .push(node!(
                //     widgets::Image::new(icon),
                //     lay![
                //         size: [28, 28],
                //         margin:[0., 10., 0., 20.],
                //     ]
                // ))
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
                        Text::new(txt!(truncate(device.name.clone(), 22)))
                            .style("color", Color::WHITE)
                            .style("font", "Inter")
                            .with_class("text-2xl leading-7 font-normal"),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                            cross_alignment: Alignment::Center,
                        ]
                    ))
                    .push(node!(
                        // mini status
                        Text::new(txt!("Saved"))
                            .style("color", Color::WHITE)
                            .style("font", "Inter")
                            .with_class("text-sm leading-5 font-normal"),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                            cross_alignment: Alignment::Center,
                        ]
                    )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [20, Auto],
                        axis_alignment: Alignment::End,
                        cross_alignment:Alignment::Center,
                        padding: [0. , 0., 0., 10.]
                    ]
                )
                .push(node!(
                    IconButton::new("info_icon")
                        // .on_click(Box::new(move || msg!(Message::ChangeRoute {
                        //     route: Routes::Network {
                        //         screen: NetworkScreenRoutes::SavedNetworkDetails {
                        //             mac: network.mac.clone()
                        //         }
                        //     }
                        // })))
                        .icon_type(IconType::Png)
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(34.0),
                                height: Dimension::Px(34.0),
                            }
                        )
                        .style("background_color", Color::TRANSPARENT)
                        .style("border_color", Color::TRANSPARENT)
                        .style("active_color", Color::rgba(85., 85., 85., 0.50))
                        .style("radius", 10.),
                    lay![
                        size: [52, 52],
                        axis_alignment: Alignment::End,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            )
        };

        let available_row_component = |device: BluetoothDevice| {
            // let icon = get_network_icon(network.flags.clone(), Some(network.signal.clone()));

            node!(
                Div::new(),
                lay![
                    size: [440, 68],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(
                node!(ClicableIconComponent {
                    on_click: Some(Box::new(move || {
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::Networking
                            }
                        })
                        // if network.flags.clone().to_lowercase().contains("open") {
                        //     WirelessModel::connect_to_open_network(ssid.clone());
                        //     msg!(Message::ChangeRoute {
                        //         route: Routes::Network {
                        //             screen: NetworkScreenRoutes::Networking
                        //         }
                        //     })
                        // } else {
                        //     msg!(Message::ChangeRoute {
                        //         route: Routes::Network {
                        //             screen: NetworkScreenRoutes::AddNetwork { ssid: ssid.clone() }
                        //         }
                        //     })
                        // }
                    }))
                },)
                // .push(node!(
                //     widgets::Image::new(icon),
                //     lay![
                //         size: [28, 28],
                //         margin:[0., 10., 0., 20.],
                //     ]
                // ))
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, 100],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                        ]
                    )
                    .push(node!(
                        Text::new(txt!(truncate(device.name.clone(), 22)))
                            .style("color", Color::WHITE)
                            .style("font", "Inter")
                            .with_class("text-2xl leading-7 font-normal"),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                        ]
                    )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [20, Auto],
                        axis_alignment: Alignment::End,
                        cross_alignment:Alignment::Center,
                        padding: [0. , 0., 0., 10.]
                    ]
                )
                .push(node!(
                    IconButton::new("info_icon")
                        // .on_click(Box::new(move || msg!(Message::ChangeRoute {
                        //     route: Routes::Network {
                        //         screen: NetworkScreenRoutes::UnknownNetworkDetails {
                        //             mac: network.mac.clone()
                        //         }
                        //     }
                        // })))
                        .icon_type(IconType::Png)
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(34.0),
                                height: Dimension::Px(34.0),
                            }
                        )
                        .style("background_color", Color::TRANSPARENT)
                        .style("border_color", Color::TRANSPARENT)
                        .style("active_color", Color::rgba(85., 85., 85., 0.50))
                        .style("radius", 10.),
                    lay![
                        size: [52, 52],
                        axis_alignment: Alignment::End,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            )
        };

        let available_devices_text: Node = node!(
            Text::new(txt!("Available devices"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 10.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [2.0, 0.0, 2.0, 0.0],
                axis_alignment: Alignment::Start
            ]
        );

        content_node = content_node.push(toggle_row);

        let saved_devices = BluetoothStore::get().saved_devices.get().clone();
        let available_devices = BluetoothStore::get().available_devices.get().clone();
        if status.to_owned() == true {
            // todo: show connected device row

            for (i, device) in saved_devices.clone().into_iter().enumerate() {
                if device.name.clone().len() > 0 {
                    let row_node = node!(
                        Div::new(),
                        lay![
                            size: [440, Auto],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                            cross_alignment: Alignment::Stretch,
                        ],
                    )
                    .push(saved_row_component(device.clone()).key(i as u64))
                    .push(node!(HDivider {
                        size: 0.8,
                        color: Color::rgba(83., 83., 83., 1.)
                    }))
                    .key(2 * i as u64);

                    scrollable_section = scrollable_section.push(row_node);
                }
            }

            scrollable_section = scrollable_section.push(available_devices_text);

            for (i, device) in available_devices.clone().into_iter().enumerate() {
                if device.name.clone().len() > 0 {
                    let row_node = node!(
                        Div::new(),
                        lay![
                            size: [440, Auto],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                            cross_alignment: Alignment::Stretch,
                        ],
                    )
                    .push(available_row_component(device.clone()).key(i as u64))
                    .push(node!(HDivider {
                        size: 0.8,
                        color: Color::rgba(83., 83., 83., 1.)
                    }))
                    .key(2 * i as u64);

                    scrollable_section = scrollable_section.push(row_node);
                }
            }
        }

        if status.clone() == true {
            content_node = content_node.push(scrollable_section);
            content_node = content_node.push(node!(HDivider {
                size: 1.,
                color: Color::rgba(83., 83., 83., 1.)
            }));
        }

        if status.to_owned() == true {
            base = base.push(header_node!(
                "Bluetooth",
                Box::new(|| msg!(Message::ChangeRoute {
                    route: Routes::SettingsList
                })),
                "add_icon",
                Box::new(|| msg!(Message::ChangeRoute {
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::AddNetwork {
                            ssid: "".to_string()
                        }
                    }
                })),
                "wireless_settings",
                Box::new(|| msg!(Message::ChangeRoute {
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::NetworkSettings
                    }
                }))
            ));
        } else {
            base = base.push(header_node!(
                "Bluetooth",
                Box::new(|| msg!(Message::ChangeRoute {
                    route: Routes::SettingsList
                }))
            ));
        }

        base = base.push(content_node);

        Some(base)
    }
}

pub fn get_network_icon(flags: String, signal: Option<String>) -> String {
    let mut icon = if flags.contains("WPA") {
        "secured_wireless_strong".to_string()
    } else {
        "wireless_strong".to_string()
    };

    if let Some(signal_str) = signal {
        if let Ok(signal_strength) = signal_str.parse::<u32>() {
            if signal_strength < 30 {
                icon = icon.replace("strong", "low");
            } else if signal_strength < 70 {
                icon = icon.replace("strong", "weak");
            }
        }
    }

    icon
}
