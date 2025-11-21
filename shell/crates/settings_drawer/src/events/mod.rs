use networkmanager::interfaces::wireless::WirelessNetworkInfo;

#[derive(Debug)]
pub enum AppEvents {
    WirelessStatusChanged { enabled: bool },
    WirelessStrength { strength: u8 },
    ConnectedNetwork { network: Option<WirelessNetworkInfo> },
    BluetoothEnabled { enabled: bool },
    BluetoothConnectionStatus { connected: bool },
}
