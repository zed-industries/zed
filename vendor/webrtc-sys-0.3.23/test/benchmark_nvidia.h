#include "benchmark.h"
#include "nvidia/nvidia_encoder_factory.h"

class NvidiaBenchmark : public Benchmark {
 public:
  NvidiaBenchmark();
  NvidiaBenchmark(std::string name, std::string description);
  NvidiaBenchmark(std::string name,
               std::string description,
               std::string resultsFileName);

  ~NvidiaBenchmark() {}

  bool IsSupported() override {
    return webrtc::NvidiaVideoEncoderFactory::IsSupported();
  }

 protected:
  webrtc::VideoEncoder* GetNewEncoder(webrtc::Environment &env) override;

 private:
  std::unique_ptr<webrtc::VideoEncoder> _encoder;
  std::unique_ptr<webrtc::NvidiaVideoEncoderFactory> _factory;
};