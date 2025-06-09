use bevy::{asset::Handle, image::Image, platform::collections::HashMap};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HomeBundleType {
    #[default]
    App,
    Widget,
}

#[derive(Debug, Clone)]
pub struct HomeEntry {
    pub app_id: String,
    pub icon_name: Option<String>,
    pub icon_path: Option<Handle<Image>>,
    pub name: String,
    pub bundle_type: HomeBundleType,
}

#[derive(Debug, Clone)]
pub struct HomeScreenSettings {
    pub width: f32,
    pub height: f32,
    pub grid_template_columns: u16,
    pub grid_template_rows: u16,
    pub home_entries: HashMap<String, Vec<HomeEntry>>,
}


// TEMP : Example function to create an array of home entries
#[allow(dead_code)]
fn home_entries() -> Vec<HomeEntry> {
    let mut home_entries = Vec::new();
    for i in 0..34 {
        home_entries.push(HomeEntry {
            app_id: format!("App {}", i+1),
            icon_name: None,
            icon_path: None,
            name: format!("App {}", i+1),
            bundle_type: HomeBundleType::App,
        });
    }
    home_entries
}

impl Default for HomeScreenSettings {
    fn default() -> Self {
        Self {
            width: 540.,   // confirm whole window vs only home screen fragment
            height: 540.,
            grid_template_columns: 4,
            grid_template_rows: 4,
            home_entries: HashMap::from([(
                "sm".to_string(),
                home_entries(),
            )]),
        }
    }
}
