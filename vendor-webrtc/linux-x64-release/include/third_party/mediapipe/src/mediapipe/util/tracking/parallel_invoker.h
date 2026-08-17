// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Parallel for loop execution.
// For details adapt parallel_using_* flags defined in parallel_invoker.cc.

// Usage example (for 1D):

// Define Functor or lambda function that implements:
// void operator()(const BlockedRange & range) const;
// (in addition functor needs to be copyable).

// Execute a for loop in parallel from 0 to N via:
// ParallelFor(0,              // start_index
//             num_frames,     // end_index, exclusive
//             1               // number of elements processed per iteration
//             [](const BlockedRange& range) {
//     // Process per-thread sub-range
//     for (int i = range.begin(); i < range.end(); ++i) {
//       // Process i'th item.
//     }
//  }

// Specific implementation to copy a vector of images in parallel.
// class CopyInvoker {
//  public:
//   CopyInvoker(const vector<cv::Mat>& inputs,
//               vector<cv::Mat*>* outputs)
//       : inputs_(inputs), outputs_(outputs) {
//   }
//   CopyInvoker(const CopyInvoker& rhs)
//       : inputs_(rhs.inputs_), outputs_(rhs.outputs) {
//   }
//   void operator()(const BlockedRange& range) {
//     for (int frame = range.begin(); frame < range.end(); ++frame) {
//       inputs_[frame].copyTo(*(*outputs_)[frame]);
//     }
//   }
//  private:
//   const vector<cv::Mat>& inputs_;
//   vector<cv::Mat*>* outputs_;
// }

// vector<cv::Mat> inputs;
// vector<cv::Mat*> outputs;
// ParallelFor(0, num_frames, 1, CopyInvoker(inputs, &outputs));
//
// OR (with lambdas):
// ParallelFor(0, num_frames, 1,
//             [&inputs, &outputs](const BlockedRange& range) {
//     for (int frame = range.begin(); frame < range.end(); ++frame) {
//       inputs[frame].copyTo(*(outputs)[frame]);
//     }
// }

#ifndef MEDIAPIPE_UTIL_TRACKING_PARALLEL_INVOKER_H_
#define MEDIAPIPE_UTIL_TRACKING_PARALLEL_INVOKER_H_

#include <stddef.h>

#include <memory>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/synchronization/mutex.h"

#ifdef PARALLEL_INVOKER_ACTIVE
#include "mediapipe/framework/port/threadpool.h"

#ifdef __APPLE__
#include <dispatch/dispatch.h>
#include <stdatomic.h>
#endif

#endif  // PARALLEL_INVOKER_ACTIVE

// Specifies parallelization implementation to use.
enum PARALLEL_INVOKER_MODE {
  PARALLEL_INVOKER_NONE = 0,         // Uses single threaded execution
  PARALLEL_INVOKER_THREAD_POOL = 1,  // Uses //thread/threadpool
  PARALLEL_INVOKER_OPENMP = 2,       // Uses OpenMP (requires compiler support)
  PARALLEL_INVOKER_GCD = 3,          // Uses GCD (Apple)
  PARALLEL_INVOKER_MAX_VALUE = 4,    // Increase when adding more modes
};

extern int flags_parallel_invoker_mode;
extern int flags_parallel_invoker_max_threads;

// Note flag: Parallel processing only activated if
// PARALLEL_INVOKER_ACTIVE is defined.

namespace mediapipe {

// Partitions the range [begin, end) into equal blocks of size grain_size each
// (except last one, might be less than grain_size).
class BlockedRange {
 public:
  BlockedRange(int begin, int end, int grain_size)
      : begin_(begin), end_(end), grain_size_(grain_size) {}

  int begin() const { return begin_; }
  int end() const { return end_; }
  int grain_size() const { return grain_size_; }

