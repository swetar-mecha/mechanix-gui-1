use crate::{
    StyledWidgetsPlugin,
    components::home::systems::{CoreScrollArea, CoreScrollbarPlugin},
    settings::home::{HomeBundleType, HomeEntry, HomeScreenSettings},
    utils::{FontAssets, Icon},
    widgets::app_bundle::{ButtonVariant, StyledAppBundle},
};
use bevy::{
    color::palettes::css::{GREY, LIGHT_GREY},
    prelude::*,
    winit::WinitSettings,
};
use bevy_asset_loader::prelude::*;
use bevy_core_widgets::hover::Hovering;
use bevy_styled_widgets::prelude::ThemeManager;
mod styles;
mod systems;
use styles::*;

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

/// Loads image assets
#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(key = "images.file.manager.app")]
    file_manager: Handle<Image>,

    #[asset(key = "images.app.icon")]
    app_icon: Handle<Image>,

    #[asset(key = "images.widget")]
    widget_icon: Handle<Image>,
}

#[derive(Component, Debug, Clone)]
pub struct ControlName(pub String);

#[derive(Default, Debug, Clone)]
pub struct GuiSettings {
    pub home: HomeScreenSettings,
}

pub fn run_home() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPlugin, CoreScrollbarPlugin))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>(),
        )
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup_view_root)
        .run();
}
fn setup_view_root(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();

    let current_menu = "sm"; //
    let GuiSettings { home } = GuiSettings::default();
    let HomeScreenSettings {
        width,
        height,
        grid_template_columns,
        grid_template_rows,
        home_entries,
    } = home;

    let all_entries = home_entries.get(current_menu).unwrap_or(&vec![]).clone();
    let items_per_view = ROWS * VISIBLE_COLUMNS;
    let views = (all_entries.len() as f32 / items_per_view as f32).ceil() as usize;
    let total_cells = views * items_per_view;
    let total_columns = views * VISIBLE_COLUMNS;
    let grid_width = total_columns as f32 * CELL_SIZE;

    commands.spawn((Camera2d,));

    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: Val::Px(VIEWPORT_SIZE),
                height: Val::Px(VIEWPORT_SIZE),
                padding: UiRect::all(Val::Px(4.0)),
                overflow: Overflow::scroll_x(),
                ..default()
            },
            BackgroundColor(GREY.into()),
            ScrollPosition {
                offset_x: 5.0,
                offset_y: 0.0,
            },
            CoreScrollArea,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    width: Val::Px(grid_width), // fixed width for the grid
                    height: Val::Percent(100.0),
                    grid_template_columns: RepeatedGridTrack::flex(total_columns as u16, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(ROWS as u16, 1.0),
                    padding: ROOT_PADDING,
                    row_gap: ROW_GAP,
                    column_gap: COLUMN_GAP,
                    ..default()
                })
                .with_children(|parent| {
                    for i in 0..total_cells {
                        if i < all_entries.len() {
                            let home_entry = &all_entries[i];
                            spawn_menu_widget(parent, &image_assets, home_entry);
                        } else {
                            // To fill empty cells
                            parent.spawn((
                                Node {
                                    width: Val::Px(CELL_SIZE),
                                    height: Val::Px(CELL_SIZE),
                                    align_self: AlignSelf::Center,
                                    justify_self: JustifySelf::Center,
                                    ..default()
                                },
                                Hovering::default(),
                            ));
                        }
                    }
                });
        });
}

fn spawn_menu_widget(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    image_assets: &ImageAssets,
    home_entry: &HomeEntry,
) {
    let HomeEntry {
        name, bundle_type, ..
    } = home_entry;

    // kept for widget to use in future
    let (grid_column, grid_row) = if *bundle_type == HomeBundleType::Widget {
        (GridPlacement::span(1), GridPlacement::span(1))
        // (GridPlacement::span(2), GridPlacement::span(2))
    } else {
        (GridPlacement::span(1), GridPlacement::span(1))
    };

    let icon: Handle<Image> = match *bundle_type {
        HomeBundleType::App => image_assets.app_icon.clone(),
        HomeBundleType::Widget => image_assets.widget_icon.clone(),
    };

    parent.spawn((
        Node {
            display: Display::Grid,
            grid_column: grid_column,
            grid_row: grid_row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        Children::spawn(Spawn((
            StyledAppBundle::builder()
                .image(image_assets.app_icon.clone()) // todo: dynamic image
                .text(name.to_string())
                .text_color(LIGHT_GREY.into())
                .variant(ButtonVariant::Primary)
                .border_radius(20.)
                .width(ICON_WIDTH)
                .height(ICON_HEIGHT)
                .build(),
            ControlName(name.to_string()),
        ))),
    ));
}
