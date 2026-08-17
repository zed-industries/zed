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
      if [ "$arch" != "arm" ] && [ "$arch" != "x64" ] && [ "$arch" != "arm64" ]; then
        echo "Error: Invalid value for --arch. Must be 'arm', 'x64' or 'arm64'."
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

echo "Building LiveKit WebRTC - Android"
echo "Arch: $arch"
echo "Profile: $profile"

if [ ! -e "$(pwd)/depot_tools" ]
then
  git clone --depth 1 https://chromium.googlesource.com/chromium/tools/depot_tools.git
fi

export COMMAND_DIR=$(cd $(dirname $0); pwd)
export PATH="$(pwd)/depot_tools:$PATH"
export OUTPUT_DIR="$(pwd)/src/out-$arch-$profile"
export ARTIFACTS_DIR="$(pwd)/android-$arch-$profile"

if [ ! -e "$(pwd)/src" ]
then
  gclient sync -D --no-history
fi

cd src
# git apply "$COMMAND_DIR/patches/add_licenses.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn
git apply "$COMMAND_DIR/patches/ssl_verify_callback_with_native_handle.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn
git apply "$COMMAND_DIR/patches/add_deps.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn
git apply "$COMMAND_DIR/patches/android_use_libunwind.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn

cd third_party/libyuv
git apply "$COMMAND_DIR/patches/disable_sme_for_libyuv.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn

cd ../../..

mkdir -p "$ARTIFACTS_DIR/lib"

debug="false"
if [ "$profile" = "debug" ]; then
  debug="true"
fi

args="is_debug=$debug \
  is_java_debug=$debug \
  target_os=\"android\" \
  target_cpu=\"$arch\" \
  rtc_enable_protobuf=false \
  treat_warnings_as_errors=false \
  use_custom_libcxx=false \
  rtc_include_tests=false \
  rtc_build_tools=false \
  rtc_build_examples=false \
  rtc_libvpx_build_vp9=false \
  is_component_build=false \
  enable_stripping=true \
  rtc_use_h264=false \
  rtc_use_h265=true \
  rtc_use_pipewire=false \
  symbol_level=0 \
  enable_iterator_debugging=false \
  use_rtti=true"

if [ "$debug" = "true" ]; then
  args="${args} is_asan=true is_lsan=true";
fi

# generate ninja files
gn gen "$OUTPUT_DIR" --root="src" --args="${args}"

# build shared library
autoninja -C "$OUTPUT_DIR" :default \
  sdk/android:native_api \
  sdk/android:libwebrtc \
  sdk/android:libjingle_peerconnection_so

# make libwebrtc.a
# don't include nasm
ar -rc "$ARTIFACTS_DIR/lib/libwebrtc.a" `find "$OUTPUT_DIR/obj" -name '*.o' -not -path "*/third_party/nasm/*"`

python3 "./src/tools_webrtc/libs/generate_licenses.py" \
  --target :default "$OUTPUT_DIR" "$OUTPUT_DIR"

cp "$OUTPUT_DIR/obj/webrtc.ninja" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/libjingle_peerconnection_so.so" "$ARTIFACTS_DIR/lib"
cp "$OUTPUT_DIR/args.gn" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/LICENSE.md" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/lib.java/sdk/android/libwebrtc.jar" "$ARTIFACTS_DIR"
cp "src/sdk/android/AndroidManifest.xml" "$ARTIFACTS_DIR"

cd src
find . -name "*.h" -print | cpio -pd "$ARTIFACTS_DIR/include"
find . -name "*.inc" -print | cpio -pd "$ARTIFACTS_DIR/include"
