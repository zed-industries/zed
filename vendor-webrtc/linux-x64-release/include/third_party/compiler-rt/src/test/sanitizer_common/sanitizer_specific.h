#ifndef __SANITIZER_COMMON_SANITIZER_SPECIFIC_H__
#define __SANITIZER_COMMON_SANITIZER_SPECIFIC_H__

#include <sanitizer/lsan_interface.h>

__attribute__((weak)) int __lsan_do_recoverable_leak_check() { return 0; }
__attribute__((weak)) void __lsan_disable(void) {}
__attribute__((weak)) void __lsan_enable(void) {}

#ifndef __has_feature
#  define __has_feature(x) 0
#endif

#if __has_feature(memory_sanitizer)
#  include <sanitizer/msan_interface.h>
static void check_mem_is_good(void *p, size_t s) {
  __msan_check_mem_is_initialized(p, s);
}
static void make_mem_good(void *p, size_t s) { __msan_unpoison(p, s); }
static void make_mem_bad(void *p, size_t s) { __msan_poison(p, s); }
#elif __has_feature(address_sanitizer)
#  include <sanitizer/asan_interface.h>
#  include <stdlib.h>
static void check_mem_is_good(void *p, size_t s) {
  if (__asan_region_is_poisoned(p, s))
    abort();
}
static void make_mem_good(void *p, size_t s) {
  __asan_unpoison_memory_region(p, s);
}
static void make_mem_bad(void *p, size_t s) {
  __asan_poison_memory_region(p, s);
}
#elif __has_feature(hwaddress_sanitizer)
#  include <sanitizer/hwasan_interface.h>
#  include <stdlib.h>
static void check_mem_is_good(void *p, size_t s) {
  if (__hwasan_test_shadow(p, s) != -1)
    abort();
}
static void make_mem_good(void *p, size_t s) {
  __hwasan_tag_memory(p, __hwasan_get_tag_from_pointer(p), s);
}
static void make_mem_bad(void *p, size_t s) {
  uint8_t tag = ~__hwasan_get_tag_from_pointer(p);
  if (!tag) {
    // Nothing wrong with tag zero, but non-zero tags help to detect never
    // tagged memory.
    tag = 1;
  }
  __hwasan_tag_memory(p, tag, s);
  // With misaligned `p` or short granules we can't guarantee tag mismatch.
  if (__hwasan_test_shadow(p, s) != 0)
    abort();
  if (s > 1 && __hwasan_test_shadow(((char *)p) + s - 1, 1) != 0)
    abort();
}
#else
static void check_mem_is_good(void *p, size_t s) {}
static void make_mem_good(void *p, size_t s) {}
static void make_mem_bad(void *p, size_t s) {}
#endif

#endif // __SANITIZER_COMMON_SANITIZER_SPECIFIC_H__
