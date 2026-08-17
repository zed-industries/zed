#ifndef _TIME_H
#define _TIME_H

#define CLOCKS_PER_SEC 1000

typedef unsigned long long clock_t;

// Clock is just use for progress reporting, which we disable anyway.
inline clock_t clock() {
    return 0;
}

#endif // _TIME_H
