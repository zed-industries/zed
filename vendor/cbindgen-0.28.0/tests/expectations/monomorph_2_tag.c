#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct A;

struct B;

struct List_A {
  struct A *members;
  uintptr_t count;
};

struct List_B {
  struct B *members;
  uintptr_t count;
};

void foo(struct List_A a);

void bar(struct List_B b);
