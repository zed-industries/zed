#define CBINDGEN_PACKED        __attribute__ ((packed))
#define CBINDGEN_ALIGNED(n)    __attribute__ ((aligned(n)))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct OpaqueAlign16Union OpaqueAlign16Union;

typedef struct OpaqueAlign1Struct OpaqueAlign1Struct;

typedef struct OpaqueAlign1Union OpaqueAlign1Union;

typedef struct OpaqueAlign2Struct OpaqueAlign2Struct;

typedef struct OpaqueAlign32Struct OpaqueAlign32Struct;

typedef struct OpaqueAlign4Struct OpaqueAlign4Struct;

typedef struct OpaqueAlign4Union OpaqueAlign4Union;

typedef struct OpaqueAlign8Struct OpaqueAlign8Struct;

typedef struct CBINDGEN_PACKED {
  uintptr_t arg1;
  uint8_t *arg2;
} PackedStruct;

typedef union CBINDGEN_PACKED {
  uintptr_t variant1;
  uint8_t *variant2;
} PackedUnion;
