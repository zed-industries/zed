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

echo "Building LiveKit WebRTC - Linux"
echo "Arch: $arch"
echo "Profile: $profile"

if [ ! -e "$(pwd)/depot_tools" ]
then
  git clone --depth 1 https://chromium.googlesource.com/chromium/tools/depot_tools.git
fi

export COMMAND_DIR=$(cd $(dirname $0); pwd)
export PATH="$(pwd)/depot_tools:$PATH"
export OUTPUT_DIR="$(pwd)/src/out-$arch-$profile"
export ARTIFACTS_DIR="$(pwd)/linux-$arch-$profile"

if [ ! -e "$(pwd)/src" ]
then
  gclient sync -D --no-history
fi

cd src
git apply "$COMMAND_DIR/patches/add_licenses.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn
git apply "$COMMAND_DIR/patches/ssl_verify_callback_with_native_handle.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn
git apply "$COMMAND_DIR/patches/add_deps.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn

# Disable CREL (compact relocations). Chromium's build enables experimental
# CREL via -Wa,--crel which causes segfaults on aarch64-linux (and is known
# broken on arm32 and s390x too).
# See: https://crbug.com/376278218
# See: https://github.com/zed-industries/zed/pull/51433#discussion_r2944567608
git -C build apply "$COMMAND_DIR/patches/disable_crel.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn

cd third_party

git apply "$COMMAND_DIR/patches/david_disable_gun_source_macro.patch" -v --ignore-space-change --ignore-whitespace --whitespace=nowarn

cd ../..

mkdir -p "$ARTIFACTS_DIR/lib"

python3 "./src/build/linux/sysroot_scripts/install-sysroot.py" --arch="$arch"

debug="false"
if [ "$profile" = "debug" ]; then
  debug="true"
fi

args="is_debug=$debug  \
  target_os=\"linux\" \
  target_cpu=\"$arch\" \
  rtc_enable_protobuf=false \
  treat_warnings_as_errors=false \
  use_custom_libcxx=false \
  use_llvm_libatomic=false \
  use_libcxx_modules=false \
  use_custom_libcxx_for_host=false \
  rtc_include_tests=false \
  rtc_build_tools=false \
  rtc_build_examples=false \
  rtc_libvpx_build_vp9=true \
  enable_libaom=true \
  is_component_build=false \
  enable_stripping=true \
  ffmpeg_branding=\"Chrome\" \
  rtc_use_h264=true \
  rtc_use_h265=true \
  rtc_use_pipewire=true \
  symbol_level=0 \
  enable_iterator_debugging=false \
  use_rtti=true \
  rtc_use_x11=true"

# generate ninja files
gn gen "$OUTPUT_DIR" --root="src" --args="${args}"

# build static library
ninja -C "$OUTPUT_DIR" :default

# make libwebrtc.a
# don't include nasm
ar -rc "$ARTIFACTS_DIR/lib/libwebrtc.a" `find "$OUTPUT_DIR/obj" -name '*.o' -not -path "*/third_party/nasm/*"`
src/third_party/llvm-build/Release+Asserts/bin/llvm-objcopy --redefine-syms="$COMMAND_DIR/boringssl_prefix_symbols.txt" "$ARTIFACTS_DIR/lib/libwebrtc.a"

python3 "./src/tools_webrtc/libs/generate_licenses.py" \
  --target :default "$OUTPUT_DIR" "$OUTPUT_DIR"

cp "$OUTPUT_DIR/obj/webrtc.ninja" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/obj/modules/desktop_capture/desktop_capture.ninja" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/args.gn" "$ARTIFACTS_DIR"
cp "$OUTPUT_DIR/LICENSE.md" "$ARTIFACTS_DIR"

cd src
find . -name "*.h" -print | cpio -pd "$ARTIFACTS_DIR/include"
find . -name "*.inc" -print | cpio -pd "$ARTIFACTS_DIR/include"
