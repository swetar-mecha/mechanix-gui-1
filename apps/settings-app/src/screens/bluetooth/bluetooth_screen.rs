use mechanix_system_dbus_client::bluetooth::BluetoothService;
use mechanix_system_dbus_client::wireless::WirelessService;

use crate::async_handler::AsyncHandler;
use crate::async_handler::AsyncHandlerResponse;
use crate::async_response;
use crate::footer_node;
use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

enum BluetoothScreenMessage {
    Toggle,
}

#[derive(Debug, Default)]
pub struct BluetoothState {
    status: bool,
    devices: Vec<String>,
}

#[derive(Debug)]
#[component(State = "BluetoothState")]
pub struct BluetoothScreen {}

#[state_component_impl(BluetoothState)]
impl Component for BluetoothScreen {
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
            Toggle::new(true),
            lay![
                margin:[0., 0., 0., 28.],
                axis_alignment: Alignment::End
            ]
        );
        header = header.push(header_text);
        header = header.push(toggle);
        header_node = header_node.push(header);

        let devices = [("English"), ("English"), ("Chinese")];
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
                tab_item_node!([text_bold_node(device)], [icon_node("right_arrow_icon")])
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
        main_node = main_node.push(footer_node!(Routes::SettingsList));
        base = base.push(main_node);
        Some(base)
    }

    fn update(&mut self, msg: component::Message) -> Vec<component::Message> {
        AsyncHandler::call(async { BluetoothService::status().await.unwrap() }, "wifi");
        async_response!(msg: "wifi", payload as i8, {
            println!("{}", payload);
            std::process::exit(0);
        });
        vec![]
    }
}

impl BluetoothScreen {
    pub fn new() -> Self {
        Self {
            dirty: false,
            state: Some(BluetoothState::default()),
        }
    }
}
