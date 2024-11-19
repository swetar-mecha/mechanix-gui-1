use core::panic;
use std::future::poll_fn;
use std::pin::Pin;
use std::task::Poll;

use futures_util::Future;
use mechanix_system_dbus_client::bluetooth::BluetoothService;
use tokio::runtime::Runtime;

use crate::footer_node;
use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

enum BluetoothMessage {
    ToggleBluetooth,
}

#[derive(Debug, Default)]
pub struct BluetoothScreenState {
    pub status: bool,
}

#[derive(Debug)]
#[component(State = "BluetoothScreenState")]
pub struct BluetoothScreen {}

impl BluetoothScreen {
    pub fn new() -> Self {
        Self {
            dirty: false,
            state: Some(BluetoothScreenState { status: false }),
        }
    }
}

#[state_component_impl(BluetoothScreenState)]
impl Component for BluetoothScreen {
    fn init(&mut self) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Error while starting the runtime");
        let status = rt
            .block_on(BluetoothService::status())
            .expect("Error checking connection");
        self.state_mut().status = status > 0;
    }

    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
            ]
        );

        let mut header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 25],
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
            Text::new(txt!("Bluetooth"))
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
            Toggle::new(self.state_ref().status)
                .on_change(Box::new(|_| msg!(BluetoothMessage::ToggleBluetooth))),
            lay![
                margin:[0., 0., 0., 28.],
                axis_alignment: Alignment::End
            ]
        );
        header = header.push(header_text);
        header = header.push(toggle);
        header_node = header_node.push(header);

        let devices = [("English"), ("English"), ("Chinese")];
        base = base.push(footer_node!(
            Routes::SettingsList,
            "add_icon",
            Box::new(|| msg!(Message::ChangeRoute {
                route: Routes::SettingsList,
            }))
        ));
        main_node = main_node.push(header_node);
        main_node = main_node.push(text_node("Available Devices"));
        main_node = main_node.push(node!(Div::new(), lay![size: [10]]));
        main_node = main_node.push(tab_item_node!(
            [text_bold_node("mecha compute")],
            [icon_node("connected_icon"), icon_node("right_arrow_icon")],
            route: Routes::BluetoothDeviceInfo
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        for (i, device) in devices.into_iter().enumerate() {
            main_node = main_node.push(
                tab_item_node!([text_bold_node(device)], [icon_node("right_arrow_icon")], route: Routes::BluetoothPairingEnterCode)
                    .key((i + 1) as u64),
            );
            main_node = main_node.push(node!(HDivider { size: 1. }).key(2 * i as u64));
        }

        main_node = main_node.push(node!(Div::new(), lay![size: [50]]));
        main_node = main_node.push(text_node("Other Devices"));
        main_node = main_node.push(node!(Div::new(), lay![size: [10]]));
        main_node = main_node.push(tab_item_node!(
            [text_bold_node("mecha compute")],
            [icon_node("connected_icon"), icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        for (i, device) in devices.into_iter().enumerate() {
            main_node = main_node.push(
                tab_item_node!([text_bold_node(device)], [icon_node("right_arrow_icon")])
                    .key((i + 1) as u64),
            );
            main_node = main_node.push(node!(HDivider { size: 1. }).key(2 * i as u64));
        }
        base = base.push(main_node);
        Some(base)
    }

    fn update(&mut self, msg: component::Message) -> Vec<component::Message> {
        if let Some(msg) = msg.downcast_ref::<BluetoothMessage>() {
            match msg {
                BluetoothMessage::ToggleBluetooth => {
                    if self.state_ref().status {
                        tokio::spawn(async move {
                            let _ = BluetoothService::disable_bluetooth().await;
                        });
                    } else {
                        tokio::spawn(async move {
                            let _ = BluetoothService::enable_bluetooth().await;
                        });
                    }
                }
            }

            self.state_mut().status = !self.state_ref().status;
            vec![]
        } else {
            vec![msg]
        }
    }
}
