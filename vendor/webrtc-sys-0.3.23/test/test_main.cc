#include "benchmark_nvidia.h"
#include "benchmark_openh264.h"
#include "benchmark_vaapi.h"
#include "stdio.h"

int main(int argc, char** argv) {

  std::vector<Benchmark*> benchmarks;
  benchmarks.push_back(new NvidiaBenchmark());
  //benchmarks.push_back(new VaapiBenchmark());
  benchmarks.push_back(new OpenH264Benchmark());

  for (auto benchmark : benchmarks) {
    if (benchmark->IsSupported()) {
      benchmark->Perform();
    }
  }

  return 0;
}
