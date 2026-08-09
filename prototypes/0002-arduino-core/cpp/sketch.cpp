#include <Arduino.h>

const int LED_PIN = LED_BUILTIN;

extern "C" uint8_t citadel_tick(void); // logic lives in Rust

void setup() {
    pinMode(LED_PIN, OUTPUT);
}

void loop() {
    digitalWrite(LED_PIN, citadel_tick());
    delay(500); // core-provided delay; no branch, no computed intermediate
}
