
use bevy::prelude::*;

pub const ROWS: usize = 4;
pub const VISIBLE_COLUMNS: usize = 4;
pub const VIEWPORT_SIZE: f32 = 540.0; // 540 px for the viewport
pub const CELL_SIZE: f32 = VIEWPORT_SIZE / ROWS as f32; // 135.0 px per cell


// medium sizes
pub const ROOT_PADDING:UiRect =  UiRect::new(Val::Px(10.), Val::Px(10.), Val::Px(10.), Val::Px(10.));
pub const ROW_GAP : Val=  Val::Px(4.0);
pub const COLUMN_GAP : Val =  Val::Px(4.0);
pub const TEXT_SIZE : f32 =  15.;
pub const TEXT_PADDING: UiRect = UiRect::all(Val::Px(22.));
pub const ICON_WIDTH: f32 = 84.0;
pub const ICON_HEIGHT: f32 = 84.0;