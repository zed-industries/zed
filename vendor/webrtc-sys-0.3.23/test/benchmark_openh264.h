#include "benchmark.h"

class OpenH264Benchmark : public Benchmark {
 public:
  OpenH264Benchmark();
  OpenH264Benchmark(std::string name, std::string description);
  OpenH264Benchmark(std::string name,
               std::string description,
               std::string resultsFileName);

  ~OpenH264Benchmark() {}

  bool IsSupported() override {
    return true;
  }

 protected:
  webrtc::VideoEncoder* GetNewEncoder(webrtc::Environment &env) override;

 private:
  std::unique_ptr<webrtc::VideoEncoder> _encoder;
};