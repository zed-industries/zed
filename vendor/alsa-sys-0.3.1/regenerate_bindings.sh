#!/bin/sh
bindgen --size_t-is-usize --no-recursive-whitelist --no-prepend-enum-name --no-layout-tests \
--generate "functions,types" --whitelist-function="snd_.*" --whitelist-type="_?snd_.*" \
/usr/include/alsa/asoundlib.h -o src/generated.rs
