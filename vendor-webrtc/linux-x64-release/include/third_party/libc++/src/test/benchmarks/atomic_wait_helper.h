//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_BENCHMARK_ATOMIC_WAIT_HELPER_H
#define TEST_BENCHMARK_ATOMIC_WAIT_HELPER_H

#include <atomic>
#include <chrono>
#include <exception>
#include <stop_token>
#include <thread>

struct HighPrioTask {
  sched_param param;
  pthread_attr_t attr_t;
  pthread_t thread;
  std::atomic_bool stopped{false};

  HighPrioTask(const HighPrioTask&) = delete;

  HighPrioTask() {
    pthread_attr_init(&attr_t);
    pthread_attr_setschedpolicy(&attr_t, SCHED_FIFO);
    param.sched_priority = sched_get_priority_max(SCHED_FIFO);
    pthread_attr_setschedparam(&attr_t, &param);
    pthread_attr_setinheritsched(&attr_t, PTHREAD_EXPLICIT_SCHED);

    auto thread_fun = [](void* arg) -> void* {
      auto* stop = reinterpret_cast<std::atomic_bool*>(arg);
      while (!stop->load(std::memory_order_relaxed)) {
        // spin
      }
      return nullptr;
    };

    if (pthread_create(&thread, &attr_t, thread_fun, &stopped) != 0) {
      throw std::runtime_error("failed to create thread");
    }
  }

  ~HighPrioTask() {
    stopped = true;
    pthread_attr_destroy(&attr_t);
    pthread_join(thread, nullptr);
  }
};

template <std::size_t N>
struct NumHighPrioTasks {
  static constexpr auto value = N;
};

template <std::size_t N>
struct NumWaitingThreads {
  static constexpr auto value = N;
};

template <std::size_t N>
struct NumberOfAtomics {
  static constexpr auto value = N;
};

struct KeepNotifying {
  template <class Atomic>
  static void notify(Atomic& a, std::stop_token st) {
    while (!st.stop_requested()) {
      a.fetch_add(1, std::memory_order_relaxed);
      a.notify_all();
    }
  }
};

template <std::size_t N>
struct NotifyEveryNus {
  template <class Atomic>
  static void notify(Atomic& a, std::stop_token st) {
    while (!st.stop_requested()) {
      auto start = std::chrono::system_clock::now();
      a.fetch_add(1, std::memory_order_relaxed);
      a.notify_all();
      while (std::chrono::system_clock::now() - start < std::chrono::microseconds{N}) {
      }
    }
  }
};

#endif // TEST_BENCHMARK_ATOMIC_WAIT_HELPER_H