#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const TOGGLE_EVERY: u32 = 50_000;

static mut TICK_COUNT: u32 = 0;
static mut LED_STATE: u8 = 0;

#[no_mangle]
pub extern "C" fn citadel_tick() -> u8 {
    unsafe {
        TICK_COUNT += 1;
        if TICK_COUNT >= TOGGLE_EVERY {
            TICK_COUNT = 0;
            LED_STATE ^= 1;
        }
        LED_STATE
    }
}
