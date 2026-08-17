#ifndef BENCHMARKS_CPP_BENCH_H_
#define BENCHMARKS_CPP_BENCH_H_

#include <cstdint>

struct Bench {
  virtual ~Bench() {}

  inline void Add(int64_t value) { sum += value; }

  virtual uint8_t *Encode(void *buf, int64_t &len) = 0;
  virtual void *Decode(void *buf, int64_t len) = 0;
  virtual int64_t Use(void *decoded) = 0;
  virtual void Dealloc(void *decoded) = 0;

  int64_t sum = 0;
};

#endif // BENCHMARKS_CPP_BENCH_H_