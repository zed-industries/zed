// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_BENCHMARKS_CONTAINERS_SEQUENCE_SEQUENCE_CONTAINER_BENCHMARKS_H
#define TEST_BENCHMARKS_CONTAINERS_SEQUENCE_SEQUENCE_CONTAINER_BENCHMARKS_H

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <iterator>
#include <ranges> // for std::from_range
#include <string>
#include <type_traits>
#include <vector>

#include "benchmark/benchmark.h"
#include "test_iterators.h"
#include "../../GenerateInput.h"

namespace support {

template <class Container>
void DoNotOptimizeData(Container& c) {
  if constexpr (requires { c.data(); }) {
    benchmark::DoNotOptimize(c.data());
  } else {
    benchmark::DoNotOptimize(&c);
  }
}

template <class Container>
void sequence_container_benchmarks(std::string container) {
  using ValueType = typename Container::value_type;

  using Generator     = ValueType (*)();
  Generator cheap     = [] { return Generate<ValueType>::cheap(); };
  Generator expensive = [] { return Generate<ValueType>::expensive(); };
  auto tostr          = [&](Generator gen) -> std::string {
    return gen == cheap ? " (cheap elements)" : " (expensive elements)";
  };
  std::vector<Generator> generators;
  generators.push_back(cheap);
  if constexpr (!std::is_integral_v<ValueType>) {
    generators.push_back(expensive);
  }

  // Some of these benchmarks are structured to perform the operation being benchmarked
  // a small number of times at each iteration, in order to offset the cost of
  // PauseTiming() and ResumeTiming().
  static constexpr std::size_t BatchSize = 32;

  auto bench = [&](std::string operation, auto f) {
    benchmark::RegisterBenchmark(container + "::" + operation, f)->Arg(32)->Arg(1024)->Arg(8192);
  };

  /////////////////////////
  // Constructors
  /////////////////////////
  if constexpr (std::is_constructible_v<Container, std::size_t>) {
    // not all containers provide this constructor
    bench("ctor(size)", [](auto& st) {
      auto const size = st.range(0);

      for ([[maybe_unused]] auto _ : st) {
        Container c(size); // we assume the destructor doesn't dominate the benchmark
        DoNotOptimizeData(c);
      }
    });
  }

  for (auto gen : generators)
    bench("ctor(size, value_type)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      ValueType value = gen();
      benchmark::DoNotOptimize(value);

      for ([[maybe_unused]] auto _ : st) {
        Container c(size, value); // we assume the destructor doesn't dominate the benchmark
        DoNotOptimizeData(c);
      }
    });

  for (auto gen : generators)
    bench("ctor(Iterator, Iterator)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      std::vector<ValueType> in;
      std::generate_n(std::back_inserter(in), size, gen);
      const auto begin = in.begin();
      const auto end   = in.end();
      benchmark::DoNotOptimize(in);

      for ([[maybe_unused]] auto _ : st) {
        Container c(begin, end); // we assume the destructor doesn't dominate the benchmark
        DoNotOptimizeData(c);
      }
    });

#if defined(__cpp_lib_containers_ranges) && __cpp_lib_containers_ranges >= 202202L
  for (auto gen : generators)
    bench("ctor(Range)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      std::vector<ValueType> in;
      std::generate_n(std::back_inserter(in), size, gen);
      benchmark::DoNotOptimize(in);

      for ([[maybe_unused]] auto _ : st) {
        Container c(std::from_range, in); // we assume the destructor doesn't dominate the benchmark
        DoNotOptimizeData(c);
      }
    });
