pub mod bluetooth_agent;
mod bluetooth_device;
mod bluetooth_devices;
pub mod bluetooth_structures;

use core::fmt;
use std::thread;
use std::time::Duration;

use crate::bluetooth_agent::adapter1::Adapter1Proxy;
use crate::bluetooth_device::device1::Device1Proxy;
use crate::bluetooth_devices::BluezProxy;
use bluetooth_structures::{BluetoothDevice, BluetoothDeviceProps, CentralDevice};
use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use tokio::runtime::Runtime;
use zbus::fdo;
use zbus::zvariant::ObjectPath;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BluetoothDeviceState {
    Saved,
    Connecting,
    Disconnecting,
    Connected,
    Unknown,
}

impl fmt::Display for BluetoothDeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BluetoothDeviceState::Saved => write!(f, "Saved"),
            BluetoothDeviceState::Connecting => write!(f, "Connecting.."),
            BluetoothDeviceState::Connected => write!(f, "Connected"),
            BluetoothDeviceState::Disconnecting => write!(f, "Disconnecting.."),
            BluetoothDeviceState::Unknown => write!(f, ""),
        }
    }
}

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref WIRELESS_MODEL: BluetoothStore = BluetoothStore {
        is_enabled: Context::new(false),
        device_state: Context::new(BluetoothDeviceState::Saved),
        is_streaming: Context::new(false),
        saved_devices: Context::new(vec![]),
        connected_devices: Context::new(vec![]),
        available_devices: Context::new(vec![]),
        central_device: Context::new(None),
        central_device_name: Context::new("".to_string()),
        central_device_alias: Context::new("".to_string()),
    };
}

#[derive(Model)]
pub struct BluetoothStore {
    pub is_enabled: Context<bool>,
    pub device_state: Context<BluetoothDeviceState>,
    pub is_streaming: Context<bool>,
    pub saved_devices: Context<Vec<BluetoothDeviceProps>>,
    pub connected_devices: Context<Vec<BluetoothDeviceProps>>,
    pub available_devices: Context<Vec<BluetoothDeviceProps>>,
    pub central_device: Context<Option<CentralDevice>>,
    pub central_device_name: Context<String>,
    pub central_device_alias: Context<String>,
}

