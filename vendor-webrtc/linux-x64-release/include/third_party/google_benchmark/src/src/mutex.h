#ifndef BENCHMARK_MUTEX_H_
#define BENCHMARK_MUTEX_H_

#include <condition_variable>
#include <mutex>

#include "check.h"

// Enable thread safety attributes only with clang.
// The attributes can be safely erased when compiling with other compilers.
#if defined(HAVE_THREAD_SAFETY_ATTRIBUTES)
#define THREAD_ANNOTATION_ATTRIBUTE_(x) __attribute__((x))
#else
#define THREAD_ANNOTATION_ATTRIBUTE_(x)  // no-op
#endif

#define CAPABILITY(x) THREAD_ANNOTATION_ATTRIBUTE_(capability(x))

#define SCOPED_CAPABILITY THREAD_ANNOTATION_ATTRIBUTE_(scoped_lockable)

#define GUARDED_BY(x) THREAD_ANNOTATION_ATTRIBUTE_(guarded_by(x))

#define PT_GUARDED_BY(x) THREAD_ANNOTATION_ATTRIBUTE_(pt_guarded_by(x))

#define ACQUIRED_BEFORE(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(acquired_before(__VA_ARGS__))

#define ACQUIRED_AFTER(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(acquired_after(__VA_ARGS__))

#define REQUIRES(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(requires_capability(__VA_ARGS__))

#define REQUIRES_SHARED(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(requires_shared_capability(__VA_ARGS__))

#define ACQUIRE(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(acquire_capability(__VA_ARGS__))

#define ACQUIRE_SHARED(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(acquire_shared_capability(__VA_ARGS__))

#define RELEASE(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(release_capability(__VA_ARGS__))

#define RELEASE_SHARED(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(release_shared_capability(__VA_ARGS__))

#define TRY_ACQUIRE(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(try_acquire_capability(__VA_ARGS__))

#define TRY_ACQUIRE_SHARED(...) \
  THREAD_ANNOTATION_ATTRIBUTE_(try_acquire_shared_capability(__VA_ARGS__))

#define EXCLUDES(...) THREAD_ANNOTATION_ATTRIBUTE_(locks_excluded(__VA_ARGS__))

#define ASSERT_CAPABILITY(x) THREAD_ANNOTATION_ATTRIBUTE_(assert_capability(x))

#define ASSERT_SHARED_CAPABILITY(x) \
  THREAD_ANNOTATION_ATTRIBUTE_(assert_shared_capability(x))

#define RETURN_CAPABILITY(x) THREAD_ANNOTATION_ATTRIBUTE_(lock_returned(x))

#define NO_THREAD_SAFETY_ANALYSIS \
  THREAD_ANNOTATION_ATTRIBUTE_(no_thread_safety_analysis)

namespace benchmark {

typedef std::condition_variable Condition;

// NOTE: Wrappers for std::mutex and std::unique_lock are provided so that
// we can annotate them with thread safety attributes and use the
// -Wthread-safety warning with clang. The standard library types cannot be
// used directly because they do not provide the required annotations.
class CAPABILITY("mutex") Mutex {
 public:
  Mutex() {}

  void lock() ACQUIRE() { mut_.lock(); }
  void unlock() RELEASE() { mut_.unlock(); }
  std::mutex& native_handle() { return mut_; }

 private:
  std::mutex mut_;
};

class SCOPED_CAPABILITY MutexLock {
  typedef std::unique_lock<std::mutex> MutexLockImp;

 public:
  MutexLock(Mutex& m) ACQUIRE(m) : ml_(m.native_handle()) {}
  ~MutexLock() RELEASE() {}
  MutexLockImp& native_handle() { return ml_; }

 private:
  MutexLockImp ml_;
};

class Barrier {
 public:
  Barrier(int num_threads) : running_threads_(num_threads) {}

  // Called by each thread
  bool wait() EXCLUDES(lock_) {
    bool last_thread = false;
    {
      MutexLock ml(lock_);
      last_thread = createBarrier(ml);
    }
    if (last_thread) phase_condition_.notify_all();
    return last_thread;
  }

  void removeThread() EXCLUDES(lock_) {
    MutexLock ml(lock_);
    --running_threads_;
    if (entered_ != 0) phase_condition_.notify_all();
  }

 private:
  Mutex lock_;
  Condition phase_condition_;
  int running_threads_;

  // State for barrier management
  int phase_number_ = 0;
  int entered_ = 0;  // Number of threads that have entered this barrier

  // Enter the barrier and wait until all other threads have also
  // entered the barrier.  Returns iff this is the last thread to
  // enter the barrier.
  bool createBarrier(MutexLock& ml) REQUIRES(lock_) {
    BM_CHECK_LT(entered_, running_threads_);
    entered_++;
    if (entered_ < running_threads_) {
      // Wait for all threads to enter
      int phase_number_cp = phase_number_;
      auto cb = [this, phase_number_cp]() {
        return this->phase_number_ > phase_number_cp ||
               entered_ == running_threads_;  // A thread has aborted in error
      };
      phase_condition_.wait(ml.native_handle(), cb);
      if (phase_number_ > phase_number_cp) return false;
      // else (running_threads_ == entered_) and we are the last thread.
    }
    // Last thread has reached the barrier
    phase_number_++;
    entered_ = 0;
    return true;
  }
};

}  // end namespace benchmark

#endif  // BENCHMARK_MUTEX_H_
