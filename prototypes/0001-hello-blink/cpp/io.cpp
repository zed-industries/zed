#include <avr/io.h>

extern "C" uint8_t citadel_tick(void);

extern "C" void citadel_setup(void) {
    DDRB = (1 << DDB5);
}

extern "C" void citadel_loop(void) {
    PORTB = citadel_tick() << DDB5;
}