impl BluetoothStore {
    pub fn get() -> &'static Self {
        &WIRELESS_MODEL
    }

    // pub fn get_enabled_status() {
    //     RUNTIME.spawn(async move {
    //         let connection = zbus::Connection::system().await.unwrap();
    //         let adapter1_proxy = Adapter1Proxy::new(&connection).await.unwrap();
    //         let powered = adapter1_proxy.powered().await.unwrap();

    //         // println!("get_enabled_status powered: {:?}", powered.clone());
    //         BluetoothStore::get().is_enabled.set(powered);

    //         // let mut stream: zbus::proxy::PropertyStream<'_, bool> =
    //         //     adapter1_proxy.receive_powered_changed().await;

    //         // while let Some(property) = stream.next().await {
    //         //     if let Ok(is_enabled) = property.get().await {
    //         //         println!("is_enabled: {}", is_enabled);
    //         //     }
    //         // }
    //     });
    // }

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
                        BluetoothStore::stream_bluetooth_devices();
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

            let bluetooth_proxy = BluezProxy::new(&connection).await.unwrap();
            let managed_objects = bluetooth_proxy.get_managed_objects().await.unwrap();

            let mut available_devices: Vec<BluetoothDeviceProps> = vec![];
            let mut saved_devices: Vec<BluetoothDeviceProps> = vec![];
            let mut connected_devices: Vec<BluetoothDeviceProps> = vec![];

            for (object_path, interfaces) in managed_objects.iter() {
           
                if let Some(device_properties) = interfaces.get("org.bluez.Device1") {
                    if let Some(device) = BluetoothDeviceProps::from_properties(device_properties) {
                        //     true => saved_devices.push(device),
                        //     false => available_devices.push(device),
                        // }

                        match device.connected {
                            true => connected_devices.push(device),
                            false => match device.paired {
                                true => saved_devices.push(device),
                                false => available_devices.push(device),
                            },
                        }
                    }
                } else if let Some(adapter_properties) = interfaces.get("org.bluez.Adapter1") {
                    let device: Option<CentralDevice> =
                        CentralDevice::from_properties(adapter_properties);
                    let name = device.as_ref().map(|dev| dev.name.clone());
                    let alias = device.as_ref().map(|dev| dev.alias.clone());

                    BluetoothStore::get().central_device.set(device);
                    if let Some(name) = name {
                        BluetoothStore::get().central_device_name.set(name);
                    }
                    if let Some(alias) = alias {
                        BluetoothStore::get().central_device_alias.set(alias);
                    }
                }
            }

            BluetoothStore::get().saved_devices.set(saved_devices);
            BluetoothStore::get()
                .connected_devices
                .set(connected_devices);
            BluetoothStore::get()
                .available_devices
                .set(available_devices);
        });
    }

    pub fn stream_bluetooth_devices() {
        RUNTIME.spawn(async {
            println!("stream_bluetooth_devices called....THISSSS");
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = Adapter1Proxy::new(&connection).await.unwrap();

            proxy.start_discovery().await.unwrap();
            println!("discovery started...");
            thread::sleep(Duration::from_secs(8));

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
                    BluetoothStore::get().central_device_alias.set(value);
                    println!("DEVICE NAME UPDATED {:?} ", r);
                    return;
                }
                Err(e) => eprintln!("set_alias error {:?} ", &e),
            };
        });
    }

    pub fn connect_device(device_address: String) {
        RUNTIME.spawn(async move {
            println!("connect_device called....{:?} ", device_address.clone());

            BluetoothStore::get()
                .device_state
                .set(BluetoothDeviceState::Connecting);

            let connection = zbus::Connection::system().await.unwrap();
            let device_path_str =
                format!("/org/bluez/hci0/dev_{}", device_address.replace(":", "_"));
            println!("device_path_str {:?} ", device_path_str);

            let device_path =
                ObjectPath::try_from(device_path_str.as_str()).expect("Invalid object path");

            let device_proxy_builder = Device1Proxy::builder(&connection)
                .destination("org.bluez")
                .unwrap()
                .path(&device_path)
                .unwrap()
                .interface("org.bluez.Device1");

            let device_proxy = device_proxy_builder.unwrap().build().await.unwrap();

            // device_proxy.receive_connected_changed()

            // device_proxy.connect().await.unwrap();
            let paired = device_proxy.paired().await.unwrap();
            if paired == false {
                let pair_resp = match device_proxy.pair().await {
                    Ok(r) => println!("PAIR DONE {:?} ", r),
                    Err(e) => {
                        eprintln!("PAIR ERROR: {:?}", e);
                    }
                };
                println!("pair_resp : {:?} ", pair_resp);
                device_proxy.set_trusted(true).await.unwrap();
            }
            match device_proxy.connect().await {
                Ok(r) => {
                    println!("CONNECTED {:?} ", r);
                    BluetoothStore::get()
                        .device_state
                        .set(BluetoothDeviceState::Connected);
                    Self::stream_bluetooth_devices();
                }
                Err(e) => {
                    BluetoothStore::get()
                        .device_state
                        .set(BluetoothDeviceState::Unknown);
                    eprintln!("ERROR IN CONNECTING DEVICE {:?}", e.to_string());
                }
            }
        });
    }

    pub fn disconnect_device(device_address: String) {
        RUNTIME.spawn(async move {
            println!("disconnect_device called....{:?} ", device_address.clone());
            BluetoothStore::get()
                .device_state
                .set(BluetoothDeviceState::Disconnecting);

            let connection = zbus::Connection::system().await.unwrap();
            let device_path_str =
                format!("/org/bluez/hci0/dev_{}", device_address.replace(":", "_"));
            println!("device_path_str {:?} ", device_path_str);

            let device_path =
                ObjectPath::try_from(device_path_str.as_str()).expect("Invalid object path");

            let device_proxy_builder = Device1Proxy::builder(&connection)
                .destination("org.bluez")
                .unwrap()
                .path(&device_path)
                .unwrap()
                .interface("org.bluez.Device1");

            let device_proxy = device_proxy_builder.unwrap().build().await.unwrap();

            match device_proxy.disconnect().await {
                Ok(r) => {
                    println!("DISCONNECTED {:?} ", r);
                    Self::stream_bluetooth_devices();
                    BluetoothStore::get()
                        .device_state
                        .set(BluetoothDeviceState::Saved);
                }
                Err(e) => {
                    BluetoothStore::get()
                        .device_state
                        .set(BluetoothDeviceState::Saved);
                    eprintln!("ERROR IN CONNECTING DEVICE {:?}", e.to_string());
                }
            }
        });
    }

    pub fn remove_device(device_address: String) {
        RUNTIME.spawn(async move {
            println!("remove_device called....{:?} ", device_address.clone());
            let connection = zbus::Connection::system().await.unwrap();
            let device_path_str =
                format!("/org/bluez/hci0/dev_{}", device_address.replace(":", "_"));
            println!("device_path_str {:?} ", device_path_str);

            let device_path =
                ObjectPath::try_from(device_path_str.as_str()).expect("Invalid object path");

            let adapter1_proxy = Adapter1Proxy::new(&connection).await.unwrap();

            match adapter1_proxy.remove_device(&device_path).await {
                Ok(r) => {
                    println!("REMOVED device {:?} ", r);
                    Self::stream_bluetooth_devices();
                }
                Err(e) => eprintln!("ERROR IN CONNECTING DEVICE {:?}", e.to_string()),
            }
        });
    }

    // let connection = zbus::Connection::system().await.unwrap();
    // let adapter1_proxy = Adapter1Proxy::new(&connection).await.unwrap();
    // let powered = adapter1_proxy.powered().await.unwrap();

    // // println!("get_enabled_status powered: {:?}", powered.clone());
    // BluetoothStore::get().is_enabled.set(powered);

    pub fn stream_bluetooth_enabled_status() {
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



    pub fn start_streaming() {
        let check = BluetoothStore::get().is_streaming.get().clone();
        println!("start_streaming CHECK : {:?}", check);
        if *BluetoothStore::get().is_streaming.get() {
            return;
        }
        BluetoothStore::get().is_streaming.set(true);
        Self::get_managed_objects();
        Self::stream_bluetooth_enabled_status();
        // if BluetoothStore::get().is_enabled.get().clone() {
        Self::stream_bluetooth_devices();
        // }
    }
}


pub async fn get_device_name<'a>(
    connection: &zbus::Connection,
    device_path: zbus::zvariant::OwnedObjectPath,
) -> String {

    let device_path =
    ObjectPath::try_from(device_path.as_str()).expect("Invalid object path");

    let device_proxy_builder = Device1Proxy::builder(&connection)
    .destination("org.bluez")
    .unwrap()
    .path(&device_path)
    .unwrap()
    .interface("org.bluez.Device1");

    let device_proxy = device_proxy_builder.unwrap().build().await.unwrap();
    let name : String = device_proxy.name().await.unwrap();
    name
}
