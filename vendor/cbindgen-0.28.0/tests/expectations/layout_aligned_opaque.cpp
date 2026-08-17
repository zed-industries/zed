#define CBINDGEN_PACKED        __attribute__ ((packed))
#define CBINDGEN_ALIGNED(n)    __attribute__ ((aligned(n)))


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct OpaqueAlign16Union;

struct OpaqueAlign1Struct;

struct OpaqueAlign1Union;

struct OpaqueAlign2Struct;

struct OpaqueAlign32Struct;

struct OpaqueAlign4Struct;

struct OpaqueAlign4Union;

struct OpaqueAlign8Struct;

struct CBINDGEN_PACKED PackedStruct {
  uintptr_t arg1;
  uint8_t *arg2;
};

union CBINDGEN_PACKED PackedUnion {
  uintptr_t variant1;
  uint8_t *variant2;
};
