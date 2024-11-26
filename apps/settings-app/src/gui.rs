use crate::{
    screens::{
        battery::{battery_screen::BatteryScreen, performance_mode::PerformanceMode},
        bluetooth::{
            bluetooth_pairing_enter_code::BluetoothPairingEnterCode,
            bluetooth_pairing_verify_code::BluetoothPairingVerifyCode,
            bluetooth_screen::BluetoothScreen, device_info::BluetoothDeviceInfo,
        },
        display::{display_screen::DisplayScreen, screen_off_time::ScreenOffTime},
        language::{language_screen::LanguageScreen, language_select::LanguageSelect},
        settings_menu::settings_screen::SettingsScreen,
        sound::sound_screen::SoundScreen,
        wireless::{
            available_networks::AvailableNetworksScreen,
            connect_network_enter_code::ConnectNetworkEnterCode, handler::WirelessDetailsItem,
            manage_networks::ManageNetworksScreen, network_details_screen::NetworkDetailsScreen,
            network_screen::NetworkScreen,
        },
    },
    settings::{self, MainSettings},
    shared::h_divider::HDivider,
    AppMessage, AppParams, WirelessMessage,
};
use mctk_core::{
    component::{self, Component, RootComponent},
    lay,
    layout::{Alignment, Direction},
    node, rect,
    reexports::smithay_client_toolkit::reexports::calloop,
    size_pct,
    style::Styled,
    widgets::Div,
    Color, Node,
};
use mctk_macros::{component, state_component_impl};
use mechanix_system_dbus_client::wireless::{
    KnownNetworkListResponse, KnownNetworkResponse, WirelessInfoResponse,
};
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

#[derive(Default, Debug, Clone)]
pub enum NetworkScreenRoutes {
    #[default]
    NetworkScreen,
    NetworkDetailsScreen, // available or connected/manage/known
    ManageNetworksScreen,
    AvailableNetworksScreen,
    ConnectNetworkEnterCode,
}

#[derive(Default, Debug, Clone)]
pub enum Routes {
    #[default]
    SettingsList,
    Network {
        screen: NetworkScreenRoutes,
    },
    BluetoothScreen,
    BluetoothPairingVerifyCode,
    BluetoothPairingEnterCode,
    BluetoothDeviceInfo,
    ScreenOffTime,
    DisplayScreen,
    AppearanceScreen,
    BatteryScreen,
    PerformanceModes,
    SoundScreen,
    LockScreen,
    LanguageScreen,
    LanguageSelect,
}

// TODO : ASK : can we separate module wise state
#[derive(Debug)]
pub struct SettingsAppState {
    settings: Arc<RwLock<MainSettings>>,
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
    current_route: Routes,
    connected_network_name: String,
    // connected_network_info: Option<WirelessInfoResponse>,
    connected_network_details: Option<WirelessDetailsItem>,
    available_networks_list: Vec<WirelessDetailsItem>,
    known_networks_list: Vec<KnownNetworkResponse>,
    wireless_Status: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeRoute { route: Routes },
}

pub enum NetworkMessage {
    WirelessStatus {
        status: bool,
    },
    ConnectedNetworkName {
        name: String,
    },
    ConnectedNetworkDetails {
        details: Option<WirelessDetailsItem>,
    },
    AvailableNetworksList {
        list: Vec<WirelessDetailsItem>,
    },
    KnownNetworksList {
        list: Vec<KnownNetworkResponse>,
    },
    Toggle(bool),
}

/// # SettingsApp State
///
/// This struct is the state definition of the entire application
#[component(State = "SettingsAppState")]
#[derive(Debug, Default)]
pub struct SettingsApp {}

