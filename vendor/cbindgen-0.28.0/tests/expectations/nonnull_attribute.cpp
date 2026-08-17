#ifdef __clang__
#define CBINDGEN_NONNULL _Nonnull
#else
#define CBINDGEN_NONNULL
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Opaque;

struct References {
  const Opaque *CBINDGEN_NONNULL a;
  Opaque *CBINDGEN_NONNULL b;
  const Opaque *c;
  Opaque *d;
};

template<typename T>
struct Pointers {
  float *CBINDGEN_NONNULL a;
  T *CBINDGEN_NONNULL b;
  Opaque *CBINDGEN_NONNULL c;
  T *CBINDGEN_NONNULL *CBINDGEN_NONNULL d;
  float *CBINDGEN_NONNULL *CBINDGEN_NONNULL e;
  Opaque *CBINDGEN_NONNULL *CBINDGEN_NONNULL f;
  T *g;
  int32_t *h;
  int32_t *CBINDGEN_NONNULL *i;
  const T *j;
  T *k;
};

extern "C" {

void value_arg(References arg);

void mutltiple_args(int32_t *CBINDGEN_NONNULL arg,
                    Pointers<uint64_t> *foo,
                    Opaque *CBINDGEN_NONNULL *CBINDGEN_NONNULL d);

void ref_arg(const Pointers<uint64_t> *CBINDGEN_NONNULL arg);

void mut_ref_arg(Pointers<uint64_t> *CBINDGEN_NONNULL arg);

void optional_ref_arg(const Pointers<uint64_t> *arg);

void optional_mut_ref_arg(Pointers<uint64_t> *arg);

void nullable_const_ptr(const Pointers<uint64_t> *arg);

void nullable_mut_ptr(Pointers<uint64_t> *arg);

}  // extern "C"
