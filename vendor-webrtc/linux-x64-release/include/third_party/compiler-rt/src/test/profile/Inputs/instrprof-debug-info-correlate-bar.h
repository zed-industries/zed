int foo(int);
int unused(int);

inline int bar(int a) {
  while (a > 100)
    a /= 2;
  return a;
}
