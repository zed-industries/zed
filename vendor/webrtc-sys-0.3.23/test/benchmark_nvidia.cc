#include "benchmark_nvidia.h"

#include "api/environment/environment_factory.h"
#include "modules/video_coding/codecs/h264/include/h264.h"
#include "fileutils.h"

using namespace webrtc;

NvidiaBenchmark::NvidiaBenchmark()
    : Benchmark("NvidiaBenchmark",
                "Nvidia benchmark over a range of test cases",
                webrtc::test::OutputPath() + "NvidiaBenchmark.txt",
                "nvidia_bitstream_output.h264") {}

NvidiaBenchmark::NvidiaBenchmark(std::string name, std::string description)
    : Benchmark(name,
                description,
                webrtc::test::OutputPath() + "NvidiaBenchmark.txt",
                "nvidia_bitstream_output.h264") {}

NvidiaBenchmark::NvidiaBenchmark(std::string name,
                                 std::string description,
                                 std::string resultsFileName)
    : Benchmark(name, description, resultsFileName, "nvidia_bitstream_output.h264") {}


VideoEncoder* NvidiaBenchmark::GetNewEncoder(webrtc::Environment &env) {
  if (!NvidiaVideoEncoderFactory::IsSupported()) {
    fprintf(stderr, "NVIDIA is not supported on this system.\n");
    return nullptr;
  }

  if (!_factory) {
    _factory = std::make_unique<NvidiaVideoEncoderFactory>();
  }
  std::map<std::string, std::string> baselineParameters = {
      {"profile-level-id", "42e01f"},
      {"level-asymmetry-allowed", "1"},
      {"packetization-mode", "1"},
  };
  auto format = SdpVideoFormat("H264", baselineParameters);

  auto enc = _factory->Create(env, format);
  if (!enc) {
    fprintf(stderr, "Failed to create H264 encoder.\n");
    return nullptr;
  }
  _encoder = std::move(enc);

  return _encoder.get();
}