#ifdef __clang__
#define CBINDGEN_NONNULL _Nonnull
#else
#define CBINDGEN_NONNULL
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct {
  const Opaque *CBINDGEN_NONNULL a;
  Opaque *CBINDGEN_NONNULL b;
  const Opaque *c;
  Opaque *d;
} References;

typedef struct {
  float *CBINDGEN_NONNULL a;
  uint64_t *CBINDGEN_NONNULL b;
  Opaque *CBINDGEN_NONNULL c;
  uint64_t *CBINDGEN_NONNULL *CBINDGEN_NONNULL d;
  float *CBINDGEN_NONNULL *CBINDGEN_NONNULL e;
  Opaque *CBINDGEN_NONNULL *CBINDGEN_NONNULL f;
  uint64_t *g;
  int32_t *h;
  int32_t *CBINDGEN_NONNULL *i;
  const uint64_t *j;
  uint64_t *k;
} Pointers_u64;

void value_arg(References arg);

void mutltiple_args(int32_t *CBINDGEN_NONNULL arg,
                    Pointers_u64 *foo,
                    Opaque *CBINDGEN_NONNULL *CBINDGEN_NONNULL d);

void ref_arg(const Pointers_u64 *CBINDGEN_NONNULL arg);

void mut_ref_arg(Pointers_u64 *CBINDGEN_NONNULL arg);

void optional_ref_arg(const Pointers_u64 *arg);

void optional_mut_ref_arg(Pointers_u64 *arg);

void nullable_const_ptr(const Pointers_u64 *arg);

void nullable_mut_ptr(Pointers_u64 *arg);
