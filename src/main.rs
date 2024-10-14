use iced::widget::{column, container, text};
use iced::Length::Fill;
use iced::{Color, Element, Task};
use screens::bluetootth::bluetooth_screen::{BluetoothMessage, BluetoothScreen};
use screens::network::manage_networks_screen::ManageNetworksScreen;
use screens::network::network_screen::{NetworkMessage, NetworkScreen};
use server::network_client::WirelessService;

mod resources;
mod screens;
mod server;

use crate::screens::SettingsScreen;
use resources::route_configs::Routes;
use serde::{Deserialize, Serialize};

mod commons;

pub fn main() -> iced::Result {
    iced::application("Settings", SettingsApp::update, SettingsApp::view)
        .font(include_bytes!("resources/fonts/SpaceGrotesk.ttf").as_slice())
        .window_size((480.0, 480.0))
        // .theme(|_| Theme::Dracula)
        .theme(SettingsApp::theme)
        // .run()
        .run_with(SettingsApp::new)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum SettingsList {
    Network,
    Bluetooth,
    Display,
}

pub struct SettingsApp {
    navigated_routes: Vec<Routes>,
    settings_screen: SettingsScreen,
    network_screen: NetworkScreen,
    manage_networks_screen: ManageNetworksScreen,
    bluetooth_screen: BluetoothScreen,
    // isNetworkConnected: bool,
}

// impl Default for SettingsApp {
//     fn default() -> SettingsApp {
//         SettingsApp::new()
//     }
// }

#[derive(Debug, Clone)]
pub enum Message {
    ChangeRoute(Routes),
    BackButtonPressed,
    NetworkToggled(bool),
    BluetoothToggled(bool),
    NetworkConnection(Result<bool, CustomError>),
}

impl SettingsApp {
    // fn new() -> Self {
    //     Self {
    //         navigated_routes: vec![Routes::Settings],
    //         settings_screen: SettingsScreen::new(),
    //         network_screen: NetworkScreen::new(),
    //         bluetooth_screen: BluetoothScreen::new(),
    //         manage_networks_screen: ManageNetworksScreen::new(),
    //         // settings_list: vec![
    //         //     SettingsList::Network,
    //         //     SettingsList::Bluetooth,
    //         //     SettingsList::Display,
    //         // ],
    //     };
    // }

    fn new() -> (Self, Task<Message>) {
        (
            Self {
                navigated_routes: vec![Routes::Settings],
                settings_screen: SettingsScreen::new(),
                network_screen: NetworkScreen::new(),
                bluetooth_screen: BluetoothScreen::new(),
                manage_networks_screen: ManageNetworksScreen::new(),
                // isNetworkConnected: false,
            },
            Task::batch([
                Task::perform(get_info(), Message::NetworkConnection),
                // widget::focus_next(),
            ]),
        )
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::custom(
            String::from("Custom"),
            iced::theme::Palette {
                background: Color::BLACK,
                primary: Color::from_rgb8(45, 138, 255),
                text: Color::WHITE,
                success: Color::from_rgba(0.0, 1.0, 0.0, 1.0),
                danger: Color::from_rgba(1.0, 1.0, 0.0, 1.0),
            },
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeRoute(route) => {
                self.navigated_routes.push(route.clone());
            }
            Message::BackButtonPressed => {
                if self.navigated_routes.len() > 0 {
                    self.navigated_routes.pop();
                }
            }
            Message::NetworkToggled(value) => {
                println!("NETWORK SWITCH TOGGLED ------> {:?}", value);
                // self.isNetworkConnected = value;
                // self.toggler_value = value,
            }
            Message::BluetoothToggled(value) => {
                println!("BLUETOOTH SWITCH TOGGLED ------> {:?}", value);
                // self.bluetooth_screen
                //     .update(BluetoothMessage::BluetoothToggled(value));
                // self.toggler_value = value,
            }
            Message::NetworkConnection(result) => match result {
                Ok(result) => {
                    println!("checking NetworkConnection {:?} ", result);
                    self.network_screen
                        .update(NetworkMessage::UpdateToggle(result));
                }
                Err(e) => {
                    println!("ERROR :: checking NetworkConnection {:?} ", e.to_owned());
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        // let header: iced::widget::Row<'_, Message> =
        //     row![text("Settings").size(20), horizontal_space(),]
        //         .height(50)
        //         .spacing(20)
        //         .align_y(Center);

        // fn get_svg(path: &str, w: u16, h: u16) -> Svg<'static> {
        //     let svg_handle = svg::Handle::from_path(format!(
        //         "{}/src/resources/svgs/{}",
        //         env!("CARGO_MANIFEST_DIR"),
        //         path
        //     ));
        //     svg(svg_handle).width(w).height(h)
        // }

        // let data_row_type1 = |main_icon, label, value| -> Row<'_, Message> {
        //     row![
        //         main_icon,
        //         horizontal_space().width(10),
        //         text(label),
        //         horizontal_space(),
        //         text(value).size(14),
        //         horizontal_space().width(10),
        //         button(get_svg("right_arrow.svg", 20, 20))
        //             .padding([5, 5])
        //             .on_press(Message::NetworkPressed)
        //             .style(button::text),
        //         // get_svg("right_arrow.svg", 20, 20)
        //     ]
        //     .height(40)
        //     .align_y(Center)
        // };

        // let data_row_type2 = |main_icon, label_1| -> Row<'_, Message> {
        //     row![
        //         main_icon,
        //         horizontal_space().width(10),
        //         text(label_1),
        //         horizontal_space(),
        //         get_svg("right_arrow.svg", 20, 20)
        //     ]
        //     .height(40)
        //     .align_y(Center)
        // };

        // let content = container(column![
        //     header,
        //     column![row![data_row_type1(
        //         get_svg("wifi_icon.svg", 24, 24),
        //         "Network",
        //         "Mecha_11"
        //     )]],
        //     horizontal_rule(20),
        //     column![data_row_type1(
        //         get_svg("bluetooth_icon.svg", 24, 24),
        //         "Bluetooth",
        //         "Machine1"
        //     )],
        //     horizontal_rule(20),
        //     column![data_row_type2(
        //         get_svg("display_icon.svg", 24, 24),
        //         "Display"
        //     )],
        //     horizontal_rule(20),
        //     column![data_row_type2(
        //         get_svg("appearance_icon.svg", 24, 24),
        //         "Appearance"
        //     )],
        //     horizontal_rule(20),
        //     column![row![data_row_type1(
        //         get_svg("battery_icon.svg", 24, 24),
        //         "Battery",
        //         "80%"
        //     )]],
        //     horizontal_rule(20),
        //     column![data_row_type2(get_svg("sound_icon.svg", 24, 24), "Sound")],
        //     horizontal_rule(20),
        //     column![data_row_type2(get_svg("lock_icon.svg", 24, 24), "Lock")],
        //     horizontal_rule(20),
        //     column![data_row_type2(
        //         get_svg("date_time_icon.svg", 24, 24),
        //         "Date & Time"
        //     )],
        //     horizontal_rule(20),
        //     column![data_row_type2(
        //         get_svg("language_icon.svg", 24, 24),
        //         "Language"
        //     )],
        //     horizontal_rule(20),
        //     column![data_row_type2(get_svg("update_icon.svg", 24, 24), "Update")],
        //     horizontal_rule(20),
        //     column![data_row_type2(get_svg("about_icon.svg", 24, 24), "About")],
        //     horizontal_rule(20),
        // ])
        // .padding(10);

        // scrollable(content).into()

        // let toggler = toggler(true)
        //     .label("Toggle me!")
        //     .on_toggle(Message::NetworkToggled)
        //     .spacing(10);

        println!(
            "navigation TO -----> {:?}",
            self.navigated_routes[self.navigated_routes.len() - 1]
        );

        let mut current_screen: Element<Message> = column![text("Route not found")].into();
        match self.navigated_routes[self.navigated_routes.len() - 1] {
            Routes::Settings => current_screen = self.settings_screen.view().into(),
            Routes::Network => {
                current_screen = {
                    // Task::perform(get_info());

                    self.network_screen.view().into()
                }
            }
            Routes::ManageNetworks => current_screen = self.manage_networks_screen.view().into(),
            Routes::Bluetooth => current_screen = self.bluetooth_screen.view().into(),
            Routes::Display => {}
        }

        // let scrollable = scrollable(container(current_screen));
        // container(scrollable).width(Fill).height(Fill).into()

        container(current_screen).width(Fill).height(Fill).into()
    }
}
#[derive(Clone, Debug)]
struct CustomError(String);

async fn get_info() -> Result<bool, CustomError> {
    let response = zbus::block_on(async move {
        match WirelessService::wifi_status().await {
            Ok(status) => Ok(status),
            Err(e) => {
                println!("Error getting wifi status: {}", e.to_string());
                Err(CustomError(e.to_string()))
            }
        }
    });
    response
}

// match WirelessService::wifi_status().await {
//     Ok(status) => {
//         println!("CHECK WIFI STATUS : {} ", status.clone());
//         status.clone();
//         // let _ = sender.send(Message::WifiStatusChanged(status));
//         match status {
//             true => {
//                 // get_connected_network(sender).await;
//             }
//             false => {}
//         }
//     }
//     Err(e) => {
//         println!("Error getting device oem info: {}", e.to_string());
//         CustomError(e.to_string());
//     }
// };
