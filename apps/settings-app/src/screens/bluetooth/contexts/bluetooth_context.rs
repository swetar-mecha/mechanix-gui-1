use std::thread;
use std::time::Duration;

use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use tokio::runtime::Runtime;

use crate::screens::bluetooth::bluetooth_dbus::bluez_proxy::{self, BluezProxy};
use crate::screens::bluetooth::bluetooth_dbus::central_device::adapter1::{self, Adapter1Proxy};
use crate::screens::bluetooth::contexts::bluetooth_structures::{BluetoothDevice, CentralDevice};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref WIRELESS_MODEL: BluetoothStore = BluetoothStore {
        is_enabled: Context::new(false),
        is_streaming: Context::new(false),
        saved_devices: Context::new(vec![]),
        available_devices: Context::new(vec![]),
        central_device: Context::new(None),
        central_device_name: Context::new("".to_string()),
    };
}

#[derive(Model)]
pub struct BluetoothStore {
    pub is_enabled: Context<bool>,
    pub is_streaming: Context<bool>,
    pub saved_devices: Context<Vec<BluetoothDevice>>,
    pub available_devices: Context<Vec<BluetoothDevice>>,
    pub central_device: Context<Option<CentralDevice>>,
    pub central_device_name: Context<String>,
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
            // println!("get_enabled_status powered: {:?}", powered.clone());
            BluetoothStore::get().is_enabled.set(powered);

            // let mut stream: zbus::proxy::PropertyStream<'_, bool> =
            //     adapter1_proxy.receive_powered_changed().await;

            // while let Some(property) = stream.next().await {
            //     if let Ok(is_enabled) = property.get().await {
            //         println!("is_enabled: {}", is_enabled);
            //     }
            // }
        });
    }

    fn stream_bluetooth_enabled_status() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = Adapter1Proxy::new(&connection).await.unwrap();
            let mut stream = proxy.receive_powered_changed().await;

            while let Some(property) = stream.next().await {
                if let Ok(powered) = property.get().await {
                    BluetoothStore::get().is_enabled.set(powered);

                    if powered == false {
                        BluetoothStore::get().saved_devices.set([].to_vec());
                        BluetoothStore::get().available_devices.set([].to_vec());
                    }
                }
            }
        });
    }

    pub fn toggle_bluetooth() {
        RUNTIME.spawn(async {
            let is_enabled = *BluetoothStore::get().is_enabled.get();

            // if is_enabled == false {
            //     BluetoothStore::get().state.set(WifiState::Connecting);
            // }

            let connection = zbus::Connection::system().await.unwrap();
            let proxy = Adapter1Proxy::new(&connection).await.unwrap();
            // proxy.set_powered(!is_enabled).await.unwrap();

            println!("updatig set_powered {:?} ", !is_enabled);
            match proxy.set_powered(!is_enabled).await {
                Ok(_) => {
                    BluetoothStore::get().is_enabled.set(!is_enabled);
                    if !is_enabled == true {
                        BluetoothStore::get_managed_objects();
                    } else {
                        BluetoothStore::get().is_streaming.set(false);
                    }
                }
                Err(error) => eprintln!("Failed to toggle bluetooth : {:?} ", error),
            };
        });
    }

    pub fn get_managed_objects() {
        RUNTIME.spawn(async move {
            println!("get_managed_objects called....");

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
                    let device: Option<CentralDevice> =
                        CentralDevice::from_properties(adapter_properties);
                    let name = device.as_ref().map(|dev| dev.name.clone());

                    BluetoothStore::get().central_device.set(device);
                    if let Some(name) = name {
                        BluetoothStore::get().central_device_name.set(name);
                    }
                }
            }

            BluetoothStore::get().saved_devices.set(saved_devices);
            BluetoothStore::get()
                .available_devices
                .set(available_devices);
        });
    }

    pub fn scan_devices() {
        RUNTIME.spawn(async {
            println!("scan_devices called....");
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = Adapter1Proxy::new(&connection).await.unwrap();

            proxy.start_discovery().await.unwrap();
            println!("discovery started...");
            thread::sleep(Duration::from_secs(10));

            BluetoothStore::get_managed_objects();

            proxy.stop_discovery().await.unwrap();
            println!("discovery stopped...");
        });
    }

    pub fn change_central_device_alias(value: String) {
        RUNTIME.spawn(async {
            println!("change_central_device_alias called....{:?} ", value.clone());
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = Adapter1Proxy::new(&connection).await.unwrap();

            match proxy.set_alias(&value).await {
                Ok(r) => {
                    BluetoothStore::get().central_device_name.set(value);
                    println!("DEVICE NAME UPDATED {:?} ", r);
                    return;
                }
                Err(e) => eprintln!("set_alias error {:?} ", &e),
            };
        });
    }

    pub fn start_streaming() {
        let check = BluetoothStore::get().is_streaming.get().clone();
        println!("start_streaming CHECK : {:?}", check);
        if *BluetoothStore::get().is_streaming.get() {
            return;
        }
        BluetoothStore::get().is_streaming.set(true);

        Self::get_enabled_status();
        Self::stream_bluetooth_enabled_status();

        // if BluetoothStore::get().is_enabled.get().clone() {
        Self::get_managed_objects();
        // }
    }
}
