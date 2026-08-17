#include <pthread.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

void *volatile mem;
volatile int len;

void *Thread(void *p) {
  while ((p = __atomic_load_n(&mem, __ATOMIC_ACQUIRE)) == 0)
    usleep(100);
  memset(p, 0, len);
  return 0;
}

extern "C" void libfunc() {
  pthread_t t;
  pthread_create(&t, 0, Thread, 0);
  len = 10;
  __atomic_store_n(&mem, malloc(len), __ATOMIC_RELEASE);
  pthread_join(t, 0);
  free(mem);
  fprintf(stderr, "OK\n");
}
