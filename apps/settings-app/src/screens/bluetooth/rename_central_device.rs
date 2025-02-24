use lazy_static::lazy_static;
use mctk_core::context::Context;
use std::fmt::Debug;

use super::contexts::bluetooth_context::BluetoothStore;
use crate::{
    components::*,
    gui::{BluetoothScreenRoutes, Message, Routes},
    header_node, main,
};

lazy_static! {
    static ref FORM: Form = Form {
        name: Context::new("".to_string()),
    };
}

struct Form {
    pub name: Context<String>,
}

impl Debug for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("name", &self.name.get())
            .finish()
    }
}

// #[derive(Debug, Default)]
#[derive(Debug)]
pub struct CentralDeviceState {
    // name: String,
    form: &'static Form,
}

#[derive(Debug)]
#[component(State = "CentralDeviceState")]
pub struct RenameCentralDevice {}

impl RenameCentralDevice {
    pub fn new() -> Self {
        RenameCentralDevice {
            dirty: false,
            state: Some(CentralDeviceState {
                form: &FORM,
                // name: BluetoothStore::get().central_device_name.get().clone(),
            }),
        }
    }
}

#[state_component_impl(CentralDeviceState)]
impl Component for RenameCentralDevice {
    fn init(&mut self) {
        FORM.name
            .set(BluetoothStore::get().central_device_name.get().clone());
    }
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100, Auto],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
                padding: [10.0, 0.0, 5.0, 0.0],
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, Auto],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
            ]
        );

        // let sub_header = node!(
        //     Div::new(),
        //     lay![
        //         margin: [0., 10., 0., 0.]
        //     ]
        // )
        // .push(sub_header_node("Rename Device"));
        // main_node = main_node.push(sub_header);

        main_node = main_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));

        main_node = main_node
            .push(node!(
                Div::new(),
                lay![
                    size_pct: [100, 68],
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                ]
            ))
            .push(node!(
                TextBox::new(Some("".to_string()))
                    .style("background_color", Color::TRANSPARENT)
                    .style("font", "Inter")
                    .with_class("text-xl leading-6 font-medium")
                    .style("text_color", Color::WHITE)
                    .style("border_color", Color::TRANSPARENT)
                    .style("cursor_color", Color::WHITE)
                    .style("placeholder_color", Color::rgb(107., 107., 107.))
                    .on_change(Box::new(move |s| {
                        FORM.name.set(s.to_string());
                        msg!(())
                        // msg!(RenameMessage::NameChanged(s.to_string()))
                    }))
                    .placeholder("Enter name"),
                lay![]
            ));

        main_node = main_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));

        let confirm_icon = if FORM.name.get().clone().is_empty() {
            "disable_confirm_icon"
        } else {
            "enable_confirm_icon"
        };

        base = base.push(header_node!(
            "Rename Device",
            Box::new(|| {
                msg!(Message::ChangeRoute {
                    route: Routes::Bluetooth {
                        screen: BluetoothScreenRoutes::BluetoothScreen
                    }
                })
            }),
            confirm_icon,
            IconType::Svg,
            Box::new(move || {
                if !FORM.name.get().clone().is_empty() {
                    BluetoothStore::change_central_device_alias(FORM.name.get().clone());

                    return msg!(Message::ChangeRoute {
                        route: Routes::Bluetooth {
                            screen: BluetoothScreenRoutes::BluetoothScreen,
                        },
                    });
                } else {
                    println!("HANDLE VALIDATION!");
                }
                Box::new(())
            })
        ));

        base = base.push(main_node);
        Some(base)
    }
}
