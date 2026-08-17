#if defined(APPLE) || (defined(WINDOWS) && defined(X86))
#define GLOBAL(name) .globl _ ## name; _ ## name
#else
#define GLOBAL(name) .globl name; name
#endif
