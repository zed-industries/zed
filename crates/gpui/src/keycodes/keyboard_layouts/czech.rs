use std::{collections::HashMap, sync::LazyLock};

use crate::VirtualKeyCode;

// http://kbdlayout.info/kbdcz1/virtualkeys?arrangement=ANSI104
pub(crate) static CZECH_QWERTY_ANSI: LazyLock<HashMap<u16, VirtualKeyCode>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(0x32, VirtualKeyCode::OEM3);
    map.insert(0x21, VirtualKeyCode::OEM4);
    map.insert(0x1E, VirtualKeyCode::OEM6);
    map.insert(0x2A, VirtualKeyCode::OEM5);
    map.insert(0x29, VirtualKeyCode::OEM1);
    map.insert(0x27, VirtualKeyCode::OEM7);
    map.insert(0x2C, VirtualKeyCode::OEM2);
    map
});

// http://kbdlayout.info/kbdcz1/virtualkeys?arrangement=ISO105
pub(crate) static CZECH_QWERTY_ISO: LazyLock<HashMap<u16, VirtualKeyCode>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(0x0A, VirtualKeyCode::OEM3);
    map.insert(0x21, VirtualKeyCode::OEM4);
    map.insert(0x1E, VirtualKeyCode::OEM6);
    map.insert(0x29, VirtualKeyCode::OEM1);
    map.insert(0x27, VirtualKeyCode::OEM7);
    map.insert(0x2A, VirtualKeyCode::OEM5);
    map.insert(0x2C, VirtualKeyCode::OEM2);
    map.insert(0x32, VirtualKeyCode::OEM102);
    map
});
