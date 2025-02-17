#[derive(Debug, Clone, PartialEq)]
pub struct BluetoothDevice {
    pub name: String,
    pub alias: String,
    pub address: String,
    pub address_type: String,
    pub legacy_pairing: bool,
    pub icon: String,
    pub trusted: bool,
    pub paired: bool,
    pub powered: Option<bool>,
}

impl BluetoothDevice {
    pub fn from_properties(
        device_properties: &std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    ) -> Option<Self> {
        let name = device_properties
            .get("Name")
            .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
            .unwrap_or_default(); // Use an empty string if None

        // Ensure the device has a valid name
        if name.is_empty() {
            return None; // Return None if name is empty
        }

        Some(BluetoothDevice {
            name,
            address: device_properties
                .get("Address")
                .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),

            address_type: device_properties
                .get("AddressType")
                .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            legacy_pairing: device_properties
                .get("LegacyPairing")
                .and_then(|v| v.downcast_ref::<bool>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            alias: device_properties
                .get("Alias")
                .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            icon: device_properties
                .get("Icon")
                .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            trusted: device_properties
                .get("Trusted")
                .and_then(|v| v.downcast_ref::<bool>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            paired: device_properties
                .get("Paired")
                .and_then(|v| v.downcast_ref::<bool>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            powered: Some(
                device_properties
                    .get("Powered")
                    .and_then(|v| v.downcast_ref::<bool>().ok()) // Convert Result<String, Error> to Option<String>
                    .unwrap_or_default(),
            ),
        })
    }
}

#[derive(Debug)]
pub struct CentralDevice {
    name: String,
    alias: String,
    address: String,
    address_type: String,
    discoverable: bool,
    pairable: bool,
    pairable_timeout: u32,
    powered: bool,
}

impl CentralDevice {
    pub fn from_properties(
        device_properties: &std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    ) -> Option<Self> {
        let name = device_properties
            .get("Name")
            .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
            .unwrap_or_default(); // Use an empty string if None

        // Ensure the device has a valid name
        if name.is_empty() {
            return None; // Return None if name is empty
        }

        Some(CentralDevice {
            name,
            address: device_properties
                .get("Address")
                .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            address_type: device_properties
                .get("AddressType")
                .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            alias: device_properties
                .get("Alias")
                .and_then(|v| v.downcast_ref::<String>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            discoverable: device_properties
                .get("Discoverable")
                .and_then(|v| v.downcast_ref::<bool>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            pairable: device_properties
                .get("Pairable")
                .and_then(|v| v.downcast_ref::<bool>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            pairable_timeout: device_properties
                .get("Pairable")
                .and_then(|v| v.downcast_ref::<u32>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
            powered: device_properties
                .get("Powered")
                .and_then(|v| v.downcast_ref::<bool>().ok()) // Convert Result<String, Error> to Option<String>
                .unwrap_or_default(),
        })
    }
}
