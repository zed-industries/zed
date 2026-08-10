#include <Arduino.h>

#define DOUBLE(x) ((x) * 2)
#define BLINK_IF_HOT digitalWrite(13, HIGH); if (analogRead(0) > 512) { digitalWrite(13, LOW); }

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

    switch (raw) {
        case 0:
            digitalWrite(LED_PIN, LOW);
            break;
        default:
            digitalWrite(LED_PIN, HIGH);
            break;
    }

    int state = digitalRead(LED_PIN) ? 1 : 0;
    digitalWrite(LED_PIN, state);

    BLINK_IF_HOT;
}
