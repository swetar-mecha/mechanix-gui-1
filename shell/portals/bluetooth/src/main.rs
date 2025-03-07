mod gui;

use bluetoothmanager::get_device_name;
use mctk_core::prelude::smithay_client_toolkit::shell::wlr_layer;
use mctk_core::prelude::*;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::{self, channel::Event};
use mctk_smithay::layer_shell::layer_surface::LayerOptions;
use mctk_smithay::layer_shell::layer_window::{LayerWindow, LayerWindowParams};
use mctk_smithay::xdg_shell::xdg_window::{XdgWindow, XdgWindowParams};
use mctk_smithay::{WindowInfo, WindowMessage, WindowOptions};
use smithay_client_toolkit::reexports::calloop::channel::Sender;
use zbus::zvariant::ObjectPath;
use std::collections::HashMap;
use tokio::sync::oneshot;
use tracing_subscriber::prelude::*;
use bluetoothmanager::bluetooth_agent::{agent1, agent_manager1};
use gui::App;

const AGENT_PATH: &str = "/org/bluez/agent/cosmic";
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let log_level = std::env::var("RUST_LOG")
        .ok()
        .and_then(|level| level.parse::<tracing::Level>().ok())
        .unwrap_or(tracing::Level::DEBUG);

    let log_format = tracing_subscriber::fmt::format()
        .pretty()
        .without_time()
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
        .with_thread_names(true);

    let log_filter = tracing_subscriber::fmt::Layer::default()
        .with_writer(std::io::stderr)
        .event_format(log_format)
        .with_filter(tracing_subscriber::filter::filter_fn(move |metadata| {
            metadata.level() <= &log_level
        }));

    tracing_subscriber::registry().with(log_filter).init();

    let system_conn = zbus::Connection::system().await.unwrap();
    let (agent, mut receiver) = agent1::create();
    let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);
    tracing::debug!("connecting agent");

    system_conn.object_server().at(&agent_path, agent).await?;

    tracing::debug!("connecting to bluez agent manager");

    let bluez = agent_manager1::AgentManager1Proxy::new(&system_conn).await?;

    tracing::debug!("registering agent");

    bluez
        .register_agent(
            &agent_path,
            <&'static str>::from(agent1::Capability::DisplayYesNo),   // DEFAULT
            // <&'static str>::from(agent1::Capability::KeyboardDisplay),
        )
        .await?;

    if let Err(why) = bluez.request_default_agent(&agent_path).await {
        println!("ERROR OCCURED==============> {:?}", why.clone());
        _ = bluez.unregister_agent(&agent_path).await;
        Err(why)?;
    }

    tracing::debug!("registered");

    while let Some(msg) = receiver.recv().await {
        tracing::debug!(?msg, "message received");

        match msg {
            agent1::Message::RequestAuthorization { device, response } => {
               println!("RequestAuthorization=========");
                _ = response.send(true);
            }
            agent1::Message::RequestConfirmation {
                device,
                passkey,
                response,
            } => {
                println!("RequestConfirmation=========");

                let (agent_tx, agent_rx) = oneshot::channel();
                let connection = zbus::Connection::system().await?;
                let device_name = get_device_name(&connection, device).await;
                let _ = launch_ui(Some(agent_tx), device_name.clone(), passkey.to_string());
                let res = agent_rx.await.unwrap();
            
                println!("23. agent1::Message::RequestConfirmation-RES------> {:?}", res.to_owned());
                // if res.to_owned() == true {
                //     bluetoothmanager::BluetoothStore::stream_bluetooth_devices();
                // }
                _ = response.send(res);
            }
            agent1::Message::RequestPasskey { device, response } => {
                _ = 
                println!("RequestPasskey-------");

                response.send(None);
            }
            agent1::Message::RequestPinCode { device, response } => {
                _ =
                println!("RequestPinCode-------");
                // get pin code
                // DEBUG  message received, msg: RequestPinCode { device: OwnedObjectPath(ObjectPath("/org/bluez/hci0/dev_10_08_C1_C6_B7_79")), response: Sender { inner: Some(Inner { state: State { is_complete: false, is_closed: false, is_rx_task_set: true, is_tx_task_set: false } }) } }

                // linux -> connected direct
                // car device -> request pin - enter pin dialogue 
                
                 response.send(Some("123456".to_string()));
            }
            agent1::Message::AuthorizeService { device, uuid } => {
                println!("AuthorizeService-------");
            }
            agent1::Message::Cancel => {}
            agent1::Message::DisplayPasskey {
                device,
                passkey,
                entered,
            } => {
                println!("DisplayPasskey-------");

            }
            agent1::Message::DisplayPinCode { device, pincode } => {
                println!("DisplayPinCode-------");

            }
            agent1::Message::Release => {
                println!("Release-------");

            }
        }
    }

    _ = bluez.unregister_agent(&agent_path).await;

    tracing::debug!("exiting");

    Ok(())
}

// App level channel
#[derive(Debug)]
pub enum AppMessage {
    ConfirmPasskey,
    Cancel,
}

#[derive(Debug, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
    device_name: String,
    passkey: String,
}

#[derive(Debug, Default)]
pub struct AppState {
    app_channel: Option<Sender<AppMessage>>,
    device_name: String,
    passkey: String,
}

fn launch_ui(mut agent_tx: Option<oneshot::Sender<bool>>, device_name: String, passkey: String) -> anyhow::Result<()> {
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    // tracing_subscriber::fmt()
    //     .compact()
    //     .with_env_filter(env_filter)
    //     .init();

    // let mut fonts: Vec<String> = Vec::new();
    let assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let window_opts = WindowOptions {
        height: 440 as u32,
        width: 480 as u32,
        scale_factor: 1.0,
    };

    let window_info = WindowInfo {
        id: "mechanix-dialog".to_string(),
        title: "mechanix-dialog".to_string(),
        namespace: "mechanix-dialog".to_string(),
    };
    // let settings = match settings::read_settings_yml() {
    //     Ok(settings) => settings,
    //     Err(e) => {
    //         println!("error while reading settings {:?}", e);
    //         KeyboardSettings::default()
    //     }
    // };


    // let layouts = settings.layouts.clone();

    // let app_id = settings
    // .app
    // .id
    // .clone()
    // .unwrap_or(String::from("mechanix.shell.portals.bluetooth"));
    // let namespace = app_id.clone();
    let namespace = String::from("mechanix.shell.portals.bluetooth");

    let mut layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
        layer: wlr_layer::Layer::Top,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::None,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };
    
    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let (layer_tx, layer_rx) = calloop::channel::channel();

    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<App, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts: layer_shell_opts.clone(),
            svgs,
            layer_tx: Some(layer_tx.clone()),
            layer_rx: Some(layer_rx),
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel_tx.clone()),
            device_name,
            passkey,
        },
    );
    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Cancel => {
                    let _ = agent_tx.take().unwrap().send(false);
                    exit(window_tx_2.clone());
                }
                AppMessage::ConfirmPasskey => {
                    let _ = agent_tx.take().unwrap().send(true);
                    exit(window_tx_2.clone());
                }
            },
            calloop::channel::Event::Closed => {
                println!("calloop::event::closed");
            }
        };
    });

    loop {
        if app.is_exited {
            break;
        }

        let _ = event_loop.dispatch(None, &mut app);
    }

    Ok(())
}

fn exit(window_tx: Sender<WindowMessage>) {
    let _ = window_tx.send(WindowMessage::WindowEvent {
        event: mctk_smithay::WindowEvent::CloseRequested,
    });
}