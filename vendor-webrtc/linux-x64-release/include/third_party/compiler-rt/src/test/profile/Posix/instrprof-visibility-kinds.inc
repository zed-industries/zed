void f1() {}

#ifndef NO_WEAK
void f2() __attribute__((weak));
void f2() {}
#endif

void f3() __attribute__((always_inline));
void f3() {}

#ifndef NO_EXTERN
extern void f4();
#endif

void f5() __attribute__((visibility("default")));
void f5() {}

void f6() __attribute__((visibility("hidden")));
void f6() {}

void f7() __attribute__((visibility("internal")));
void f7() {}

void call() {
  f1();
#ifndef NO_WEAK
  f2();
#endif
  f3();
#ifndef NO_EXTERN
  f4();
#endif
  f5();
  f6();
  f7();
}