 private:
  int begin_;
  int end_;
  int grain_size_;
};

// Partitions the range row_range x col_range into equal
// blocks of size row_range.grain_size() x col_range.grain_size() each
// (except last column and row might be of size less than grain_size in one
// or both of their dimensions).
class BlockedRange2D {
 public:
  BlockedRange2D(const BlockedRange& rows, const BlockedRange& cols)
      : rows_(rows), cols_(cols) {}

  const BlockedRange& rows() const { return rows_; }
  const BlockedRange& cols() const { return cols_; }

 private:
  BlockedRange rows_;
  BlockedRange cols_;
};

#ifdef PARALLEL_INVOKER_ACTIVE

// Singleton ThreadPool for parallel invoker.
ThreadPool* ParallelInvokerThreadPool();

#ifdef __APPLE__
// Enable to allow GCD as an option beside ThreadPool.
#define USE_PARALLEL_INVOKER_GCD 1
#define CHECK_GCD_PARALLEL_WORK_COUNT DEBUG

template <class Invoker>
class ParallelInvokerGCDContext {
 public:
  ParallelInvokerGCDContext(const Invoker& invoker, const BlockedRange& rows)
      : local_invoker_(invoker), rows_(rows) {
#if CHECK_GCD_PARALLEL_WORK_COUNT
    count_ = 0;
#endif
  }

  const Invoker& invoker() {
#if CHECK_GCD_PARALLEL_WORK_COUNT
    // Implicitly tracking the # of launched tasks at invoker retrieval.
    atomic_fetch_add(&count_, 1);
#endif
    return local_invoker_;
  }
  const BlockedRange& rows() const { return rows_; }
#if CHECK_GCD_PARALLEL_WORK_COUNT
  const int count() { return atomic_load(&count_); }
#endif

 private:
  Invoker local_invoker_;
  const BlockedRange& rows_;
#if CHECK_GCD_PARALLEL_WORK_COUNT
  _Atomic(int32_t) count_;
#endif
};

template <class Invoker>
class ParallelInvokerGCDContext2D : public ParallelInvokerGCDContext<Invoker> {
 public:
  ParallelInvokerGCDContext2D(const Invoker& invoker, const BlockedRange& rows,
                              const BlockedRange& cols)
      : ParallelInvokerGCDContext<Invoker>(invoker, rows), cols_(cols) {}

  const BlockedRange& cols() const { return cols_; }

