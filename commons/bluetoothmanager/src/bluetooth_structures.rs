use futures::join;
use crate::bluetooth_device::{battery1, device1};

#[derive(Debug, Clone, PartialEq)]
pub struct BluetoothDeviceProps {
    pub name: String,
    pub alias: String,
    pub address: String,
    pub address_type: String,
    pub legacy_pairing: bool,
    pub icon: String,
    pub trusted: bool,
    pub paired: bool,
    pub connected: bool,
    pub powered: Option<bool>,
}

impl BluetoothDeviceProps {
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

        Some(BluetoothDeviceProps {
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
            connected: device_properties
                .get("Connected")
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
    pub name: String,
    pub alias: String,
    pub address: String,
    pub address_type: String,
    pub discoverable: bool,
    pub pairable: bool,
    pub pairable_timeout: u32,
    pub powered: bool,
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



#[derive(Debug)]
pub struct BluetoothDevice<'a> {
	pub device: device1::Device1Proxy<'a>,
	pub battery: Option<battery1::Battery1Proxy<'a>>,
}

impl<'a> BluetoothDevice<'a> {
	pub async fn new<'b: 'a>(
		connection: &zbus::Connection,
		path: zbus::zvariant::ObjectPath<'b>,
	) -> zbus::Result<Self> {
		let (device, battery) = join!(
			device1::Device1Proxy::builder(connection)
				.path(&path)?
				.build(),
			battery1::Battery1Proxy::builder(connection)
				.path(path)?
				.build()
		);

		match (device, battery) {
			(Ok(device), Ok(battery)) if battery.percentage().await.is_err() => Ok(Self {
				device,
				battery: None,
			}),
			(Ok(device), Ok(battery)) => Ok(Self {
				device,
				battery: Some(battery),
			}),
			(Ok(device), Err(zbus::Error::InterfaceNotFound)) => Ok(Self {
				device,
				battery: None,
			}),
			(Err(why), _) => Err(why),
			(_, Err(why)) => Err(why),
		}
	}

	pub async fn icon(&self) -> String {
		self.device
			.inner()
			.get_property::<String>("Icon")
			.await
			.unwrap_or("unknown".to_owned())
	}

	pub fn path(&self) -> zbus::zvariant::OwnedObjectPath {
		self.device.inner().path().to_owned().into()
	}
}