#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
} MyCLikeEnum;

typedef enum MyCLikeEnum_Prepended {
  Foo1_Prepended,
  Bar1_Prepended,
  Baz1_Prepended,
} MyCLikeEnum_Prepended;

typedef struct MyFancyStruct {
  int32_t i;
#ifdef __cplusplus
    inline void foo();
#endif
} MyFancyStruct;

typedef enum MyFancyEnum_Tag {
  Foo,
  Bar,
  Baz,
} MyFancyEnum_Tag;

typedef struct MyFancyEnum {
  MyFancyEnum_Tag tag;
  union {
    struct {
      int32_t bar;
    };
    struct {
      int32_t baz;
    };
  };
#ifdef __cplusplus
    inline void wohoo();
#endif
} MyFancyEnum;

typedef union MyUnion {
  float f;
  uint32_t u;
  int32_t extra_member;
} MyUnion;

typedef struct MyFancyStruct_Prepended {
#ifdef __cplusplus
    inline void prepended_wohoo();
#endif
  int32_t i;
} MyFancyStruct_Prepended;

typedef enum MyFancyEnum_Prepended_Tag {
  Foo_Prepended,
  Bar_Prepended,
  Baz_Prepended,
} MyFancyEnum_Prepended_Tag;

typedef struct MyFancyEnum_Prepended {
#ifdef __cplusplus
    inline void wohoo();
#endif
  MyFancyEnum_Prepended_Tag tag;
  union {
    struct {
      int32_t bar_prepended;
    };
    struct {
      int32_t baz_prepended;
    };
  };
} MyFancyEnum_Prepended;

typedef union MyUnion_Prepended {
    int32_t extra_member;
  float f;
  uint32_t u;
} MyUnion_Prepended;

void root(struct MyFancyStruct s,
          struct MyFancyEnum e,
          enum MyCLikeEnum c,
          union MyUnion u,
          struct MyFancyStruct_Prepended sp,
          struct MyFancyEnum_Prepended ep,
          enum MyCLikeEnum_Prepended cp,
          union MyUnion_Prepended up);
