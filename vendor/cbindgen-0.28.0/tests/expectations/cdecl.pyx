from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef void (*A)();

  ctypedef void (*B)();

  ctypedef bool (*C)(int32_t, int32_t);

  ctypedef bool (*(*D)(int32_t))(float);

  ctypedef const int32_t (*(*E)())[16];

  ctypedef const int32_t *F;

  ctypedef const int32_t *const *G;

  ctypedef int32_t *const *H;

  ctypedef const int32_t (*I)[16];

  ctypedef double (**J)(float);

  ctypedef int32_t K[16];

  ctypedef const int32_t *L[16];

  ctypedef bool (*M[16])(int32_t, int32_t);

  ctypedef void (*N[16])(int32_t, int32_t);

  ctypedef void (*P)(int32_t named1st, bool, bool named3rd, int32_t _);

  void (*O())();

  void root(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, P p);
