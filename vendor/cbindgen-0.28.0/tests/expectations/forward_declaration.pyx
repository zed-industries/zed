#if 0
''' '
#endif
#if defined(CBINDGEN_STYLE_TYPE)
/* ANONYMOUS STRUCTS DO NOT SUPPORT FORWARD DECLARATIONS!
#endif
#if 0
' '''
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct StructInfo:
    const TypeInfo *const *fields;
    uintptr_t num_fields;

  ctypedef enum TypeData_Tag:
    Primitive,
    Struct,

  ctypedef struct TypeData:
    TypeData_Tag tag;
    StructInfo struct_;

  ctypedef struct TypeInfo:
    TypeData data;

  void root(TypeInfo x);

#if 0
''' '
#endif
#if defined(CBINDGEN_STYLE_TYPE)
*/
#endif
#if 0
' '''
#endif
