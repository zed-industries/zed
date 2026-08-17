#!/bin/sh -e

BINARY=$1
shift

# Make sure to run the following to copy the test helper binary over.
# cargo run --target x86_64-linux-android --bin test
adb push "$BINARY" "/data/local/$BINARY"
adb shell "chmod 777 /data/local/$BINARY && env TEST_HELPER=/data/local/target/x86_64-linux-android/debug/test /data/local/$BINARY" "$@"
