use iced::widget::{svg, Svg};

pub fn get_svg(path: &str, w: u16, h: u16) -> Svg<'static> {
    let svg_handle = svg::Handle::from_path(format!(
        "{}/src/resources/svgs/{}",
        env!("CARGO_MANIFEST_DIR"),
        path
    ));
    svg(svg_handle).width(w).height(h)
}
