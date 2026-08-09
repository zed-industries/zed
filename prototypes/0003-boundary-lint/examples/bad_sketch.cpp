#include <Arduino.h>

#define DOUBLE(x) ((x) * 2)

const int SENSOR_PIN = A0;
const int LED_PIN = 13;

void setup() {
    pinMode(LED_PIN, OUTPUT);
}

void loop() {
    int raw = analogRead(SENSOR_PIN);
    int out = raw * 2;

    if (out > 512) {
        digitalWrite(LED_PIN, HIGH);
    }

    for (int i = 0; i < 3; i++) {
        digitalWrite(LED_PIN, LOW);
    }

    while (raw > 1000) {
        raw--;
    }

    int state = digitalRead(LED_PIN) ? 1 : 0;
    digitalWrite(LED_PIN, state);
}
