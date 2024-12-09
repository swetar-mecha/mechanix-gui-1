use crate::gui::Message;
use crate::gui::{self, Routes};
use crate::screens::sound::sound_model::SoundModel;
use crate::shared::slider::{Slider, SliderType};
use crate::{components::*, header_node, tab_item_node};

use super::input_device_selector::InputDeviceSelector;
use super::output_device_selector::OutputDeviceSelector;

#[derive(Debug, Clone)]
pub enum SoundScreenRoute {
    SoundScreen,
    SelectOutputDevice,
    SelectInputDevice,
}

struct SoundScreenState {
    pub route: SoundScreenRoute,
}

#[derive(Debug)]
#[component(State = "SoundScreenState")]
pub struct SoundScreen {}

impl SoundScreen {
    pub fn new() -> Self {
        SoundScreen {
            dirty: false,
            state: Some(SoundScreenState {
                route: SoundScreenRoute::SoundScreen,
            }),
        }
    }
}

#[state_component_impl(SoundScreenState)]
impl component::Component for SoundScreen {
    fn init(&mut self) {
        SoundModel::update();
        self.state_mut().route = SoundScreenRoute::SoundScreen;
    }

    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                // padding: [15.0, 10.0, 15.0, 10.0],
            ]
        );

        let output_slider = node!(
            Slider::new()
                .value((*SoundModel::get().output_volume.get()).ceil() as u8)
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(226., 102., 0.))
                .on_slide(Box::new(|value| {
                    SoundModel::set_output_volume(value.into());
                    Box::new(())
                }))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[10., 10., 0., 10.]]
        );
        let input_slider = node!(
            Slider::new()
                .value((*SoundModel::get().input_volume.get()) as u8)
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(102., 226., 0.))
                .on_slide(Box::new(|value| {
                    SoundModel::set_input_volume(value.into());
                    Box::new(())
                }))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[10., 10., 0., 10.]]
        );

        let output_device = tab_item_node!(
            [text_node("Output Speaker")],
            [icon_node("right_arrow_icon")],
            on_click: Some(Box::new(move || msg!(Message::ChangeSoundScreenRoute { route: SoundScreenRoute::SelectOutputDevice }))),
        );

        let input_device = tab_item_node!(
            [text_node("Input Microphone")],
            [icon_node("right_arrow_icon")],
            on_click: Some(Box::new(move || msg!(Message::ChangeSoundScreenRoute { route: SoundScreenRoute::SelectInputDevice }))),
        );

        // toggle row
        let toggle_row = node!(
            Div::new(),
            lay![
                size: [480, 50],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment:Alignment::Center,
                padding: [5., 0., 15., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size: [350, 50],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Text::new(txt!("Loudness Enhancer"))
                    .style("color", Color::WHITE)
                    .style("size", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [20, 40],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Toggle::new(false).on_change(Box::new(|value| {
                    SoundModel::get()
                        .max_volume
                        .set(if value { 400000.0 } else { 65536.0 });
                    // WirelessModel::update();
                    Box::new(())
                })),
                lay![]
            )),
        );

        let toggle_node = node!(
            Div::new(),
            lay![
                size: [350, 50],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(toggle_row);

        main_node = main_node.push(node!(Div::new(), lay![size: [20]]));
        main_node = main_node.push(toggle_node);
        main_node = main_node.push(text_bold_node(
            &((*SoundModel::get().output_volume.get() * *SoundModel::get().max_volume.get() * 0.01)
                as u32)
                .to_string(),
        ));
        main_node = main_node.push(output_slider);
        main_node = main_node.push(output_device);
        // main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(node!(Div::new(), lay![size: [20]]));
        main_node = main_node.push(text_bold_node("INPUT"));
        main_node = main_node.push(input_slider);
        main_node = main_node.push(input_device);

        // main_node = main_node.push(footer_node!(Routes::SettingsList));

        base = base.push(header_node!(
            "Sound",
            if let SoundScreenRoute::SoundScreen = self.state_ref().route {
                Box::new(|| {
                    msg!(Message::ChangeRoute {
                        route: Routes::SettingsList,
                    })
                })
            } else {
                Box::new(|| {
                    msg!(Message::ChangeSoundScreenRoute {
                        route: SoundScreenRoute::SoundScreen,
                    })
                })
            }
        ));
        match self.state_ref().route {
            SoundScreenRoute::SelectOutputDevice => {
                base = base.push(node!(OutputDeviceSelector {}))
            }
            SoundScreenRoute::SelectInputDevice => base = base.push(node!(InputDeviceSelector {})),
            SoundScreenRoute::SoundScreen => {
                base = base.push(main_node);
            }
        }

        Some(base)
    }

    fn update(&mut self, msg: prelude::Message) -> Vec<prelude::Message> {
        if let Some(msg) = msg.downcast_ref::<Message>() {
            match msg {
                Message::ChangeSoundScreenRoute { route } => {
                    self.state_mut().route = route.clone();
                }
                _ => (),
            }
        }
        vec![msg]
    }
}
