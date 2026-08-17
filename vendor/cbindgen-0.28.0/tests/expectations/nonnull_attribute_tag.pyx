#ifdef __clang__
#define CBINDGEN_NONNULL _Nonnull
#else
#define CBINDGEN_NONNULL
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Opaque:
    pass

  cdef struct References:
    const Opaque *a;
    Opaque *b;
    const Opaque *c;
    Opaque *d;

  cdef struct Pointers_u64:
    float *a;
    uint64_t *b;
    Opaque *c;
    uint64_t **d;
    float **e;
    Opaque **f;
    uint64_t *g;
    int32_t *h;
    int32_t **i;
    const uint64_t *j;
    uint64_t *k;

  void value_arg(References arg);

  void mutltiple_args(int32_t *arg, Pointers_u64 *foo, Opaque **d);

  void ref_arg(const Pointers_u64 *arg);

  void mut_ref_arg(Pointers_u64 *arg);

  void optional_ref_arg(const Pointers_u64 *arg);

  void optional_mut_ref_arg(Pointers_u64 *arg);

  void nullable_const_ptr(const Pointers_u64 *arg);

  void nullable_mut_ptr(Pointers_u64 *arg);
