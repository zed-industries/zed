#!/bin/sh

set -ex

os=$(uname -s)

if [[ "$os" == "Darwin" ]]; then
    host_tag="darwin-x86_64"
elif [[ "$os" == "CYGWIN"* ]]; then
    host_tag="windows-x86_64"
else
    host_tag="linux-x86_64"
fi

# Builds can be found at https://ci.android.com for the `ndk` component
build_id=$(grep -oP 'version = "\d\.\d.\d\+\K\d+' ./Cargo.toml)
# From https://cs.android.com/android/platform/superproject/main/+/main:development/python-packages/fetchartifact/fetchartifact/__init__.py
target="ndk"
artifact_name="ndk_platform.tar.bz2"
url="https://androidbuildinternal.googleapis.com/android/internal/build/v3/builds/$build_id/$target/attempts/latest/artifacts/$artifact_name/url"
echo "Downloading sysroot $build_id from $url"
curl -L $url -o "$artifact_name"
tar xvf "$artifact_name" "ndk/sysroot/usr/include"
sysroot="$PWD/ndk/sysroot/"
[ ! -d "$sysroot" ] && echo "Android sysroot $sysroot does not exist!" && exit 1

while read ARCH && read TARGET ; do
    bindgen wrapper.h -o src/ffi_$ARCH.rs \
        --blocklist-item 'JNI\w+' \
        --blocklist-item 'C?_?JNIEnv' \
        --blocklist-item '_?JavaVM' \
        --blocklist-item '_?j\w+' \
        --newtype-enum '\w+_(result|status)_t' \
        --newtype-enum 'ACameraDevice_request_template' \
        --newtype-enum 'ADataSpace' \
        --newtype-enum 'AHardwareBuffer_Format' \
        --newtype-enum 'AHardwareBuffer_UsageFlags' \
        --newtype-enum 'AHdrMetadataType' \
        --newtype-enum 'AIMAGE_FORMATS' \
        --newtype-enum 'AMediaDrmEventType' \
        --newtype-enum 'AMediaDrmKeyRequestType' \
        --newtype-enum 'AMediaDrmKeyType' \
        --newtype-enum 'AMediaKeyStatusType' \
        --newtype-enum 'AMidiDevice_Protocol' \
        --newtype-enum 'AMotionClassification' \
        --newtype-enum 'ANativeWindowTransform' \
        --newtype-enum 'ANativeWindow_ChangeFrameRateStrategy' \
        --newtype-enum 'ANativeWindow_FrameRateCompatibility' \
        --newtype-enum 'ANativeWindow_LegacyFormat' \
        --newtype-enum 'AndroidBitmapCompressFormat' \
        --newtype-enum 'AndroidBitmapFormat' \
        --newtype-enum 'AppendMode' \
        --newtype-enum 'DeviceTypeCode' \
        --newtype-enum 'DurationCode' \
        --newtype-enum 'FeatureLevelCode' \
        --newtype-enum 'FuseCode' \
        --newtype-enum 'HeapTaggingLevel' \
        --newtype-enum 'OperandCode' \
        --newtype-enum 'OperationCode' \
        --newtype-enum 'OutputFormat' \
        --newtype-enum 'PaddingCode' \
        --newtype-enum 'PreferenceCode' \
        --newtype-enum 'PriorityCode' \
        --newtype-enum 'ResNsendFlags' \
        --newtype-enum 'ResultCode' \
        --newtype-enum 'SeekMode' \
        --newtype-enum 'acamera_\w+' \
        --newtype-enum 'android_LogPriority' \
        --newtype-enum 'android_fdsan_error_level' \
        --newtype-enum 'android_fdsan_owner_type' \
        --newtype-enum 'cryptoinfo_mode_t' \
        --newtype-enum 'log_id' \
        -- \
        --sysroot="$sysroot" --target=$TARGET
done << EOF
arm
arm-linux-androideabi
aarch64
aarch64-linux-android
i686
i686-linux-android
x86_64
x86_64-linux-android
EOF