 private:
  BlockedRange cols_;
};

template <class Invoker>
static void ParallelForGCDTask(void* context, size_t index) {
  ParallelInvokerGCDContext<Invoker>* invoker_context =
      static_cast<ParallelInvokerGCDContext<Invoker>*>(context);
  const BlockedRange& all_tasks = invoker_context->rows();
  int start = all_tasks.begin() + index * all_tasks.grain_size();
  int end = std::min(all_tasks.end(), start + all_tasks.grain_size());
  BlockedRange this_task(start, end, all_tasks.grain_size());

  const Invoker& invoker = invoker_context->invoker();
  invoker(this_task);
}

template <class Invoker>
static void ParallelForGCDTask2D(void* context, size_t index) {
  ParallelInvokerGCDContext2D<Invoker>* invoker_context =
      static_cast<ParallelInvokerGCDContext2D<Invoker>*>(context);
  // Partitioning across rows.
  const BlockedRange& all_tasks = invoker_context->rows();
  int start = all_tasks.begin() + index * all_tasks.grain_size();
  int end = std::min(all_tasks.end(), start + all_tasks.grain_size());
  BlockedRange this_task(start, end, all_tasks.grain_size());

  const Invoker& invoker = invoker_context->invoker();
  invoker(BlockedRange2D(this_task, invoker_context->cols()));
}
#endif  // __APPLE__

#endif  // PARALLEL_INVOKER_ACTIVE
// Simple wrapper for compatibility with below ParallelFor function.
template <class Invoker>
void SerialFor(size_t start, size_t end, size_t grain_size,
               const Invoker& invoker) {
  invoker(BlockedRange(start, end, 1));
}

inline void CheckAndSetInvokerOptions() {
#if defined(PARALLEL_INVOKER_ACTIVE)
#if defined(__ANDROID__)
  // If unsupported option is selected, force usage of OpenMP if detected, and
  // ThreadPool otherwise.
  if (flags_parallel_invoker_mode != PARALLEL_INVOKER_NONE &&
      flags_parallel_invoker_mode != PARALLEL_INVOKER_THREAD_POOL &&
      flags_parallel_invoker_mode != PARALLEL_INVOKER_OPENMP) {
#if defined(_OPENMP)
    ABSL_LOG(WARNING) << "Unsupported invoker mode selected on Android. "
                      << "OpenMP linkage detected, so falling back to OpenMP";
    flags_parallel_invoker_mode = PARALLEL_INVOKER_OPENMP;
#else   // _OPENMP
    // Fallback mode for active parallel invoker without OpenMP is ThreadPool.
    ABSL_LOG(WARNING) << "Unsupported invoker mode selected on Android. "
                      << "Falling back to ThreadPool";
    flags_parallel_invoker_mode = PARALLEL_INVOKER_THREAD_POOL;
#endif  // _OPENMP
  }
#endif  // __ANDROID__

#if defined(__APPLE__) || defined(__EMSCRIPTEN__)
  // Force usage of ThreadPool if unsupported option is selected.
  // (OpenMP is not supported on iOS, due to missing clang support).
  if (flags_parallel_invoker_mode != PARALLEL_INVOKER_NONE &&
#if defined(USE_PARALLEL_INVOKER_GCD)
      flags_parallel_invoker_mode != PARALLEL_INVOKER_GCD &&
#endif  // USE_PARALLEL_INVOKER_GCD
      flags_parallel_invoker_mode != PARALLEL_INVOKER_THREAD_POOL) {
    ABSL_LOG(WARNING) << "Unsupported invoker mode selected on iOS. "
                      << "Falling back to ThreadPool mode";
    flags_parallel_invoker_mode = PARALLEL_INVOKER_THREAD_POOL;
  }
#endif  // __APPLE__ || __EMSCRIPTEN__

#if !defined(__APPLE__) && !defined(__EMSCRIPTEN__) && !defined(__ANDROID__)
  flags_parallel_invoker_mode = PARALLEL_INVOKER_THREAD_POOL;
#endif  // !__APPLE__ && !__EMSCRIPTEN__ && !__ANDROID__

  // If OpenMP is requested, make sure we can actually use it, and fall back
  // to ThreadPool if not.
  if (flags_parallel_invoker_mode == PARALLEL_INVOKER_OPENMP) {
#if !defined(_OPENMP)
    ABSL_LOG(ERROR)
        << "OpenMP invoker mode selected but not compiling with OpenMP "
        << "enabled. Falling back to ThreadPool";
    flags_parallel_invoker_mode = PARALLEL_INVOKER_THREAD_POOL;
#endif  // _OPENMP
  }

#else   // PARALLEL_INVOKER_ACTIVE
  if (flags_parallel_invoker_mode != PARALLEL_INVOKER_NONE) {
    ABSL_LOG(ERROR)
        << "Parallel execution requested but PARALLEL_INVOKER_ACTIVE "
        << "compile flag is not set. Falling back to single threaded "
        << "execution.";
    flags_parallel_invoker_mode = PARALLEL_INVOKER_NONE;
  }
#endif  // PARALLEL_INVOKER_ACTIVE

  ABSL_CHECK_LT(flags_parallel_invoker_mode, PARALLEL_INVOKER_MAX_VALUE)
      << "Invalid invoker mode specified.";
  ABSL_CHECK_GE(flags_parallel_invoker_mode, 0)
      << "Invalid invoker mode specified.";
}

// Performs parallel iteration from [start to end), scheduling grain_size
// iterations per thread. For each iteration
// invoker(BlockedRange(thread_local_start, thread_local_end))
// is called. Each thread is given its local copy of invoker, i.e.
// invoker needs to have copy constructor defined.
template <class Invoker>
void ParallelFor(size_t start, size_t end, size_t grain_size,
                 const Invoker& invoker) {
#ifdef PARALLEL_INVOKER_ACTIVE
  CheckAndSetInvokerOptions();
  switch (flags_parallel_invoker_mode) {
#if defined(__APPLE__)
    case PARALLEL_INVOKER_GCD: {
      int iterations_remain = (end - start + grain_size - 1) / grain_size;
      ABSL_CHECK_GT(iterations_remain, 0);
      if (iterations_remain == 1) {
        // Execute invoker serially.
        invoker(BlockedRange(start, std::min(end, start + grain_size), 1));
      } else {
        BlockedRange all_tasks(start, end, grain_size);
        ParallelInvokerGCDContext<Invoker> context(invoker, all_tasks);
        dispatch_queue_t concurrent_queue =
            dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0);
        dispatch_apply_f(iterations_remain, concurrent_queue, &context,
                         ParallelForGCDTask<Invoker>);
#if CHECK_GCD_PARALLEL_WORK_COUNT
        ABSL_CHECK_EQ(iterations_remain, context.count());
#endif
      }
      break;
    }
#endif  // __APPLE__

    case PARALLEL_INVOKER_THREAD_POOL: {
      int iterations_remain = (end - start + grain_size - 1) / grain_size;
      ABSL_CHECK_GT(iterations_remain, 0);
      if (iterations_remain == 1) {
        // Execute invoker serially.
        invoker(BlockedRange(start, std::min(end, start + grain_size), 1));
        break;
      }

      struct {
        absl::Mutex mutex;
        absl::CondVar completed;
        int iterations_remain ABSL_GUARDED_BY(mutex);
      } loop;
      {
        absl::MutexLock lock(&loop.mutex);
        loop.iterations_remain = iterations_remain;
      }

      for (int x = start; x < end; x += grain_size) {
        auto loop_func = [x, end, grain_size, &loop, invoker]() {
          // Execute invoker.
          invoker(BlockedRange(x, std::min(end, x + grain_size), 1));

          // Decrement counter.
          absl::MutexLock lock(&loop.mutex);
          --loop.iterations_remain;
          if (loop.iterations_remain == 0) {
            loop.completed.SignalAll();
          }
        };

        // Attempt to run in parallel, if busy run serial to avoid deadlocking.
        // This can happen during nested invocation of ParallelFor, as if the
        // loop iteration itself is calling ParallelFor we might deadlock if
        // we can not guarantee for the iteration to be scheduled.
        ParallelInvokerThreadPool()->Schedule(loop_func);
      }

      // Wait on termination of all iterations.
      loop.mutex.Lock();
      while (loop.iterations_remain > 0) {
        loop.completed.Wait(&loop.mutex);
      }
      loop.mutex.Unlock();
      break;
    }

    case PARALLEL_INVOKER_OPENMP: {
      // Use thread-local copy of invoker.
      Invoker local_invoker(invoker);
#pragma omp parallel for firstprivate(local_invoker) \
    num_threads(flags_parallel_invoker_max_threads)
      for (int x = start; x < end; ++x) {
        local_invoker(BlockedRange(x, x + 1, 1));
      }
      break;
    }

    case PARALLEL_INVOKER_NONE: {
      SerialFor(start, end, grain_size, invoker);
      break;
    }

    case PARALLEL_INVOKER_MAX_VALUE: {
      ABSL_LOG(FATAL) << "Impossible.";
      break;
    }
  }
#else
  SerialFor(start, end, grain_size, invoker);
#endif  // PARALLEL_INVOKER_ACTIVE
}

