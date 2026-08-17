#if 0
DEF FOO = 0
DEF BAR = 0
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(FOO)
#define FOO 1
#endif

#if defined(BAR)
#define BAR 2
#endif

#if defined(FOO)
struct Foo {

};
#endif

#if defined(BAR)
struct Bar {

};
#endif

#if defined(FOO)
void foo(const struct Foo *foo);
#endif

#if defined(BAR)
void bar(const struct Bar *bar);
#endif
