use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use tokio::runtime::Runtime;
// use zbus::fdo::ObjectManagerProxy;

use crate::screens::bluetooth::bluetooth_dbus::bluez_proxy::{self, BluezProxy};
use crate::screens::bluetooth::bluetooth_dbus::central_device::adapter1::{self, Adapter1Proxy};
use crate::screens::bluetooth::contexts::bluetooth_structures::{BluetoothDevice, CentralDevice};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref WIRELESS_MODEL: BluetoothStore = BluetoothStore {
        is_enabled: Context::new(false),
        saved_devices: Context::new(vec![]),
        available_devices: Context::new(vec![]),
        central_device: Context::new(None),
    };
}

#[derive(Model)]
pub struct BluetoothStore {
    pub is_enabled: Context<bool>,
    pub saved_devices: Context<Vec<BluetoothDevice>>,
    pub available_devices: Context<Vec<BluetoothDevice>>,
    pub central_device: Context<Option<CentralDevice>>,
}

impl BluetoothStore {
    pub fn get() -> &'static Self {
        &WIRELESS_MODEL
    }

    pub fn get_enabled_status() {
        RUNTIME.spawn(async move {
            let connection = zbus::Connection::system().await.unwrap();
            let adapter1_proxy = Adapter1Proxy::new(&connection).await.unwrap();
            let powered = adapter1_proxy.powered().await.unwrap();

            BluetoothStore::get().is_enabled.set(powered);

            if powered.clone() == true {
                Self::get_managed_objects();
            }

            // let mut stream: zbus::proxy::PropertyStream<'_, bool> =
            //     adapter1_proxy.receive_powered_changed().await;

            // while let Some(property) = stream.next().await {
            //     if let Ok(is_enabled) = property.get().await {
            //         println!("is_enabled: {}", is_enabled);
            //     }
            // }
        });
    }

    pub fn toggle_bluetooth() {
        RUNTIME.spawn(async {
            let is_enabled = *BluetoothStore::get().is_enabled.get();

            // if is_enabled == false {
            //     BluetoothStore::get().state.set(WifiState::Connecting);
            // }

            let connection = zbus::Connection::system().await.unwrap();
            let adapter1_proxy = Adapter1Proxy::new(&connection).await.unwrap();
            adapter1_proxy.set_powered(!is_enabled).await.unwrap();

            // BluetoothStore::update();
            Self::get_enabled_status(); // temp
        });
    }

    pub fn get_managed_objects() {
        RUNTIME.spawn(async move {
            let connection = zbus::Connection::system().await.unwrap();

            // let mut saved_devices = vec![];
            // let mut availabled_Devices = vec![];
            let bluetooth_proxy = BluezProxy::new(&connection).await.unwrap();
            let managed_objects = bluetooth_proxy.get_managed_objects().await.unwrap();

            let mut available_devices: Vec<BluetoothDevice> = vec![];
            let mut saved_devices: Vec<BluetoothDevice> = vec![];
            // let mut saved_devices: Vec<Option<BluetoothDevice>> = vec![];

            // println!("BluetoothStore:: INSIDE GET MANAGED OBJECTS------- {}", managed_objects);
            for (object_path, interfaces) in managed_objects.iter() {
                // for (interface, properties) in interfaces.iter() {
                //     println!("CHECK -----------> interface {:?} ", interface);
                // }
                // println!(" =====> object_path: {:?}", object_path);

                if let Some(device_properties) = interfaces.get("org.bluez.Device1") {
                    let paired: bool = device_properties
                        .get("Paired")
                        .unwrap()
                        .downcast_ref()
                        .unwrap();

                    if let Some(device) = BluetoothDevice::from_properties(device_properties) {
                        match paired {
                            true => saved_devices.push(device),
                            false => available_devices.push(device),
                        }
                    }
                } else if let Some(adapter_properties) = interfaces.get("org.bluez.Adapter1") {
                    let device = CentralDevice::from_properties(adapter_properties);
                    BluetoothStore::get().central_device.set(device);
                }
            }

            BluetoothStore::get().saved_devices.set(saved_devices);
            BluetoothStore::get()
                .available_devices
                .set(available_devices);

            // println!("--------> available_devices {:?} ", available_devices);
            // println!("--------> saved_devices {:?} ", saved_devices);
        });
    }
}
