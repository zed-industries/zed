#if 0
DEF PLATFORM_UNIX = 0
DEF PLATFORM_WIN = 0
DEF X11 = 0
DEF M_32 = 0
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum BarType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  A,
  B,
  C,
};
#ifndef __cplusplus
typedef uint32_t BarType;
#endif // __cplusplus
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum FooType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  A,
  B,
  C,
};
#ifndef __cplusplus
typedef uint32_t FooType;
#endif // __cplusplus
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
struct FooHandle {
  FooType ty;
  int32_t x;
  float y;
};
#endif

enum C_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  C1,
  C2,
#if defined(PLATFORM_WIN)
  C3,
#endif
#if defined(PLATFORM_UNIX)
  C5,
#endif
};
#ifndef __cplusplus
typedef uint8_t C_Tag;
#endif // __cplusplus

#if defined(PLATFORM_UNIX)
struct C5_Body {
  C_Tag tag;
  int32_t int_;
};
#endif

union C {
  C_Tag tag;
#if defined(PLATFORM_UNIX)
  struct C5_Body c5;
#endif
};

#if (defined(PLATFORM_WIN) || defined(M_32))
struct BarHandle {
  BarType ty;
  int32_t x;
  float y;
};
#endif

struct ConditionalField {
#if defined(X11)
  int32_t field
#endif
  ;
};

struct Normal {
  int32_t x;
  float y;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if defined(PLATFORM_WIN)
extern int32_t global_array_with_different_sizes[2];
#endif

#if defined(PLATFORM_UNIX)
extern int32_t global_array_with_different_sizes[1];
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(struct FooHandle a, union C c);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(struct BarHandle a, union C c);
#endif

void cond(struct ConditionalField a);

#if defined(PLATFORM_WIN)
extern int32_t foo(void);
#endif

#if defined(PLATFORM_WIN)
extern void bar(struct Normal a);
#endif

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
