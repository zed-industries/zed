//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_BENCHMARKS_CONTAINERS_ASSOCIATIVE_CONTAINER_BENCHMARKS_H
#define TEST_BENCHMARKS_CONTAINERS_ASSOCIATIVE_CONTAINER_BENCHMARKS_H

#include <algorithm>
#include <iterator>
#include <random>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "benchmark/benchmark.h"
#include "../../GenerateInput.h"

namespace support {

template <class Container>
struct adapt_operations {
  // using ValueType = ...;
  // using KeyType   = ...;
  // static ValueType value_from_key(KeyType const& k);
  // static KeyType key_from_value(ValueType const& value);

  // using InsertionResult = ...;
  // static Container::iterator get_iterator(InsertionResult const&);
};

template <class Container>
void associative_container_benchmarks(std::string container) {
  using Key   = typename Container::key_type;
  using Value = typename Container::value_type;

  auto generate_unique_keys = [=](std::size_t n) {
    std::set<Key> keys;
    while (keys.size() < n) {
      Key k = Generate<Key>::random();
      keys.insert(k);
    }
    return std::vector<Key>(keys.begin(), keys.end());
  };

  auto make_value_types = [](std::vector<Key> const& keys) {
    std::vector<Value> kv;
    for (Key const& k : keys)
      kv.push_back(adapt_operations<Container>::value_from_key(k));
    return kv;
  };

  auto get_key = [](Value const& v) { return adapt_operations<Container>::key_from_value(v); };

  auto bench = [&](std::string operation, auto f) {
    benchmark::RegisterBenchmark(container + "::" + operation, f)->Arg(32)->Arg(1024)->Arg(8192);
  };

  static constexpr bool is_multi_key_container =
      !std::is_same_v<typename adapt_operations<Container>::InsertionResult,
                      std::pair<typename Container::iterator, bool>>;

  static constexpr bool is_ordered_container = requires(Container c, Key k) { c.lower_bound(k); };

  // These benchmarks are structured to perform the operation being benchmarked
  // a small number of times at each iteration, in order to offset the cost of
  // PauseTiming() and ResumeTiming().
  static constexpr std::size_t BatchSize = 32;

  struct alignas(Container) ScratchSpace {
    char storage[sizeof(Container)];
  };

  /////////////////////////
  // Constructors
  /////////////////////////
  bench("ctor(const&)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));
    Container src(in.begin(), in.end());
    ScratchSpace c[BatchSize];

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        new (c + i) Container(src);
        benchmark::DoNotOptimize(c + i);
        benchmark::ClobberMemory();
      }

      st.PauseTiming();
      for (std::size_t i = 0; i != BatchSize; ++i) {
        reinterpret_cast<Container*>(c + i)->~Container();
      }
      st.ResumeTiming();
    }
  });

  bench("ctor(iterator, iterator) (unsorted sequence)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::mt19937 randomness;
    std::vector<Key> keys = generate_unique_keys(size);
    std::shuffle(keys.begin(), keys.end(), randomness);
    std::vector<Value> in = make_value_types(keys);
    ScratchSpace c[BatchSize];

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        new (c + i) Container(in.begin(), in.end());
        benchmark::DoNotOptimize(c + i);
        benchmark::ClobberMemory();
      }

      st.PauseTiming();
      for (std::size_t i = 0; i != BatchSize; ++i) {
        reinterpret_cast<Container*>(c + i)->~Container();
      }
      st.ResumeTiming();
    }
  });

  bench("ctor(iterator, iterator) (sorted sequence)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Key> keys  = generate_unique_keys(size);
    std::sort(keys.begin(), keys.end());
    std::vector<Value> in = make_value_types(keys);
    ScratchSpace c[BatchSize];

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        new (c + i) Container(in.begin(), in.end());
        benchmark::DoNotOptimize(c + i);
        benchmark::ClobberMemory();
      }

      st.PauseTiming();
      for (std::size_t i = 0; i != BatchSize; ++i) {
        reinterpret_cast<Container*>(c + i)->~Container();
      }
      st.ResumeTiming();
    }
  });

  /////////////////////////
  // Assignment
  /////////////////////////
  bench("operator=(const&)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));
    Container src(in.begin(), in.end());
    Container c[BatchSize];

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        c[i] = src;
        benchmark::DoNotOptimize(c[i]);
        benchmark::ClobberMemory();
      }

      st.PauseTiming();
      for (std::size_t i = 0; i != BatchSize; ++i) {
        c[i].clear();
      }
      st.ResumeTiming();
    }
  });

  /////////////////////////
  // Insertion
  /////////////////////////
  bench("insert(value) (already present)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));
    Value to_insert        = in[in.size() / 2]; // pick any existing value
    std::vector<Container> c(BatchSize, Container(in.begin(), in.end()));
    typename Container::iterator inserted[BatchSize];

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        inserted[i] = adapt_operations<Container>::get_iterator(c[i].insert(to_insert));
        benchmark::DoNotOptimize(inserted[i]);
        benchmark::DoNotOptimize(c[i]);
        benchmark::ClobberMemory();
      }

      if constexpr (is_multi_key_container) {
        st.PauseTiming();
        for (std::size_t i = 0; i != BatchSize; ++i) {
          c[i].erase(inserted[i]);
        }
        st.ResumeTiming();
      }
    }
  });

  bench("insert(value) (new value)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size + 1));
    Value to_insert        = in.back();
    in.pop_back();
    std::vector<Container> c(BatchSize, Container(in.begin(), in.end()));

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        auto result = c[i].insert(to_insert);
        benchmark::DoNotOptimize(result);
        benchmark::DoNotOptimize(c[i]);
        benchmark::ClobberMemory();
      }

      st.PauseTiming();
      for (std::size_t i = 0; i != BatchSize; ++i) {
        c[i].erase(get_key(to_insert));
      }
      st.ResumeTiming();
    }
  });

  // The insert(hint, ...) methods are only relevant for ordered containers, and we lack
  // a good way to compute a hint for unordered ones.
  if constexpr (is_ordered_container) {
    bench("insert(hint, value) (good hint)", [=](auto& st) {
      const std::size_t size = st.range(0);
      std::vector<Value> in  = make_value_types(generate_unique_keys(size + 1));
      Value to_insert        = in.back();
      in.pop_back();

      std::vector<Container> c(BatchSize, Container(in.begin(), in.end()));
      typename Container::iterator hints[BatchSize];
      for (std::size_t i = 0; i != BatchSize; ++i) {
        hints[i] = c[i].lower_bound(get_key(to_insert));
      }

      while (st.KeepRunningBatch(BatchSize)) {
        for (std::size_t i = 0; i != BatchSize; ++i) {
          auto result = c[i].insert(hints[i], to_insert);
          benchmark::DoNotOptimize(result);
          benchmark::DoNotOptimize(c[i]);
          benchmark::ClobberMemory();
        }

        st.PauseTiming();
        for (std::size_t i = 0; i != BatchSize; ++i) {
          c[i].erase(get_key(to_insert));
          hints[i] = c[i].lower_bound(get_key(to_insert)); // refresh hints in case of invalidation
        }
        st.ResumeTiming();
      }
    });

    bench("insert(hint, value) (bad hint)", [=](auto& st) {
      const std::size_t size = st.range(0);
      std::vector<Value> in  = make_value_types(generate_unique_keys(size + 1));
      Value to_insert        = in.back();
      in.pop_back();
      std::vector<Container> c(BatchSize, Container(in.begin(), in.end()));

      while (st.KeepRunningBatch(BatchSize)) {
        for (std::size_t i = 0; i != BatchSize; ++i) {
          auto result = c[i].insert(c[i].begin(), to_insert);
          benchmark::DoNotOptimize(result);
          benchmark::DoNotOptimize(c[i]);
          benchmark::ClobberMemory();
        }

        st.PauseTiming();
        for (std::size_t i = 0; i != BatchSize; ++i) {
          c[i].erase(get_key(to_insert));
        }
        st.ResumeTiming();
      }
    });
  }

  bench("insert(iterator, iterator) (all new keys)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size + (size / 10)));

    // Populate a container with a small number of elements, that's what containers will start with.
    std::vector<Value> small;
    for (std::size_t i = 0; i != (size / 10); ++i) {
      small.push_back(in.back());
      in.pop_back();
    }
    Container c(small.begin(), small.end());

    for ([[maybe_unused]] auto _ : st) {
      c.insert(in.begin(), in.end());
      benchmark::DoNotOptimize(c);
      benchmark::ClobberMemory();

      st.PauseTiming();
      c = Container(small.begin(), small.end());
      st.ResumeTiming();
    }
  });

  bench("insert(iterator, iterator) (half new keys)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));

    // Populate a container that already contains half the elements we'll try inserting,
    // that's what our container will start with.
    std::vector<Value> small;
    for (std::size_t i = 0; i != size / 2; ++i) {
      small.push_back(in.at(i * 2));
    }
    Container c(small.begin(), small.end());

    for ([[maybe_unused]] auto _ : st) {
      c.insert(in.begin(), in.end());
      benchmark::DoNotOptimize(c);
      benchmark::ClobberMemory();

      st.PauseTiming();
      c = Container(small.begin(), small.end());
      st.ResumeTiming();
    }
  });

  /////////////////////////
  // Erasure
  /////////////////////////
  bench("erase(key) (existent)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));
    Value element          = in[in.size() / 2]; // pick any element
    std::vector<Container> c(BatchSize, Container(in.begin(), in.end()));

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        auto result = c[i].erase(get_key(element));
        benchmark::DoNotOptimize(result);
        benchmark::DoNotOptimize(c[i]);
        benchmark::ClobberMemory();
      }

      st.PauseTiming();
      for (std::size_t i = 0; i != BatchSize; ++i) {
        c[i].insert(element);
      }
      st.ResumeTiming();
    }
  });

  bench("erase(key) (non-existent)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size + BatchSize));
    std::vector<Key> keys;
    for (std::size_t i = 0; i != BatchSize; ++i) {
      keys.push_back(get_key(in.back()));
      in.pop_back();
    }
    Container c(in.begin(), in.end());

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        auto result = c.erase(keys[i]);
        benchmark::DoNotOptimize(result);
        benchmark::DoNotOptimize(c);
        benchmark::ClobberMemory();
      }

      // no cleanup required because we erased a non-existent element
    }
  });

  bench("erase(iterator)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));
    Value element          = in[in.size() / 2]; // pick any element

    std::vector<Container> c;
    std::vector<typename Container::iterator> iterators;
    for (std::size_t i = 0; i != BatchSize; ++i) {
      c.push_back(Container(in.begin(), in.end()));
      iterators.push_back(c[i].find(get_key(element)));
    }

    while (st.KeepRunningBatch(BatchSize)) {
      for (std::size_t i = 0; i != BatchSize; ++i) {
        auto result = c[i].erase(iterators[i]);
        benchmark::DoNotOptimize(result);
        benchmark::DoNotOptimize(c[i]);
        benchmark::ClobberMemory();
      }

      st.PauseTiming();
      for (std::size_t i = 0; i != BatchSize; ++i) {
        iterators[i] = adapt_operations<Container>::get_iterator(c[i].insert(element));
      }
      st.ResumeTiming();
    }
  });

  bench("erase(iterator, iterator) (erase half the container)", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));
    Container c(in.begin(), in.end());

    auto first = std::next(c.begin(), c.size() / 4);
    auto last  = std::next(c.begin(), 3 * (c.size() / 4));
    for ([[maybe_unused]] auto _ : st) {
      auto result = c.erase(first, last);
      benchmark::DoNotOptimize(result);
      benchmark::DoNotOptimize(c);
      benchmark::ClobberMemory();

      st.PauseTiming();
      c     = Container(in.begin(), in.end());
      first = std::next(c.begin(), c.size() / 4);
      last  = std::next(c.begin(), 3 * (c.size() / 4));
      st.ResumeTiming();
    }
  });

  bench("clear()", [=](auto& st) {
    const std::size_t size = st.range(0);
    std::vector<Value> in  = make_value_types(generate_unique_keys(size));
    Container c(in.begin(), in.end());

    for ([[maybe_unused]] auto _ : st) {
      c.clear();
      benchmark::DoNotOptimize(c);
      benchmark::ClobberMemory();

      st.PauseTiming();
      c = Container(in.begin(), in.end());
      st.ResumeTiming();
    }
  });

  /////////////////////////
  // Query
  /////////////////////////
  auto with_existent_key = [=](auto func) {
    return [=](auto& st) {
      const std::size_t size = st.range(0);
      std::vector<Value> in  = make_value_types(generate_unique_keys(size));
      // Pick any `BatchSize` number of elements
      std::vector<Key> keys;
      for (std::size_t i = 0; i < in.size(); i += (in.size() / BatchSize)) {
        keys.push_back(get_key(in.at(i)));
      }
      Container c(in.begin(), in.end());

      while (st.KeepRunningBatch(BatchSize)) {
        for (std::size_t i = 0; i != BatchSize; ++i) {
          auto result = func(c, keys[i]);
          benchmark::DoNotOptimize(c);
          benchmark::DoNotOptimize(result);
          benchmark::ClobberMemory();
        }
      }
    };
  };

  auto with_nonexistent_key = [=](auto func) {
    return [=](auto& st) {
      const std::size_t size = st.range(0);
      std::vector<Value> in  = make_value_types(generate_unique_keys(size + BatchSize));
      std::vector<Key> keys;
      for (std::size_t i = 0; i != BatchSize; ++i) {
        keys.push_back(get_key(in.back()));
        in.pop_back();
      }
      Container c(in.begin(), in.end());

      while (st.KeepRunningBatch(BatchSize)) {
        for (std::size_t i = 0; i != BatchSize; ++i) {
          auto result = func(c, keys[i]);
          benchmark::DoNotOptimize(c);
          benchmark::DoNotOptimize(result);
          benchmark::ClobberMemory();
        }
      }
    };
  };

  auto find = [](Container const& c, Key const& key) { return c.find(key); };
  bench("find(key) (existent)", with_existent_key(find));
  bench("find(key) (non-existent)", with_nonexistent_key(find));

  auto count = [](Container const& c, Key const& key) { return c.count(key); };
  bench("count(key) (existent)", with_existent_key(count));
  bench("count(key) (non-existent)", with_nonexistent_key(count));

  auto contains = [](Container const& c, Key const& key) { return c.contains(key); };
  bench("contains(key) (existent)", with_existent_key(contains));
  bench("contains(key) (non-existent)", with_nonexistent_key(contains));

  if constexpr (is_ordered_container) {
    auto lower_bound = [](Container const& c, Key const& key) { return c.lower_bound(key); };
    bench("lower_bound(key) (existent)", with_existent_key(lower_bound));
    bench("lower_bound(key) (non-existent)", with_nonexistent_key(lower_bound));

    auto upper_bound = [](Container const& c, Key const& key) { return c.upper_bound(key); };
    bench("upper_bound(key) (existent)", with_existent_key(upper_bound));
    bench("upper_bound(key) (non-existent)", with_nonexistent_key(upper_bound));

    auto equal_range = [](Container const& c, Key const& key) { return c.equal_range(key); };
    bench("equal_range(key) (existent)", with_existent_key(equal_range));
    bench("equal_range(key) (non-existent)", with_nonexistent_key(equal_range));
  }
}

} // namespace support

#endif // TEST_BENCHMARKS_CONTAINERS_ASSOCIATIVE_CONTAINER_BENCHMARKS_H
