#ifndef PAGE_SIZE_
#define PAGE_SIZE_

#if defined (__unix__) || (defined (__APPLE__) && defined (__MACH__))
# include <unistd.h>
unsigned pageSize() {
  return sysconf(_SC_PAGESIZE);
}
#else
# error "GWP-ASan is not supported on this platform."
#endif

#endif // PAGE_SIZE_
