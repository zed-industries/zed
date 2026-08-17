#[repr(u8)]
pub enum DisplayItem {
    Fill(Rect, Color),
    Image { id: u32, bounds: Rect },
    ClearScreen,
}

#[repr(C)]
pub struct Rect { x: f32, y: f32, w: f32, h: f32 }

#[repr(C)]
pub struct Color { r: u8, g: u8, b: u8, a: u8 }

#[no_mangle]
pub extern "C" fn push_item(item: DisplayItem) -> bool { 
    ::std::mem::drop(item);
    true
}
