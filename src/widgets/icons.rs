use iced::{
    widget::{svg::Handle, Svg},
    Theme
};

#[derive(Clone)]
pub enum SvgIcon {
    Close,
    Minimize,
    Maximize,
}

macro_rules! icon_handle {
    ($icon:expr) => {
        Svg::new(Handle::from_memory(include_bytes!(concat!(
            "../../assets/icons/",
            $icon
        ))))
    };
}

pub fn map_icon(icon: SvgIcon, width: f32, height: f32) -> Svg<Theme> {
    match icon {
        SvgIcon::Close => icon_handle!("close.svg"),
        SvgIcon::Minimize => icon_handle!("minimize.svg"),
        SvgIcon::Maximize => icon_handle!("maximize.svg"),
    }
    .width(width)
    .height(height)
}