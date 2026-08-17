#!/bin/bash
# Copyright 2023 LiveKit, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


arch=""
profile="release"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --arch)
      arch="$2"
      if [ "$arch" != "x64" ] && [ "$arch" != "arm64" ]; then
        echo "Error: Invalid value for --arch. Must be 'x64' or 'arm64'."
        exit 1
      fi
      shift 2
      ;;
    --profile)
      profile="$2"
      if [ "$profile" != "debug" ] && [ "$profile" != "release" ]; then
        echo "Error: Invalid value for --profile. Must be 'debug' or 'release'."
        exit 1
      fi
      shift 2
      ;;
    *)
      echo "Error: Unknown argument '$1'"
      exit 1
      ;;
  esac
done

if [ -z "$arch" ]; then
  echo "Error: --arch must be set."
  exit 1
fi

echo "Building LiveKit WebRTC - MacOS"
echo "Arch: $arch"
echo "Profile: $profile"

if [ ! -e "$(pwd)/depot_tools" ]
then
  git clone --depth 1 https://chromium.googlesource.com/chromium/tools/depot_tools.git
fi

export COMMAND_DIR=$(cd $(dirname $0); pwd)
export PATH="$(pwd)/depot_tools:$PATH"
export OUTPUT_DIR="$(pwd)/src/out-$arch-$profile"
export ARTIFACTS_DIR="$(pwd)/mac-$arch-$profile"

if [ ! -e "$(pwd)/src" ]
then
  gclient sync -D --no-history
fi

cd src
git apply "$COMMAND_DIR/patches/add_licenses.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn
git apply "$COMMAND_DIR/patches/ssl_verify_callback_with_native_handle.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn
git apply "$COMMAND_DIR/patches/add_deps.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn

cd ..

mkdir -p "$ARTIFACTS_DIR/lib"

debug="false"
if [ "$profile" = "debug" ]; then
  debug="true"
fi

# generate ninja files
gn gen "$OUTPUT_DIR" --root="src" \
  --args="is_debug=$debug \
  enable_dsyms=$debug \
  target_os=\"mac\" \
  target_cpu=\"$arch\" \
  mac_deployment_target=\"10.15\" \
  treat_warnings_as_errors=false \
  rtc_enable_protobuf=false \
  rtc_include_tests=false \
  rtc_build_examples=false \
  rtc_build_tools=false \
  rtc_libvpx_build_vp9=true \
  enable_libaom=true \
  is_component_build=false \
  enable_stripping=true \
  rtc_enable_symbol_export=true \
  rtc_enable_objc_symbol_export=false \
  rtc_include_dav1d_in_internal_decoder_factory = true \
  rtc_use_h264=true \
  rtc_use_h265=true \
  use_custom_libcxx=false \
  clang_use_chrome_plugins=false \
  use_rtti=true \
  use_lld=false"

# build static library
ninja -C "$OUTPUT_DIR" :default \
  api/audio_codecs:builtin_audio_decoder_factory \
  api/task_queue:default_task_queue_factory \
  sdk:native_api \
  sdk:default_codec_factory_objc \
  pc:peer_connection \
  sdk:videocapture_objc \
  sdk:mac_framework_objc \
  desktop_capture_objc

# make libwebrtc.a
# don't include nasm
ar -rc "$ARTIFACTS_DIR/lib/libwebrtc.a" `find "$OUTPUT_DIR/obj" -name '*.o' -not -path "*/third_party/nasm/*"`

python3 "./src/tools_webrtc/libs/generate_licenses.py" \
  --target :webrtc "$OUTPUT_DIR" "$OUTPUT_DIR"

cp "$OUTPUT_DIR/obj/webrtc.ninja" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/obj/modules/desktop_capture/desktop_capture.ninja" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/args.gn" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/LICENSE.md" "$ARTIFACTS_DIR"

cd src
find . -name "*.h" -print | cpio -pd "$ARTIFACTS_DIR/include"
find . -name "*.inc" -print | cpio -pd "$ARTIFACTS_DIR/include"
