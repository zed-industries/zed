#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Unlike 0001's busy-loop counter, pacing here comes from the Arduino
// core's delay(500) call in the sketch, so every call toggles directly.
static mut LED_STATE: u8 = 0;

#[no_mangle]
pub extern "C" fn citadel_tick() -> u8 {
    unsafe {
        LED_STATE ^= 1;
        LED_STATE
    }
}
