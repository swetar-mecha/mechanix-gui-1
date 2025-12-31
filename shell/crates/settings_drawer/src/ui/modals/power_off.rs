use gpui::*;

use crate::{prelude::AMBER_600, ui::SettingsDrawer};

impl SettingsDrawer {
    pub fn render_power_off(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .absolute()
            // .child(
            //     div()
            //         .id("swiper-wing")
            //         .bg(rgb(AMBER_600))
            //         .flex()
            //         .w_full()
            //         .h(px(333.)),
            // )
            .child(
                div()
                    .id("swiper")
                    .bg(rgb(AMBER_600))
                    .flex()
                    .w_full()
                    .h(px(333.)),
            )
            .child(
                div()
                    .id("swiper-bg")
                    .bg(rgb(0x000000))
                    .flex()
                    .w_full()
                    .h(px(215.)),
            )
            .into_any()
    }
}
