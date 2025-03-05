use mctk_core::prelude::*;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::{self, channel::Event};
use mctk_core::widgets::Text;
use mctk_smithay::xdg_shell::xdg_window::{XdgWindow, XdgWindowParams};
use mctk_smithay::{WindowInfo, WindowMessage, WindowOptions};
use smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::any::Any;
use std::collections::HashMap;
use tokio::sync::oneshot;
use tracing_subscriber::prelude::*;
use zbus::zvariant::ObjectPath;
use bluetoothmanager::BluetoothStore;
use bluetoothmanager::bluetooth_agent::{agent1, agent_manager1};
// use bluetoothmanager::{BluetoothStore, bluetooth_agent::agent1};
// use bluetooth_manager::bluetooth_manager as bluez_zbus;
// use bluez_zbus::{agent1::Agent1Proxy, agent_manager1::AgentManager1Proxy};

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
    println!("Hello, bluetooth world! {:?}", BluetoothStore::get().is_enabled.get());

    let system_conn = zbus::Connection::system().await.unwrap();
    let (agent, mut receiver) = agent1::create();

    let system_conn = zbus::Connection::system().await?;
    let agent_path = ObjectPath::from_static_str_unchecked(AGENT_PATH);
    tracing::debug!("connecting agent");
   
    system_conn.object_server().at(&agent_path, agent).await?;
    tracing::debug!("connecting to bluez agent manager");

    tracing::debug!("connecting to bluez agent manager");

    let bluez = agent_manager1::AgentManager1Proxy::new(&system_conn).await?;

    tracing::debug!("registering agent");

    bluez
        .register_agent(
            &agent_path,
            <&'static str>::from(agent1::Capability::DisplayYesNo),
        )
        .await?;

    if let Err(why) = bluez.request_default_agent(&agent_path).await {
        _ = bluez.unregister_agent(&agent_path).await;
        Err(why)?;
    }

    tracing::debug!("registered");

    while let Some(msg) = receiver.recv().await {
        tracing::debug!(?msg, "message received");

        match msg {
            agent1::Message::RequestAuthorization { device, response } => {
                _ = response.send(true);
            }
            agent1::Message::RequestConfirmation {
                device,
                passkey,
                response,
            } => {
                let (agent_tx, agent_rx) = oneshot::channel();
                let connection = zbus::Connection::system().await?;
                let device = BluetoothStore::get_device(&connection, device).await.unwrap();
                let _ = launch_ui(Some(agent_tx), device.device.name().await.unwrap(), passkey.to_string());
                let res = agent_rx.await.unwrap();
                _ = response.send(res);
            }
            agent1::Message::RequestPasskey { device, response } => {
                _ = response.send(None);
            }
            agent1::Message::RequestPinCode { device, response } => {
                _ = response.send(None);
            }
            agent1::Message::AuthorizeService { device, uuid } => {}
            agent1::Message::Cancel => {}
            agent1::Message::DisplayPasskey {
                device,
                passkey,
                entered,
            } => {}
            agent1::Message::DisplayPinCode { device, pincode } => {}
            agent1::Message::Release => {}
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

#[derive(Debug, Clone)]
enum Message {
    Confirm,
    Cancel
}

#[component(State = "AppState")]
#[derive(Debug, Default)]
pub struct App {}

#[state_component_impl(AppState)]
impl Component for App {
    fn init(&mut self) {
        self.state = Some(AppState {
            app_channel: None,
            device_name: "".to_string(),
            passkey: "".to_string(),
        })
    }

    fn view(&self) -> Option<Node> {
        let device_name = self.state_ref().device_name.clone();
        let passkey = self.state_ref().passkey.clone();
        let message = format!("{:?} would like to pair, confirm code", device_name);

        Some(
            node!(
                Div::new().bg(Color::WHITE),
                lay![
                    size: size_pct!(100.0),
                    direction: Direction::Column
                ]
            )
            .push(node!(
                Div::new().bg(Color::YELLOW),
                lay![
                    size_pct:[100, 85],
                        axis_alignment: Alignment::Center,
                        cross_alignment: Alignment::Center
                ]
            )
            .push(node!(Text::new(txt!(message)).style("font_size", 24.), lay![size: [Auto, 35.], margin: [0., 0., 20., 0.]]))
            .push(node!(Text::new(txt!(passkey)).style("font_size", 40.), lay![size: [Auto, 65.]])))
            .push(
                node!(
                    Div::new().bg(Color::RED),
                    lay![size_pct:[100, 15], 
                    direction: Direction::Row]
                )
                .push(node!(
                    Button::new(txt!("Confirm"))
                        .on_click(Box::new(|| msg!(Message::Confirm)))
                        .style("color", Color::rgb(255., 0., 0.))
                        .style("background_color", Color::BLUE)
                        .style("active_color", Color::rgb(200., 200., 200.))
                        .style("font_size", 18.0),
                    lay![size_pct: [50, 100]]
                ))
                .push(node!(
                    Button::new(txt!("Cancel"))
                        .on_click(Box::new(|| msg!(Message::Cancel)))
                        .style("color", Color::rgb(255., 0., 0.))
                        .style("background_color", Color::LIGHT_GREY)
                        .style("active_color", Color::rgb(200., 200., 200.))
                        .style("font_size", 18.0),
                    lay![size_pct: [50, 100]]
                )),
            ),
        )
    }

    fn update(
        &mut self,
        message: mctk_core::component::Message,
    ) -> Vec<mctk_core::component::Message> {
        println!("App has sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::Confirm) => {
                if let Some(app_channel) = self.state_ref().app_channel.clone() {
                    let _ = app_channel.send(AppMessage::ConfirmPasskey);
                }
            }
            Some(Message::Cancel) => {
                if let Some(app_channel) = self.state_ref().app_channel.clone() {
                    let _ = app_channel.send(AppMessage::ConfirmPasskey);
                }
            }
            _ => (),
        }
        vec![]
    }
}

impl RootComponent<AppParams> for App {
    fn root(&mut self, w: &dyn std::any::Any, app_params: &dyn Any) {
        println!("root initialized");
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
        self.state_mut().device_name = app_params.device_name.clone();
        self.state_mut().passkey = app_params.passkey.clone();
    }
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

    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = XdgWindow::open_blocking::<App, AppParams>(
        XdgWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            svgs,
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