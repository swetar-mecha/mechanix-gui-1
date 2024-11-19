// use std::any::Any;
//
// use futures_util::Future;
// use mctk_core::{
//     msg,
//     reexports::smithay_client_toolkit::reexports::calloop::{channel::Sender, io::Async},
// };
//
// use crate::AppMessage;
//
// use std::sync::{Mutex, OnceLock};
//
// pub struct AsyncHandler;
//
// static APP_CHANNEL: OnceLock<Mutex<Option<Sender<AppMessage>>>> = OnceLock::new();
//
// impl AsyncHandler {
//     pub fn call<F, T>(f: F)
//     where
//         F: Future<Output = T> + Send + 'static,
//     {
//         tokio::spawn(async {
//             let response = f.await;
//             AsyncHandler::get_app_channel().unwrap().send(AppMessage::AsyncHandlerResponse { message: Box::new(response) });
//         });
//     }
//
//     pub fn set_app_channel(app_channel: Sender<AppMessage>) {
//         // Initialize if not already initialized
//         APP_CHANNEL.get_or_init(|| Mutex::new(None));
//
//         if let Some(mutex) = APP_CHANNEL.get() {
//             let mut channel = mutex.lock().unwrap();
//             *channel = Some(app_channel);
//         }
//     }
//
//     // Helper method to get the channel
//     pub fn get_app_channel() -> Option<Sender<AppMessage>> {
//         APP_CHANNEL
//             .get()
//             .and_then(|mutex| mutex.lock().ok())
//             .and_then(|guard| guard.clone())
//     }
// }

#[macro_export]
macro_rules! async_run {
    ($future:expr) => {{
        use std::sync::{Arc, Mutex};
        use std::thread;

        let completed = Arc::new(Mutex::new((false, None)));
        let completed_clone = completed.clone();

        // Spawn the future in a new thread
        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let result = runtime.block_on($future);

            // Store the result and mark as completed
            let mut guard = completed_clone.lock().unwrap();
            *guard = (true, Some(result));
        });

        // Busy wait until the future completes
        loop {
            let guard = completed.lock().unwrap();
            if guard.0 {
                // Future is completed, extract and return the result
                break guard.1.clone().unwrap();
            }
            drop(guard);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    }};
}
