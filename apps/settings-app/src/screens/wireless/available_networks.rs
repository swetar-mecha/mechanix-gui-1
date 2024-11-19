use crate::footer_node;
use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

use super::handler::WirelessDetailsItem;

#[derive(Debug)]
pub struct AvailableNetworksScreen {
    pub available_networks_list: Vec<WirelessDetailsItem>,
}
impl Component for AvailableNetworksScreen {
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
                size_pct: [100,80],
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
            Text::new(txt!("Available Networks"))
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

        header = header.push(header_text);
        header_node = header_node.push(header);

        let devices = self.available_networks_list.clone();
        main_node = main_node.push(header_node);
        main_node = main_node.push(node!(Div::new(), lay![size: [10]]));
        // main_node = main_node.push(tab_item_node!(
        //     [text_bold_node("mecha compute")],
        //     [icon_node("connected_icon"), icon_node("right_arrow_icon")],
        //     route: Routes::BluetoothDeviceInfo
        // ));
        main_node = main_node.push(node!(HDivider { size: 1. }));

        // for (i, device) in devices.into_iter().enumerate() {
        //     main_node = main_node.push(
        //         tab_item_node!(
        //             [text_bold_node(&device.scan_info.name)],
        //             [
        //                 icon_node("secured_wifi_icon"),
        //                 icon_node("wifi_strength_icon"),
        //                 icon_node("info_icon")
        //             ],
        //             route: Routes::NetworkDetails
        //         )
        //         .key((i + 1) as u64),
        //     );
        //     main_node = main_node.push(node!(HDivider { size: 1. }).key(2 * i as u64));
        // }

        let mut c_node: Node = node!(
            Div::new(),
            // .bg(Color::BLACK)
            // .scroll_y()
            // .style("bar_width", 0.)
            // .style("bar_color", Color::TRANSPARENT)
            // .style("bar_background_color", Color::TRANSPARENT),
            lay![
                size_pct: [100, 80],
                // padding: [5.0, 10.0, 5.0, 10.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        for (i, device) in devices.into_iter().enumerate() {
            main_node = main_node.push(
                tab_item_node!(
                    [text_bold_node(&device.scan_info.name)],
                    [
                        icon_node("secured_wifi_icon"),
                        icon_node("wifi_strength_icon"),
                        icon_node("info_icon")
                    ],
                    route: Routes::NetworkDetails
                )
                .key((i + 1) as u64),
            );
            main_node = main_node.push(node!(HDivider { size: 1. }).key(2 * i as u64));
        }

        base = base.push(footer_node!(Routes::NetworkScreen));
        // main_node = main_node.push(node!(HDivider { size: 1. }));
        c_node = c_node.push(main_node);

        base = base.push(c_node);
        Some(base)
    }
}
