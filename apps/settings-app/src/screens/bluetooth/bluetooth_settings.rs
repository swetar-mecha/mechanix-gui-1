use crate::components::{text_bold_node, text_node};
use crate::gui::{Message, Routes};
use crate::{components::*, header_node, tab_item_node};

use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::Styled,
    txt,
    widgets::{Div, HDivider, IconButton, IconType, Text},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};

use super::contexts::bluetooth_context::BluetoothStore;

#[derive(Debug)]
pub struct BluetoothSettings {}

impl BluetoothSettings {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for BluetoothSettings {
    fn view(&self) -> Option<Node> {
        let mut text_color = Color::WHITE;

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        base = base.push(header_node!(
            "Bluetooth Settings",
            Box::new(|| {
                msg!(Message::ChangeRoute {
                    route: Routes::Bluetooth {
                        screen: crate::gui::BluetoothScreenRoutes::BluetoothScreen
                    }
                })
            })
        ));

        // let central_device = BluetoothStore::get().central_device.get();
        // let central_device_name = central_device
        //     .as_ref()
        //     .map(|device| device.name.clone())
        //     .unwrap_or_else(|| "Unknown".to_string());

        // let central_device_alias = central_device
        //     .as_ref()
        //     .map(|device| device.alias.clone())
        //     .unwrap_or_else(|| "Unknown".to_string());

        let central_device_name = BluetoothStore::get().central_device_name.get().clone();
        let central_device_alias = BluetoothStore::get().central_device_alias.get().clone();
        let device_name = if central_device_name != central_device_alias {
            central_device_alias.clone()
        } else {
            central_device_name.clone()
        };

        let central_device_row = tab_item_node!(
            [text_node("Device name")],
            [text_bold_node(&device_name.clone())],
            on_click: Some(Box::new(move ||
                msg!(Message::ChangeRoute {
                    route: Routes::Bluetooth { screen: crate::gui::BluetoothScreenRoutes::CentralDeviceScreen }
                })
            )),
        );
        base = base.push(central_device_row);

        Some(base)
    }
}
