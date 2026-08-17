#include "benchmark.h"
#include "vaapi/vaapi_encoder_factory.h"

class VaapiBenchmark : public Benchmark {
 public:
  VaapiBenchmark();
  VaapiBenchmark(std::string name, std::string description);
  VaapiBenchmark(std::string name,
               std::string description,
               std::string resultsFileName);

  ~VaapiBenchmark() {}

  bool IsSupported() override {
    return webrtc::VAAPIVideoEncoderFactory::IsSupported();
  }

 protected:
  webrtc::VideoEncoder* GetNewEncoder(webrtc::Environment &env) override;

 private:
  std::unique_ptr<webrtc::VideoEncoder> _encoder;
  std::unique_ptr<webrtc::VAAPIVideoEncoderFactory> _factory;
};