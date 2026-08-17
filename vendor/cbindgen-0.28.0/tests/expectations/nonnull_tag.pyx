from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Opaque:
    pass

  cdef struct Foo_u64:
    float *a;
    uint64_t *b;
    Opaque *c;
    uint64_t **d;
    float **e;
    Opaque **f;
    uint64_t *g;
    int32_t *h;
    int32_t **i;

  void root(int32_t *arg, Foo_u64 *foo, Opaque **d);