#endif

  for (auto gen : generators)
    bench("ctor(const&)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      Container in;
      std::generate_n(std::back_inserter(in), size, gen);
      DoNotOptimizeData(in);

      for ([[maybe_unused]] auto _ : st) {
        Container c(in); // we assume the destructor doesn't dominate the benchmark
        DoNotOptimizeData(c);
        DoNotOptimizeData(in);
      }
    });

  /////////////////////////
  // Assignment
  /////////////////////////
  for (auto gen : generators)
    bench("operator=(const&)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      Container in1, in2;
      std::generate_n(std::back_inserter(in1), size, gen);
      std::generate_n(std::back_inserter(in2), size, gen);
      DoNotOptimizeData(in1);
      DoNotOptimizeData(in2);

      // Assign from one of two containers in succession to avoid
      // hitting a self-assignment corner-case
      Container c(in1);
      bool toggle = false;
      for ([[maybe_unused]] auto _ : st) {
        c      = toggle ? in1 : in2;
        toggle = !toggle;
        DoNotOptimizeData(c);
        DoNotOptimizeData(in1);
        DoNotOptimizeData(in2);
      }
    });

  // Benchmark Container::assign(input-iter, input-iter) when the container already contains
  // the same number of elements that we're assigning. The intent is to check whether the
  // implementation basically creates a new container from scratch or manages to reuse the
  // pre-existing storage.
  for (auto gen : generators)
    bench("assign(input-iter, input-iter) (full container)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      std::vector<ValueType> in1, in2;
      std::generate_n(std::back_inserter(in1), size, gen);
      std::generate_n(std::back_inserter(in2), size, gen);
      DoNotOptimizeData(in1);
      DoNotOptimizeData(in2);

      Container c(in1.begin(), in1.end());
      bool toggle = false;
      for ([[maybe_unused]] auto _ : st) {
        std::vector<ValueType>& in = toggle ? in1 : in2;
        auto first                 = in.data();
        auto last                  = in.data() + in.size();
        c.assign(cpp17_input_iterator(first), cpp17_input_iterator(last));
        toggle = !toggle;
        DoNotOptimizeData(c);
      }
    });

  /////////////////////////
  // Insertion
  /////////////////////////
  for (auto gen : generators)
    bench("insert(begin)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      std::vector<ValueType> in;
      std::generate_n(std::back_inserter(in), size, gen);
      DoNotOptimizeData(in);

      Container c(in.begin(), in.end());
      DoNotOptimizeData(c);

      ValueType value = gen();
      benchmark::DoNotOptimize(value);

      for ([[maybe_unused]] auto _ : st) {
        c.insert(c.begin(), value);
        DoNotOptimizeData(c);

        c.erase(std::prev(c.end())); // avoid growing indefinitely
      }
    });

  if constexpr (std::random_access_iterator<typename Container::iterator>) {
    for (auto gen : generators)
      bench("insert(middle)" + tostr(gen), [gen](auto& st) {
        auto const size = st.range(0);
        std::vector<ValueType> in;
        std::generate_n(std::back_inserter(in), size, gen);
        DoNotOptimizeData(in);

        Container c(in.begin(), in.end());
        DoNotOptimizeData(c);

        ValueType value = gen();
        benchmark::DoNotOptimize(value);

        for ([[maybe_unused]] auto _ : st) {
          auto mid = c.begin() + (size / 2); // requires random-access iterators in order to make sense
          c.insert(mid, value);
          DoNotOptimizeData(c);

          c.erase(c.end() - 1); // avoid growing indefinitely
        }
      });
  }

  if constexpr (requires(Container c) { c.reserve(0); }) {
    // Insert at the start of a vector in a scenario where the vector already
    // has enough capacity to hold all the elements we are inserting.
    for (auto gen : generators)
      bench("insert(begin, input-iter, input-iter) (no realloc)" + tostr(gen), [gen](auto& st) {
        auto const size = st.range(0);
        std::vector<ValueType> in;
        std::generate_n(std::back_inserter(in), size, gen);
        DoNotOptimizeData(in);
        auto first = in.data();
        auto last  = in.data() + in.size();

        const int small = 100; // arbitrary
        Container c;
        c.reserve(size + small); // ensure no reallocation
        std::generate_n(std::back_inserter(c), small, gen);

        for ([[maybe_unused]] auto _ : st) {
          c.insert(c.begin(), cpp17_input_iterator(first), cpp17_input_iterator(last));
          DoNotOptimizeData(c);

          st.PauseTiming();
          c.erase(c.begin() + small, c.end()); // avoid growing indefinitely
          st.ResumeTiming();
        }
      });

    // Insert at the start of a vector in a scenario where the vector already
    // has almost enough capacity to hold all the elements we are inserting,
    // but does need to reallocate.
    for (auto gen : generators)
      bench("insert(begin, input-iter, input-iter) (half filled)" + tostr(gen), [gen](auto& st) {
        auto const size = st.range(0);
        std::vector<ValueType> in;
        std::generate_n(std::back_inserter(in), size, gen);
        DoNotOptimizeData(in);
        auto first = in.data();
        auto last  = in.data() + in.size();

        const int overflow = size / 10; // 10% of elements won't fit in the vector when we insert
        Container c;
        for ([[maybe_unused]] auto _ : st) {
          st.PauseTiming();
          c = Container();
          c.reserve(size);
          std::generate_n(std::back_inserter(c), overflow, gen);
          st.ResumeTiming();

          c.insert(c.begin(), cpp17_input_iterator(first), cpp17_input_iterator(last));
          DoNotOptimizeData(c);
        }
      });

    // Insert at the start of a vector in a scenario where the vector can fit a few
    // more elements, but needs to reallocate almost immediately to fit the remaining
    // elements.
    for (auto gen : generators)
      bench("insert(begin, input-iter, input-iter) (near full)" + tostr(gen), [gen](auto& st) {
        auto const size = st.range(0);
        std::vector<ValueType> in;
        std::generate_n(std::back_inserter(in), size, gen);
        DoNotOptimizeData(in);
        auto first = in.data();
        auto last  = in.data() + in.size();

        auto const overflow = 9 * (size / 10); // 90% of elements won't fit in the vector when we insert
        Container c;
        for ([[maybe_unused]] auto _ : st) {
          st.PauseTiming();
          c = Container();
          c.reserve(size);
          std::generate_n(std::back_inserter(c), overflow, gen);
          st.ResumeTiming();

          c.insert(c.begin(), cpp17_input_iterator(first), cpp17_input_iterator(last));
          DoNotOptimizeData(c);
        }
      });
  }

  /////////////////////////
  // Variations of push_back
  /////////////////////////
  static constexpr bool has_push_back = requires(Container c, ValueType v) { c.push_back(v); };
  static constexpr bool has_capacity  = requires(Container c) { c.capacity(); };
  static constexpr bool has_reserve   = requires(Container c) { c.reserve(0); };
  if constexpr (has_push_back) {
    if constexpr (has_capacity) {
      // For containers where we can observe capacity(), push_back a single element
      // without reserving to ensure the container needs to grow
      for (auto gen : generators)
        bench("push_back() (growing)" + tostr(gen), [gen](auto& st) {
          auto const size = st.range(0);
          std::vector<ValueType> in;
          std::generate_n(std::back_inserter(in), size, gen);
          DoNotOptimizeData(in);

          auto at_capacity = [](Container c) {
            while (c.size() < c.capacity())
              c.push_back(c.back());
            return c;
          };

          std::vector<Container> c(BatchSize, at_capacity(Container(in.begin(), in.end())));
          std::vector<Container> const original = c;

          while (st.KeepRunningBatch(BatchSize)) {
            for (std::size_t i = 0; i != BatchSize; ++i) {
              c[i].push_back(in[i]);
              DoNotOptimizeData(c[i]);
            }

            st.PauseTiming();
            for (std::size_t i = 0; i != BatchSize; ++i) {
              c[i] = at_capacity(Container(in.begin(), in.end()));
              assert(c[i].size() == c[i].capacity());
            }
            st.ResumeTiming();
          }
        });
    }

    // For containers where we can reserve, push_back a single element after reserving to
    // ensure the container doesn't grow
    if constexpr (has_reserve) {
      for (auto gen : generators)
        bench("push_back() (with reserve)" + tostr(gen), [gen](auto& st) {
          auto const size = st.range(0);
          std::vector<ValueType> in;
          std::generate_n(std::back_inserter(in), size, gen);
          DoNotOptimizeData(in);

          Container c(in.begin(), in.end());
          // Ensure the container has enough capacity
          c.reserve(c.size() + BatchSize);
          DoNotOptimizeData(c);

          while (st.KeepRunningBatch(BatchSize)) {
            for (std::size_t i = 0; i != BatchSize; ++i) {
              c.push_back(in[i]);
            }
            DoNotOptimizeData(c);

            st.PauseTiming();
            c.erase(c.end() - BatchSize, c.end());
            st.ResumeTiming();
          }
        });
    }

    // push_back many elements: this is amortized constant for std::vector but not all containers
    for (auto gen : generators)
      bench("push_back() (many elements)" + tostr(gen), [gen](auto& st) {
        auto const size = st.range(0);
        std::vector<ValueType> in;
        std::generate_n(std::back_inserter(in), size, gen);
        DoNotOptimizeData(in);

        Container c;
        DoNotOptimizeData(c);
        while (st.KeepRunningBatch(size)) {
          for (int i = 0; i != size; ++i) {
            c.push_back(in[i]);
          }
          DoNotOptimizeData(c);

          st.PauseTiming();
          c.clear();
          st.ResumeTiming();
        }
      });
  }

  /////////////////////////
  // Erasure
  /////////////////////////
  for (auto gen : generators)
    bench("erase(begin)" + tostr(gen), [gen](auto& st) {
      auto const size = st.range(0);
      std::vector<ValueType> in;
      std::generate_n(std::back_inserter(in), size, gen);
      DoNotOptimizeData(in);

      Container c(in.begin(), in.end());
      DoNotOptimizeData(c);

      ValueType value = gen();
      benchmark::DoNotOptimize(value);

      for ([[maybe_unused]] auto _ : st) {
        c.erase(c.begin());
        DoNotOptimizeData(c);

        c.insert(c.end(), value); // re-insert an element at the end to avoid needing a new container
      }
    });

  if constexpr (std::random_access_iterator<typename Container::iterator>) {
    for (auto gen : generators)
      bench("erase(middle)" + tostr(gen), [gen](auto& st) {
        auto const size = st.range(0);
        std::vector<ValueType> in;
        std::generate_n(std::back_inserter(in), size, gen);
        DoNotOptimizeData(in);

        Container c(in.begin(), in.end());
        DoNotOptimizeData(c);

        ValueType value = gen();
        benchmark::DoNotOptimize(value);

        for ([[maybe_unused]] auto _ : st) {
          auto mid = c.begin() + (size / 2);
          c.erase(mid);
          DoNotOptimizeData(c);

          c.insert(c.end(), value); // re-insert an element at the end to avoid needing a new container
        }
      });
  }
}

} // namespace support

#endif // TEST_BENCHMARKS_CONTAINERS_SEQUENCE_SEQUENCE_CONTAINER_BENCHMARKS_H
