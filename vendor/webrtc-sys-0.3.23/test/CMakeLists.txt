# Project-level configuration.
cmake_minimum_required(VERSION 3.10)
project(runner LANGUAGES CXX C)
enable_language(ASM)

set(BINARY_NAME "h264_benchmark")

# Set the C++ standard to C++23.
set(CMAKE_CXX_STANDARD 20)

set(CMAKE_BUILD_TYPE Debug)


add_definitions(-DWEBRTC_POSIX)
add_definitions(-DWEBRTC_LINUX)
add_definitions(-DUSE_UDEV)
add_definitions(-DUSE_AURA=1)
add_definitions(-DUSE_GLIB=1)
add_definitions(-DUSE_OZONE=1)
add_definitions(-D__STDC_CONSTANT_MACROS)
add_definitions(-D__STDC_FORMAT_MACROS)
add_definitions(-D_FILE_OFFSET_BITS=64)
add_definitions(-D_LARGEFILE_SOURCE)
add_definitions(-D_LARGEFILE64_SOURCE)
add_definitions(-D_GNU_SOURCE)
add_definitions(-D_LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_NONE)
add_definitions(-D_GLIBCXX_ASSERTIONS=1)
add_definitions(-DCR_CLANG_REVISION=\"llvmorg-19-init-8091-gab037c4f-1\")
add_definitions(-DCR_SYSROOT_KEY=20230611T210420Z-2)
add_definitions(-DDYNAMIC_ANNOTATIONS_ENABLED=1)
add_definitions(-DWEBRTC_ENABLE_PROTOBUF=0)
add_definitions(-DWEBRTC_STRICT_FIELD_TRIALS=0)
add_definitions(-DWEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE)
add_definitions(-DRTC_USE_LIBAOM_AV1_ENCODER)
add_definitions(-DRTC_ENABLE_VP9)
add_definitions(-DRTC_DAV1D_IN_INTERNAL_DECODER_FACTORY)
add_definitions(-DWEBRTC_HAVE_SCTP)
add_definitions(-DWEBRTC_USE_H264)
add_definitions(-DWEBRTC_ENABLE_LIBEVENT)
add_definitions(-DWEBRTC_LIBRARY_IMPL)
add_definitions(-DWEBRTC_ENABLE_AVX2)

include_directories(
  "${CMAKE_CURRENT_SOURCE_DIR}/../libwebrtc/linux-x64-debug/include"
  "${CMAKE_CURRENT_SOURCE_DIR}/../libwebrtc/linux-x64-release/include/third_party/abseil-cpp"
  "${CMAKE_CURRENT_SOURCE_DIR}/../libwebrtc/linux-x64-release/include/third_party/libyuv/include"
  "${CMAKE_CURRENT_SOURCE_DIR}/../src/nvidia/NvCodec/include"
  "${CMAKE_CURRENT_SOURCE_DIR}/../src/nvidia/NvCodec/NvCodec"
  "${CMAKE_CURRENT_SOURCE_DIR}/../src"
)

link_libraries(
  "${CMAKE_CURRENT_SOURCE_DIR}/../libwebrtc/linux-x64-release/lib/libwebrtc.a"
)

find_package (Threads)

add_executable(${BINARY_NAME}
  "test_main.cc"
  "benchmark.cc"
  "benchmark_openh264.cc"
  "video_source.cc"
  "fileutils.cc"
  "cpu/cpu_linux.cc"

  "benchmark_nvidia.cc"
  "../src/nvidia/NvCodec/NvCodec/NvDecoder/NvDecoder.cpp"
  "../src/nvidia/NvCodec/NvCodec/NvEncoder/NvEncoder.cpp"
  "../src/nvidia/NvCodec/NvCodec/NvEncoder/NvEncoderCuda.cpp"
  "../src/nvidia/h264_encoder_impl.cpp"
  "../src/nvidia/h264_decoder_impl.cpp"
  "../src/nvidia/nvidia_decoder_factory.cpp"
  "../src/nvidia/nvidia_encoder_factory.cpp"
  "../src/nvidia/cuda_context.cpp"
  "../src/nvidia/implib/libcuda.so.init.c"
  "../src/nvidia/implib/libcuda.so.tramp.S"
  "../src/nvidia/implib/libnvcuvid.so.init.c"
  "../src/nvidia/implib/libnvcuvid.so.tramp.S"

  "benchmark_vaapi.cc"
  "../src/vaapi/vaapi_display_drm.cpp"
  "../src/vaapi/vaapi_h264_encoder_wrapper.cpp"
  "../src/vaapi/vaapi_encoder_factory.cpp"
  "../src/vaapi/h264_encoder_impl.cpp"
  "../src/vaapi/implib/libva-drm.so.init.c"
  "../src/vaapi/implib/libva-drm.so.tramp.S"
  "../src/vaapi/implib/libva.so.init.c"
  "../src/vaapi/implib/libva.so.tramp.S"
)

target_link_libraries(${BINARY_NAME} ${CMAKE_THREAD_LIBS_INIT})
target_link_libraries(${BINARY_NAME} dl)
