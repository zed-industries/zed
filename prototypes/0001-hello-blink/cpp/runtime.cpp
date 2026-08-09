#include <avr/io.h>

extern "C" void citadel_setup(void);
extern "C" void citadel_loop(void);

int main(void) {
    citadel_setup();
    for (;;) {
        citadel_loop();
    }
}
