// This is a translation of the xkbcommon quick start guide:
// https://xkbcommon.org/doc/current/md_doc_quick_guide.html

extern crate evdev;
extern crate xkbcommon;

use xkbcommon::xkb;

// evdev constants:
const KEYCODE_OFFSET: u16 = 8;
const KEY_STATE_RELEASE: i32 = 0;
const KEY_STATE_REPEAT: i32 = 2;

fn main() {
    // Open evdev device
    let mut device = evdev::Device::open(
        std::env::args()
            .nth(1)
            .unwrap_or(String::from("/dev/input/event0")),
    )
    .unwrap();

    // Create context
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    // Load keymap informations
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",                                          // rules
        "pc105",                                     // model
        "is",                                        // layout
        "dvorak",                                    // variant
        Some("terminate:ctrl_alt_bksp".to_string()), // options
        xkb::COMPILE_NO_FLAGS,
    )
    .unwrap();

    // Create the state tracker
    let mut state = xkb::State::new(&keymap);

    loop {
        for event in device.fetch_events().unwrap() {
            if let evdev::InputEventKind::Key(keycode) = event.kind() {
                let keycode = (keycode.0 + KEYCODE_OFFSET).into();

                //  Ask the keymap what to do with key-repeat event
                if event.value() == KEY_STATE_REPEAT && !keymap.key_repeats(keycode) {
                    continue;
                }
                print!("keycode {:?} ", keycode);

                // Get keysym
                let keysym = state.key_get_one_sym(keycode);
                print!("keysym: {} ", xkb::keysym_get_name(keysym));

                // Update state
                let _changes = if event.value() == KEY_STATE_RELEASE {
                    state.update_key(keycode, xkb::KeyDirection::Up)
                } else {
                    state.update_key(keycode, xkb::KeyDirection::Down)
                };

                // Inspect state
                if state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE) {
                    print!("Control ");
                }
                if state.led_name_is_active(xkb::LED_NAME_NUM) {
                    print!("NumLockLED");
                }

                println!();
            }
        }
    }
}
