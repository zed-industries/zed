#ifndef BENCHMARKS_CPP_FLATBUFFERS_FB_BENCH_H_
#define BENCHMARKS_CPP_FLATBUFFERS_FB_BENCH_H_

#include <cstdint>
#include <memory>

#include "benchmarks/cpp/bench.h"
#include "include/flatbuffers/flatbuffers.h"

struct StaticAllocator : public flatbuffers::Allocator {
  explicit StaticAllocator(uint8_t *buffer) : buffer_(buffer) {}

  uint8_t *allocate(size_t) override { return buffer_; }

  void deallocate(uint8_t *, size_t) override {}

  uint8_t *buffer_;
};

std::unique_ptr<Bench> NewFlatBuffersBench(
    int64_t initial_size = 1024, flatbuffers::Allocator *allocator = nullptr);

#endif  // BENCHMARKS_CPP_FLATBUFFERS_FB_BENCH_H_