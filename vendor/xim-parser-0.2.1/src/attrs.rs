use crate::{Attr, AttrType, AttributeName};

macro_rules! define_attrs {
    ($(($name:ident, $attr_name:expr, $ty:expr),)+) => {
        pub const fn get_name(id: u16) -> Option<AttributeName> {
            $(
                if id == $attr_name as _ {
                    return Some($attr_name);
                }
            )+

            None
        }

        pub const fn get_id(name: AttributeName) -> u16 {
            name as u16
        }

        $(
            pub const $name: Attr = Attr {
                id: $attr_name as _,
                name: $attr_name,
                ty: $ty,
            };
        )+
    };
}

define_attrs! {
    (QUERY_INPUT_STYLE, AttributeName::QueryInputStyle, AttrType::Style),
    (INPUT_STYLE, AttributeName::InputStyle, AttrType::Long),
    (CLIENTWIN, AttributeName::ClientWindow, AttrType::Window),
    (FOCUSWIN, AttributeName::FocusWindow, AttrType::Window),
    (FILTER_EVENTS, AttributeName::FilterEvents, AttrType::Long),
    (PREEDIT_ATTRIBUTES, AttributeName::PreeditAttributes, AttrType::NestedList),
    (STATUS_ATTRIBUTES, AttributeName::StatusAttributes, AttrType::NestedList),
    (FONT_SET, AttributeName::FontSet, AttrType::XFontSet),
    (AREA, AttributeName::Area, AttrType::XRectangle),
    (AREA_NEEDED, AttributeName::AreaNeeded, AttrType::XRectangle),
    (COLOR_MAP, AttributeName::ColorMap, AttrType::Long),
    (STD_COLOR_MAP, AttributeName::StdColorMap, AttrType::Long),
    (FOREGROUND, AttributeName::Foreground, AttrType::Long),
    (BACKGROUND, AttributeName::Background, AttrType::Long),
    (BACKGROUND_PIXMAP, AttributeName::BackgroundPixmap, AttrType::Long),
    (SPOT_LOCATION, AttributeName::SpotLocation, AttrType::XPoint),
    (LINE_SPACE, AttributeName::LineSpace, AttrType::Long),
    (SEPARATOR_OF_NESTED_LIST, AttributeName::SeparatorofNestedList, AttrType::Separator),
}
