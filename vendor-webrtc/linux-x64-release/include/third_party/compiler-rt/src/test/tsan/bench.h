#include "test.h"
#include <time.h>

int bench_nthread;
int bench_niter;
int bench_mode;

void bench();  // defined by user
void start_thread_group(int nth, void(*f)(int tid));

int main(int argc, char **argv) {
  bench_nthread = 2;
  if (argc > 1)
    bench_nthread = atoi(argv[1]);
  bench_niter = 100;
  if (argc > 2)
    bench_niter = atoi(argv[2]);
  if (argc > 3)
    bench_mode = atoi(argv[3]);

  timespec tp0;
  clock_gettime(CLOCK_MONOTONIC, &tp0);
  bench();
  timespec tp1;
  clock_gettime(CLOCK_MONOTONIC, &tp1);
  unsigned long long t =
      (tp1.tv_sec * 1000000000ULL + tp1.tv_nsec) -
      (tp0.tv_sec * 1000000000ULL + tp0.tv_nsec);
  fprintf(stderr, "%llu ns/iter\n", t / bench_niter);
  fprintf(stderr, "DONE\n");
}

void start_thread_group(int nth, void(*f)(int tid)) {
  pthread_t *th = (pthread_t*)malloc(nth * sizeof(pthread_t));
  for (int i = 0; i < nth; i++)
    pthread_create(&th[i], 0, (void*(*)(void*))f, (void*)(long)i);
  for (int i = 0; i < nth; i++)
    pthread_join(th[i], 0);
}
