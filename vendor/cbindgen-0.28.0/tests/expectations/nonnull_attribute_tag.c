#ifdef __clang__
#define CBINDGEN_NONNULL _Nonnull
#else
#define CBINDGEN_NONNULL
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque;

struct References {
  const struct Opaque *CBINDGEN_NONNULL a;
  struct Opaque *CBINDGEN_NONNULL b;
  const struct Opaque *c;
  struct Opaque *d;
};

struct Pointers_u64 {
  float *CBINDGEN_NONNULL a;
  uint64_t *CBINDGEN_NONNULL b;
  struct Opaque *CBINDGEN_NONNULL c;
  uint64_t *CBINDGEN_NONNULL *CBINDGEN_NONNULL d;
  float *CBINDGEN_NONNULL *CBINDGEN_NONNULL e;
  struct Opaque *CBINDGEN_NONNULL *CBINDGEN_NONNULL f;
  uint64_t *g;
  int32_t *h;
  int32_t *CBINDGEN_NONNULL *i;
  const uint64_t *j;
  uint64_t *k;
};

void value_arg(struct References arg);

void mutltiple_args(int32_t *CBINDGEN_NONNULL arg,
                    struct Pointers_u64 *foo,
                    struct Opaque *CBINDGEN_NONNULL *CBINDGEN_NONNULL d);

void ref_arg(const struct Pointers_u64 *CBINDGEN_NONNULL arg);

void mut_ref_arg(struct Pointers_u64 *CBINDGEN_NONNULL arg);

void optional_ref_arg(const struct Pointers_u64 *arg);

void optional_mut_ref_arg(struct Pointers_u64 *arg);

void nullable_const_ptr(const struct Pointers_u64 *arg);

void nullable_mut_ptr(struct Pointers_u64 *arg);
