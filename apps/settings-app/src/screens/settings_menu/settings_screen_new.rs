use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct SettingsScreen {}
impl Component for SettingsScreen {
    fn view(&self) -> Option<Node> {
        let mut main_node = node!(
            widgets::Div::new().scroll_y(),
            lay![
                size_pct: [100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
            ]
        );

        main_node = main_node.push(header_node("Settings"));

        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [icon_node("wifi_icon"), text_bold_node("Network")],
            [text_node("mecha1ne.."), icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [icon_node("bluetooth_icon"), text_bold_node("Bluetooth")],
            [text_node("Mecha-1"), icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [icon_node("display_icon"), text_bold_node("Display")],
            [icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [icon_node("bluetooth_icon"), text_bold_node("Appearance")],
            [text_node("Mecha-1"), icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [icon_node("battery_icon"), text_bold_node("Battery")],
            [icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [icon_node("bluetooth_icon"), text_bold_node("Bluetooth")],
            [text_node("Mecha-1"), icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [icon_node("bluetooth_icon"), text_bold_node("Bluetooth")],
            [text_node("Mecha-1"), icon_node("right_arrow_icon")]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));

        Some(main_node)
    }
}
