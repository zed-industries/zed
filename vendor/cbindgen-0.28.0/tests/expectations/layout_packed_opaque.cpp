#define CBINDGEN_PACKED        __attribute__ ((packed))
#define CBINDGEN_ALIGNED(n)    __attribute__ ((aligned(n)))


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct OpaquePackedStruct;

struct OpaquePackedUnion;

union CBINDGEN_ALIGNED(1) Align1Union {
  uintptr_t variant1;
  uint8_t *variant2;
};

union CBINDGEN_ALIGNED(4) Align4Union {
  uintptr_t variant1;
  uint8_t *variant2;
};

union CBINDGEN_ALIGNED(16) Align16Union {
  uintptr_t variant1;
  uint8_t *variant2;
};

struct CBINDGEN_ALIGNED(1) Align1Struct {
  uintptr_t arg1;
  uint8_t *arg2;
};

struct CBINDGEN_ALIGNED(2) Align2Struct {
  uintptr_t arg1;
  uint8_t *arg2;
};

struct CBINDGEN_ALIGNED(4) Align4Struct {
  uintptr_t arg1;
  uint8_t *arg2;
};

struct CBINDGEN_ALIGNED(8) Align8Struct {
  uintptr_t arg1;
  uint8_t *arg2;
};

struct CBINDGEN_ALIGNED(32) Align32Struct {
  uintptr_t arg1;
  uint8_t *arg2;
};
