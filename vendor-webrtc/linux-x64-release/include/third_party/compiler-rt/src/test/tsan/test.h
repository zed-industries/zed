#ifndef __TSAN_TEST_H__
#define __TSAN_TEST_H__

#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <dlfcn.h>
#include <stddef.h>
#include <sched.h>
#include <stdarg.h>
#include "sanitizer_common/print_address.h"

#include <sanitizer/tsan_interface.h>

#ifdef __APPLE__
#include <mach/mach_time.h>
#endif

#ifndef TSAN_VECTORIZE
#  define TSAN_VECTORIZE __SSE4_2__
#endif

#if TSAN_VECTORIZE
#  include <emmintrin.h>
#  include <smmintrin.h>
#else
struct __m128i {
  unsigned long long x[2];
};
#endif

// TSan-invisible barrier.
// Tests use it to establish necessary execution order in a way that does not
// interfere with tsan (does not establish synchronization between threads).
typedef unsigned invisible_barrier_t;

#ifdef __cplusplus
extern "C" {
#endif
void __tsan_testonly_barrier_init(invisible_barrier_t *barrier,
    unsigned count);
void __tsan_testonly_barrier_wait(invisible_barrier_t *barrier);
unsigned long __tsan_testonly_shadow_stack_current_size();
#ifdef __cplusplus
}
#endif

static inline void barrier_init(invisible_barrier_t *barrier, unsigned count) {
  __tsan_testonly_barrier_init(barrier, count);
}

static inline void barrier_wait(invisible_barrier_t *barrier) {
  __tsan_testonly_barrier_wait(barrier);
}

// Default instance of the barrier, but a test can declare more manually.
invisible_barrier_t barrier;

#ifdef __APPLE__
unsigned long long monotonic_clock_ns() {
  static mach_timebase_info_data_t timebase_info;
  if (timebase_info.denom == 0) mach_timebase_info(&timebase_info);
  return (mach_absolute_time() * timebase_info.numer) / timebase_info.denom;
}
#else
unsigned long long monotonic_clock_ns() {
  struct timespec t;
  clock_gettime(CLOCK_MONOTONIC, &t);
  return (unsigned long long)t.tv_sec * 1000000000ull + t.tv_nsec;
}
#endif

//The const kPCInc must be in sync with StackTrace::GetPreviousInstructionPc
#if defined(__s390__) || defined(__i386__) || defined(__x86_64__)
const int kPCInc = 1;
#elif defined(__sparc__) || defined(__mips__)
const int kPCInc = 8;
#elif defined(__riscv) && __riscv_xlen == 64
const int kPCInc = 2;
#else
const int kPCInc = 4;
#endif

#ifdef __cplusplus
extern "C" {
#endif

void AnnotateThreadName(const char *f, int l, const char *name);

void AnnotateRWLockCreate(const char *f, int l, void *m);
void AnnotateRWLockCreateStatic(const char *f, int l, void *m);
void AnnotateRWLockDestroy(const char *f, int l, void *m);
void AnnotateRWLockAcquired(const char *f, int l, void *m, long is_w);
void AnnotateRWLockReleased(const char *f, int l, void *m, long is_w);

void AnnotateIgnoreReadsBegin(const char *f, int l);
void AnnotateIgnoreReadsEnd(const char *f, int l);
void AnnotateIgnoreWritesBegin(const char *f, int l);
void AnnotateIgnoreWritesEnd(const char *f, int l);

void AnnotateIgnoreSyncBegin(const char *f, int l);
void AnnotateIgnoreSyncEnd(const char *f, int l);

void AnnotateHappensBefore(const char *f, int l, void *addr);
void AnnotateHappensAfter(const char *f, int l, void *addr);

void AnnotateBenignRaceSized(const char *f, int l, const volatile void *mem,
                             unsigned int size, const char *desc);
void WTFAnnotateBenignRaceSized(const char *f, int l, const volatile void *mem,
                                unsigned int size, const char *desc);

#ifdef __cplusplus
}
#endif

#define ANNOTATE_RWLOCK_CREATE(m) \
    AnnotateRWLockCreate(__FILE__, __LINE__, m)
#define ANNOTATE_RWLOCK_CREATE_STATIC(m) \
    AnnotateRWLockCreateStatic(__FILE__, __LINE__, m)
#define ANNOTATE_RWLOCK_DESTROY(m) \
    AnnotateRWLockDestroy(__FILE__, __LINE__, m)
#define ANNOTATE_RWLOCK_ACQUIRED(m, is_w) \
    AnnotateRWLockAcquired(__FILE__, __LINE__, m, is_w)
#define ANNOTATE_RWLOCK_RELEASED(m, is_w) \
    AnnotateRWLockReleased(__FILE__, __LINE__, m, is_w)
#define ANNOTATE_HAPPENS_BEFORE(addr) \
  AnnotateHappensBefore(__FILE__, __LINE__, (void *)(addr))
#define ANNOTATE_HAPPENS_AFTER(addr) \
  AnnotateHappensAfter(__FILE__, __LINE__, (void *)(addr))
#define ANNOTATE_BENIGN_RACE(var) \
  AnnotateBenignRaceSized(__FILE__, __LINE__, &(var), sizeof(var), #var)
#define WTF_ANNOTATE_BENIGN_RACE(var) \
  WTFAnnotateBenignRaceSized(__FILE__, __LINE__, &(var), sizeof(var), #var)

#ifdef __APPLE__
#define ASM_SYMBOL(symbol) "_" #symbol
#else
#define ASM_SYMBOL(symbol) #symbol
#endif

#endif // __TSAN_TEST_H__
