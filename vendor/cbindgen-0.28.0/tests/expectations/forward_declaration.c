#if 0
''' '
#endif
#if defined(CBINDGEN_STYLE_TYPE)
/* ANONYMOUS STRUCTS DO NOT SUPPORT FORWARD DECLARATIONS!
#endif
#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const TypeInfo *const *fields;
  uintptr_t num_fields;
} StructInfo;

typedef enum {
  Primitive,
  Struct,
} TypeData_Tag;

typedef struct {
  TypeData_Tag tag;
  union {
    struct {
      StructInfo struct_;
    };
  };
} TypeData;

typedef struct {
  TypeData data;
} TypeInfo;

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
