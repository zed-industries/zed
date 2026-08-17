#include "benchmark_vaapi.h"

#include "api/environment/environment_factory.h"
#include "modules/video_coding/codecs/h264/include/h264.h"
#include "fileutils.h"

using namespace webrtc;

VaapiBenchmark::VaapiBenchmark()
    : Benchmark("VaapiBenchmark",
                "VAAPI benchmark over a range of test cases",
                webrtc::test::OutputPath() + "VaapiBenchmark.txt",
                "vaapi_bitstream_output.h264") {}

VaapiBenchmark::VaapiBenchmark(std::string name, std::string description)
    : Benchmark(name,
                description,
                webrtc::test::OutputPath() + "VaapiBenchmark.txt",
                "vaapi_bitstream_output.h264") {}

VaapiBenchmark::VaapiBenchmark(std::string name,
                               std::string description,
                               std::string resultsFileName)
    : Benchmark(name, description, resultsFileName, "vaapi_bitstream_output.h264") {}

VideoEncoder* VaapiBenchmark::GetNewEncoder(webrtc::Environment &env) {
  if (!VAAPIVideoEncoderFactory::IsSupported()) {
    fprintf(stderr, "VAAPI is not supported on this system.\n");
    return nullptr;
  }
  if (!_factory) {
    _factory = std::make_unique<VAAPIVideoEncoderFactory>();
  }
  std::map<std::string, std::string> baselineParameters = {
      {"profile-level-id", "4d0032"},
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