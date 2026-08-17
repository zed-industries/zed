#include "benchmark_openh264.h"

#include "api/environment/environment_factory.h"
#include "modules/video_coding/codecs/h264/include/h264.h"
#include "fileutils.h"

using namespace webrtc;

OpenH264Benchmark::OpenH264Benchmark()
    : Benchmark("OpenH264Benchmark",
                "OpenH264 benchmark over a range of test cases",
                webrtc::test::OutputPath() + "OpenH264Benchmark.txt",
                "openh264_bitstream_output.h264") {}

OpenH264Benchmark::OpenH264Benchmark(std::string name, std::string description)
    : Benchmark(name,
                description,
                webrtc::test::OutputPath() + "OpenH264Benchmark.txt",
                "openh264_bitstream_output.h264") {}

OpenH264Benchmark::OpenH264Benchmark(std::string name,
                           std::string description,
                           std::string resultsFileName)
    : Benchmark(name, description, resultsFileName, "openh264_bitstream_output.h264") {}

VideoEncoder* OpenH264Benchmark::GetNewEncoder(webrtc::Environment &env) {
  auto enc = CreateH264Encoder(env);
  if (!enc) {
    fprintf(stderr, "Failed to create H264 encoder.\n");
    return nullptr;
  }
  _encoder = std::move(enc);

  return _encoder.get();
}