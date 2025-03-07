use std::any::Any;
use mctk_core::prelude::*;
// use mctk_core::{component, node, prelude::Component, state_component_impl, widgets::Text, Node};
use mctk_core::widgets::Text;

use crate::{AppMessage, AppParams, AppState};


#[derive(Debug, Clone)]
pub enum Message {
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
        let text_line_1 = format!("Please confirm if the PIN matches");
        let text_line_2 = format!("the one disaplayed on {:?}", device_name);


        let mut main_node =  node!(
            Div::new().bg(Color::TRANSPARENT),
                lay![
                    size: size_pct!(100.0),
                    direction: Direction::Column
                ]
            );

        let modal = node!(
            Div::new().bg(Color::rgba(29., 29., 29., 1.)).border(
                Color::rgba(127., 127., 135., 1.),
                0.,
                (10., 10., 10., 10.)
            ),
            lay![
                size: [320, 170],
                direction: Direction::Column,
                position_type: Absolute,
                position: [130., 70., 0., 0.],
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [15., 15., 15., 10.]

            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, 72],
                    cross_alignment: Alignment::Stretch,
                    direction: Direction::Column
                ]
            )
            .push(node!(
                Div::new(),
                lay![
                    size_pct: [100, 50],
                    direction: Direction::Column,
                    axis_alignment: Alignment::Center
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, 50],
                        direction: Direction::Row,
                        axis_alignment: Alignment::Center,
                    ]
                )
                .push(node!(
                    Text::new(txt!(text_line_1))
                    .style("color", Color::rgba(255., 255., 255., 1.))
                    .style("font", "Inter")
                    .with_class("text-sm leading-4 font-medium"),
                    lay![
                        size: [Auto, 20.], 
                    ]))
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, 50],
                        direction: Direction::Row,
                        axis_alignment: Alignment::Center,
                    ]
                )
                .push(node!(
                    Text::new(txt!(text_line_2))
                    .style("color", Color::rgba(255., 255., 255., 1.))
                    .style("font", "Inter")
                    .with_class("text-sm leading-4 font-medium"),
                    lay![
                        size: [Auto, 20.], 
                    ]))
            ))
            .push(node!(
                Div::new(),
                lay![
                    size_pct: [100, 50],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Center
                ]
            )
            .push(node!(
                Text::new(txt!(passkey))
                .style("color", Color::rgba(255., 255., 255., 1.))
                .style("font", "Inter")
                .with_class("text-xl leading-7 font-bold"),
                lay![
                    size: [Auto, 65.],
                ]))
            )
        )
        .push(
            // BUTTONS
            node!(
                Div::new(),
                lay![
                    size_pct: [100, 28],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
                Div::new(),
                lay![
                    size_pct: [28, 100]
                    axis_alignment: Alignment::Start,
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [72, 100]
                        axis_alignment: Alignment::Stretch,
                    ]
                )
                .push(node!(
                    Button::new(txt!("Cancel"))
                        .style("text_color", Color::WHITE)
                        .style("background_color", Color::rgba(68., 68., 68., 1.))
                        .style("active_color", Color::rgba(82., 81., 81., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        .on_click(Box::new(|| msg!(Message::Cancel))),
                    lay![
                        size_pct: [48, 100],
                        padding: [0., 0., 0., 8.],
                        axis_alignment: Alignment::Start,

                    ]
                ))
                .push(node!(
                    Button::new(txt!("Confirm"))
                        .style("text_color", Color::BLACK)
                        .style("background_color", Color::WHITE)
                        .style("active_color", Color::rgba(194., 184., 184., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        .on_click(Box::new(|| msg!(Message::Confirm))),
                    lay![
                        size_pct: [48, 100],
                        padding: [0., 12., 0., 0.],
                        axis_alignment: Alignment::End,
                    ]
                )),
            ),
        );
        main_node = main_node.push(modal);
        Some(main_node)

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
                    let _ = app_channel.send(AppMessage::Cancel);
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
