struct FOO {
  FOO() : a(0), b(0) {}
  int callee();
  __attribute__((noinline)) void caller(int n) {
      int r = callee();
      if (r == 0) {
        a += n;
        b += 1;
      }
  }
  int a;
  int volatile b;
};