#[state_component_impl(SettingsAppState)]
impl Component for SettingsApp {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => MainSettings::default(),
        };

        self.state = Some(SettingsAppState {
            settings: Arc::new(RwLock::new(MainSettings::default())),
            wireless_Status: false,
            app_channel: None,
            current_route: Routes::default(),
            connected_network_name: String::from(""),
            connected_network_details: None,
            available_networks_list: vec![],
            known_networks_list: vec![],
        });
    }

    fn view(&self) -> Option<Node> {
        let mut app_node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );
        app_node = app_node.push(node!(
            HDivider { size: 1. },
            lay![
                padding: [5.0, 10.0, 5.0, 10.0],
            ],
        ));
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [5., 0., 0., 0.],
            ]
        );

        match &self.state_ref().current_route {
            Routes::SettingsList => {
                base = base.push(node!(SettingsScreen {
                    connected_network_name: self.state_ref().connected_network_name.clone()
                }))
            }
            Routes::Network { screen } => match screen {
                NetworkScreenRoutes::NetworkScreen => {
                    base = base.push(node!(NetworkScreen::new(
                        self.state_ref().wireless_Status.clone(),
                        self.state_ref().connected_network_name.clone() // self.state_ref().connected_network_details.clone()
                    )))
                }
                NetworkScreenRoutes::ManageNetworksScreen => {
                    base = base.push(node!(ManageNetworksScreen {
                        known_networks_list: self.state_ref().known_networks_list.clone(),
                    }))
                }

                NetworkScreenRoutes::AvailableNetworksScreen => {
                    base = base.push(node!(AvailableNetworksScreen {
                        available_networks_list: self.state_ref().available_networks_list.clone(),
                    }))
                }
                NetworkScreenRoutes::NetworkDetailsScreen => {
                    base = base.push(node!(NetworkDetailsScreen {
                        wireless_details: self.state_ref().connected_network_details.clone(),
                    }))
                }
                NetworkScreenRoutes::ConnectNetworkEnterCode => {
                    base = base.push(node!(ConnectNetworkEnterCode {
                        network_name: "Test".to_string()
                    }))
                }
            },

            Routes::LanguageScreen => base = base.push(node!(LanguageScreen {})),
            Routes::DisplayScreen => base = base.push(node!(DisplayScreen {})),
            Routes::ScreenOffTime => base = base.push(node!(ScreenOffTime {})),
            Routes::SoundScreen => base = base.push(node!(SoundScreen {})),
            Routes::PerformanceModes => base = base.push(node!(PerformanceMode {})),
            Routes::BluetoothScreen => base = base.push(node!(BluetoothScreen {})),
            Routes::BluetoothPairingEnterCode => {
                base = base.push(node!(BluetoothPairingEnterCode {}))
            }
            Routes::BluetoothPairingVerifyCode => {
                base = base.push(node!(BluetoothPairingVerifyCode {}))
            }
            Routes::BluetoothDeviceInfo => base = base.push(node!(BluetoothDeviceInfo {})),
            // Routes::LockScreen => todo!(),
            Routes::LanguageSelect => base = base.push(node!(LanguageSelect {})),
            // Routes::AppearanceScreen => todo!(),
            Routes::BatteryScreen => base = base.push(node!(BatteryScreen {})),
            _ => (),
        }

        app_node = app_node.push(base);
        Some(app_node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::ChangeRoute { route } => {
                    self.state_mut().current_route = route.clone();
                }

                _ => (),
            }
        }

        if let Some(msg) = message.downcast_ref::<NetworkMessage>() {
            match msg {
                NetworkMessage::WirelessStatus { status } => {
                    self.state_mut().wireless_Status = status.clone();
                }
                NetworkMessage::ConnectedNetworkName { name } => {
                    self.state_mut().connected_network_name = name.to_string();
                }
                NetworkMessage::ConnectedNetworkDetails { details } => {
                    self.state_mut().connected_network_details = details.to_owned();
                }
                NetworkMessage::AvailableNetworksList { list } => {
                    self.state_mut().available_networks_list = list.clone();
                }
                NetworkMessage::KnownNetworksList { list } => {
                    self.state_mut().known_networks_list = list.clone();
                }
                NetworkMessage::Toggle(value) => {
                    if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        let _ = app_channel.send(AppMessage::Wireless {
                            message: WirelessMessage::Toggle {
                                value: Some(value.clone()),
                            },
                        });
                    }
                }
            }
        }
        vec![]
    }
}
impl RootComponent<AppParams> for SettingsApp {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
        self.state_mut().settings = app_params.settings.clone();
    }
}