// Simple wrapper for compatibility with below ParallelFor2D function.
template <class Invoker>
void SerialFor2D(size_t start_row, size_t end_row, size_t start_col,
                 size_t end_col, size_t grain_size, const Invoker& invoker) {
  invoker(BlockedRange2D(BlockedRange(start_row, end_row, 1),
                         BlockedRange(start_col, end_col, 1)));
}

// Same as above ParallelFor for 2D iteration.
template <class Invoker>
void ParallelFor2D(size_t start_row, size_t end_row, size_t start_col,
                   size_t end_col, size_t grain_size, const Invoker& invoker) {
#ifdef PARALLEL_INVOKER_ACTIVE
  CheckAndSetInvokerOptions();
  switch (flags_parallel_invoker_mode) {
#if defined(__APPLE__)
    case PARALLEL_INVOKER_GCD: {
      const int iterations_remain =
          (end_row - start_row + grain_size - 1) / grain_size;
      ABSL_CHECK_GT(iterations_remain, 0);
      if (iterations_remain == 1) {
        // Execute invoker serially.
        invoker(BlockedRange2D(BlockedRange(start_row, end_row, 1),
                               BlockedRange(start_col, end_col, 1)));
      } else {
        BlockedRange all_tasks(start_row, end_row, grain_size);
        ParallelInvokerGCDContext2D<Invoker> context(
            invoker, all_tasks, BlockedRange(start_col, end_col, grain_size));
        dispatch_queue_t concurrent_queue =
            dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0);
        dispatch_apply_f(iterations_remain, concurrent_queue, &context,
                         ParallelForGCDTask2D<Invoker>);
#if CHECK_GCD_PARALLEL_WORK_COUNT
        ABSL_CHECK_EQ(iterations_remain, context.count());
#endif
      }
      break;
    }
#endif  // __APPLE__

    case PARALLEL_INVOKER_THREAD_POOL: {
      int iterations_remain = end_row - start_row;  // Guarded by loop_mutex
      ABSL_CHECK_GT(iterations_remain, 0);
      if (iterations_remain == 1) {
        // Execute invoker serially.
        invoker(BlockedRange2D(BlockedRange(start_row, end_row, 1),
                               BlockedRange(start_col, end_col, 1)));
        break;
      }

      absl::Mutex loop_mutex;
      absl::CondVar loop_completed;

      for (int y = start_row; y < end_row; ++y) {
        auto loop_func = [y, start_col, end_col, &loop_mutex, &loop_completed,
                          &iterations_remain, invoker]() {
          // Execute invoker.
          invoker(BlockedRange2D(BlockedRange(y, y + 1, 1),
                                 BlockedRange(start_col, end_col, 1)));

          // Decrement counter.
          absl::MutexLock lock(&loop_mutex);
          --iterations_remain;
          if (iterations_remain == 0) {
            loop_completed.Signal();
          }
        };

        // Attempt to run in parallel, if busy run serial to avoid deadlocking.
        ParallelInvokerThreadPool()->Schedule(loop_func);
      }

      // Wait on termination of all iterations.
      loop_mutex.Lock();
      while (iterations_remain > 0) {
        loop_completed.Wait(&loop_mutex);
      }
      loop_mutex.Unlock();
      break;
    }

    case PARALLEL_INVOKER_OPENMP: {
      // Use thread-local copy of invoker.
      Invoker local_invoker(invoker);
#pragma omp parallel for firstprivate(local_invoker) \
    num_threads(flags_parallel_invoker_max_threads)
      for (int y = start_row; y < end_row; ++y) {
        local_invoker(BlockedRange2D(BlockedRange(y, y + 1, 1),
                                     BlockedRange(start_col, end_col, 1)));
      }
      break;
    }

    case PARALLEL_INVOKER_NONE: {
      SerialFor2D(start_row, end_row, start_col, end_col, grain_size, invoker);
      break;
    }

    case PARALLEL_INVOKER_MAX_VALUE: {
      ABSL_LOG(FATAL) << "Impossible.";
      break;
    }
  }
#else
  SerialFor2D(start_row, end_row, start_col, end_col, grain_size, invoker);
#endif  // PARALLEL_INVOKER_ACTIVE
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_PARALLEL_INVOKER_H_
