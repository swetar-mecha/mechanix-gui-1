use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_smithay::WindowMessage;
use std::any::Any;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Mutex;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static CHANNEL: OnceLock<Mutex<Sender<WindowMessage>>> = OnceLock::new();
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub struct AsyncHandler;

pub enum AsyncHandlerResponse {
    Ok { id: String, payload: Box<dyn Any> },
    _Err { id: String },
}

// TODO: Error Handling in this function
impl AsyncHandler {
    pub fn init(sender: Sender<WindowMessage>) {
        CHANNEL.get_or_init(|| Mutex::new(sender));
        RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"));
    }

    pub fn call<F, T>(f: F, id: &str)
    where
        F: Future<Output = T> + Send + 'static,
        T: Debug + Send + 'static,
    {
        let runtime = RUNTIME.get().expect("Runtime not initialized");
        let id = String::from(id);
        runtime.spawn(async {
            let payload = Box::new(f.await);
            if let Some(channel) = CHANNEL.get() {
                let channel = channel.lock().unwrap();
                let _ = channel.clone().send(WindowMessage::Send {
                    message: mctk_core::msg!(AsyncHandlerResponse::Ok { id, payload }),
                });
            } else {
                panic!("Channel not initialized");
            }
        });
    }
}

#[macro_export]
macro_rules! async_response {
    ($message:ident: $id:expr, $payload:ident as $type:ty, $block:block) => {{
        if let Some(AsyncHandlerResponse::Ok { id, payload }) =
            $message.downcast_ref::<AsyncHandlerResponse>()
        {
            if *id == $id {
                let $payload = payload.downcast_ref::<$type>().unwrap();
                $block
            } else {
                panic!("Mismatched identifier")
            }
        }
    }};
}
