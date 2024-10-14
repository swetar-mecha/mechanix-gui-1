use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Routes {
    Settings,
    Network,
    ManageNetworks,
    Bluetooth,
    Display,
}

struct RouteInfo {
    text: &'static str,
    value: &'static str,
    main_icon: &'static str,
    click_icon: &'static str,
}

// impl Routes {
//     fn value(&self) -> {
//         match *self {

//         }
//     }

// }

// in rust, you have this enum, asiign value for each enum like for Netwrok assign {text : "Network", value: "max1", main_icon: "src/main_icon.svg", click_icon: "right_arrow.svg"};
// for Bluetooth assign {text : "Bluetooth", value: "bluetooth1", main_icon: "src/main_icon.svg", click_icon: "right_arrow.svg"}
// use impl routes

//  enum Routes {
//     Network,
//     Bluetooth,
// }
