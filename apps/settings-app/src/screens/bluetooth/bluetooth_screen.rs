use crate::{
    components::*,
    gui::{BluetoothScreenRoutes, Message, Routes},
    header_node,
    utils::truncate,
};

use std::hash::Hash;

use bluetoothmanager::{
    bluetooth_structures::BluetoothDeviceProps, BluetoothDeviceState, BluetoothStore,
};

use mctk_core::{
    component::Component,
    event, lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::Styled,
    txt,
    widgets::{self, Div, HDivider, IconButton, IconType, Scrollable, Text, Toggle},
    Color, Node,
};

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
        BluetoothStore::start_streaming();
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        BluetoothStore::get()
            .saved_devices
            .get()
            .clone()
            .len()
            .hash(hasher);

        BluetoothStore::get()
            .available_devices
            .get()
            .clone()
            .len()
            .hash(hasher);

        self.props_hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let status = BluetoothStore::get().is_enabled.get().clone();

        let connected_devices = BluetoothStore::get().connected_devices.get().clone();
        let saved_devices = BluetoothStore::get().saved_devices.get().clone();
        let available_devices = BluetoothStore::get().available_devices.get().clone();

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

        let saved_row_component = |device: BluetoothDeviceProps| {
            let mut device_state = BluetoothStore::get().device_state.get().clone();

            if device.connected == true {
                device_state = BluetoothDeviceState::Connected;
            }

            let device_address = device.address.clone();
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
                        if device.connected == true {
                            BluetoothStore::disconnect_device(device_address.clone());
                        } else {
                            BluetoothStore::connect_device(device_address.clone());
                        }
                        msg!(Message::ChangeRoute {
                            route: Routes::Bluetooth {
                                screen: crate::gui::BluetoothScreenRoutes::BluetoothScreen
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
                        Text::new(txt!(device_state.to_string()))
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
                    IconButton::new("delete_icon")
                        .on_click(Box::new({
                            move || {
                                let device_address_clone = device.address.clone();
                                BluetoothStore::remove_device(device_address_clone);
                                msg!(Message::ChangeRoute {
                                    route: Routes::Bluetooth {
                                        screen: BluetoothScreenRoutes::BluetoothScreen,
                                    },
                                })
                            }
                        }),)
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

        let available_row_component = |device: BluetoothDeviceProps| {
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
                        BluetoothStore::connect_device(device.address.to_string());
                        msg!(Message::ChangeRoute {
                            route: Routes::Bluetooth {
                                screen: crate::gui::BluetoothScreenRoutes::BluetoothScreen
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
                ), // .push(node!(
                   //     IconButton::new("info_icon")
                   //         // .on_click(Box::new(move || msg!(Message::ChangeRoute {
                   //         //     route: Routes::Network {
                   //         //         screen: NetworkScreenRoutes::UnknownNetworkDetails {
                   //         //             mac: network.mac.clone()
                   //         //         }
                   //         //     }
                   //         // })))
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
                   //     ]
                   // )),
            )
        };

        let available_devices_text: Node = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [440, 30],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
                margin: [5., 0., 0., 0.]
            ]
        )
        .push(node!(
            Text::new(txt!("Available devices"))
                .style("color", Color::rgba(250., 251., 252., 1.))
                .style("font", "Inter")
                .with_class("text-xl leading-6 font-medium"),
            lay![
                margin: [2.0, 0.0, 2.0, 0.0],
                axis_alignment: Alignment::Start
            ]
        ))
        .push(node!(
            IconButton::new("refresh_icon")
                .on_click(Box::new(move || {
                    BluetoothStore::stream_bluetooth_devices();
                    msg!(Message::ChangeRoute {
                        route: Routes::Bluetooth {
                            screen: BluetoothScreenRoutes::BluetoothScreen,
                        },
                    })
                    // Box::new(())
                }),)
                .icon_type(IconType::Svg)
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
                margin: [2.0, 0.0, 2.0, 0.0],
                axis_alignment: Alignment::End
            ]
        ));

        content_node = content_node.push(toggle_row);
        content_node = content_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        // content_node = content_node.push(central_device_row);
        content_node = content_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));

        if status.to_owned() == true {
            // todo: show connected device row

            // connected
            for (i, device) in connected_devices.clone().into_iter().enumerate() {
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

            // saved
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

            if available_devices.clone().len() == 0 {
                scrollable_section = scrollable_section.push(node!(
                    Div::new(),
                    lay![
                        size: [440, 65],
                        direction: Direction::Row,
                        cross_alignment: Alignment::Stretch,
                    ]
                ))
            } else if available_devices.clone().len() > 0 {
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
                // "add_icon",
                // Box::new(|| msg!(Message::ChangeRoute {
                //     route: Routes::Network {
                //         screen: NetworkScreenRoutes::AddNetwork {
                //             ssid: "".to_string()
                //         }
                //     }
                // })),
                "wireless_settings",
                IconType::Png,
                Box::new(|| msg!(Message::ChangeRoute {
                    route: Routes::Bluetooth {
                        screen: BluetoothScreenRoutes::BluetoothSettings
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
