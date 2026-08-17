#include "test.h"
#include <atomic>
#include <vector>
#include <sanitizer/tsan_interface.h>

// A very primitive mutex annotated with tsan annotations.
class Mutex {
 public:
  Mutex(bool prof, unsigned create_flags, unsigned destroy_flags=0)
      : prof_(prof)
      , locked_(false)
      , seq_(0)
      , destroy_flags_(destroy_flags) {
    __tsan_mutex_create(this, create_flags);
  }

  ~Mutex() {
    __tsan_mutex_destroy(this, destroy_flags_);
  }

  void Lock() {
    __tsan_mutex_pre_lock(this, 0);
    LockImpl();
    __tsan_mutex_post_lock(this, 0, 0);
  }

  bool TryLock() {
    __tsan_mutex_pre_lock(this, __tsan_mutex_try_lock);
    bool ok = TryLockImpl();
    __tsan_mutex_post_lock(this, __tsan_mutex_try_lock |
        (ok ? 0 : __tsan_mutex_try_lock_failed), 0);
    return ok;
  }

  void Unlock() {
    __tsan_mutex_pre_unlock(this, 0);
    UnlockImpl();
    __tsan_mutex_post_unlock(this, 0);
  }

  void Wait() {
    for (int seq = seq_; seq == seq_;) {
      Unlock();
      usleep(100);
      Lock();
    }
  }

  void Broadcast() {
    __tsan_mutex_pre_signal(this, 0);
    LockImpl(false);
    seq_++;
    UnlockImpl();
    __tsan_mutex_post_signal(this, 0);
  }

 private:
  const bool prof_;
  std::atomic<bool> locked_;
  int seq_;
  unsigned destroy_flags_;

  // This models mutex profiling subsystem.
  static Mutex prof_mu_;
  static int prof_data_;

  void LockImpl(bool prof = true) {
    while (!TryLockImpl())
      usleep(100);
    if (prof && prof_)
      Prof();
  }

  bool TryLockImpl() {
    return !locked_.exchange(true);
  }

  void UnlockImpl() {
    locked_.store(false);
  }

  void Prof() {
      // This happens inside of mutex lock annotations.
      __tsan_mutex_pre_divert(this, 0);
      prof_mu_.Lock();
      prof_data_++;
      prof_mu_.Unlock();
      __tsan_mutex_post_divert(this, 0);
  }
};

Mutex Mutex::prof_mu_(false, __tsan_mutex_linker_init);
int Mutex::prof_data_;
