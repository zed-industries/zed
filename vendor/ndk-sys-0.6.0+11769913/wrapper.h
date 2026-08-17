#include <android/api-level.h>
#include <android/asset_manager.h>
#include <android/asset_manager_jni.h>
// #include <android/binder_auto_utils.h>
// #include <android/binder_enums.h>
// #include <android/binder_ibinder.h>
// #include <android/binder_ibinder_jni.h>
// #include <android/binder_interface_utils.h>
// #include <android/binder_internal_logging.h>
// #include <android/binder_parcelable_utils.h>
// #include <android/binder_parcel.h>
// #include <android/binder_parcel_jni.h>
// #include <android/binder_parcel_utils.h>
// #include <android/binder_status.h>
// #include <android/binder_to_string.h>
#include <android/bitmap.h>
#include <android/choreographer.h>
#include <android/configuration.h>
#include <android/data_space.h>
#include <android/dlext.h>
#include <android/fdsan.h>
#include <android/file_descriptor_jni.h>
#include <android/font.h>
#include <android/font_matcher.h>
// #include <android/hardware_buffer_aidl.h>
#include <android/hardware_buffer.h>
#include <android/hardware_buffer_jni.h>
#include <android/hdr_metadata.h>
#include <android/imagedecoder.h>
#include <android/input.h>
#include <android/keycodes.h>
// #include <android/legacy_stdlib_inlines.h>
// #include <android/legacy_termios_inlines.h>
// #include <android/legacy_threads_inlines.h>
// #include <android/legacy_unistd_inlines.h>
#include <android/log.h>
#include <android/looper.h>
#include <android/multinetwork.h>
#include <android/native_activity.h>
// #include <android/native_window_aidl.h>
#include <android/native_window.h>
#include <android/native_window_jni.h>
// Not available in nightly NDK CI builds
// #include <android/ndk-version.h>
#include <android/NeuralNetworks.h>
#include <android/NeuralNetworksTypes.h>
#include <android/obb.h>
#include <android/performance_hint.h>
#include <android/permission_manager.h>
// #include <android/persistable_bundle_aidl.h>
// #include <android/persistable_bundle.h>
#include <android/rect.h>
#include <android/sensor.h>
#include <android/set_abort_message.h>
#include <android/sharedmem.h>
#include <android/sharedmem_jni.h>
#include <android/storage_manager.h>
// #include <android/surface_control.h>
// #include <android/surface_control_jni.h>
#include <android/surface_texture.h>
#include <android/surface_texture_jni.h>
#include <android/sync.h>
#include <android/system_fonts.h>
// #include <android/thermal.h>
#include <android/trace.h>
#include <android/versioning.h>
#include <android/window.h>

#include <aaudio/AAudio.h>

#include <amidi/AMidi.h>

#include <camera/NdkCameraCaptureSession.h>
#include <camera/NdkCameraDevice.h>
#include <camera/NdkCameraError.h>
#include <camera/NdkCameraManager.h>
#include <camera/NdkCameraMetadata.h>
#include <camera/NdkCameraMetadataTags.h>
#include <camera/NdkCameraWindowType.h>
#include <camera/NdkCaptureRequest.h>

#include <media/NdkImage.h>
#include <media/NdkImageReader.h>
#include <media/NdkMediaCodec.h>
#include <media/NdkMediaCrypto.h>
#include <media/NdkMediaDataSource.h>
#include <media/NdkMediaDrm.h>
#include <media/NdkMediaError.h>
#include <media/NdkMediaExtractor.h>
#include <media/NdkMediaFormat.h>
#include <media/NdkMediaMuxer.h>
