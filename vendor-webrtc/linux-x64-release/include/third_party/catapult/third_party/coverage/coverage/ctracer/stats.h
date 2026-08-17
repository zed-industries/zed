/* Licensed under the Apache License: http://www.apache.org/licenses/LICENSE-2.0 */
/* For details: https://bitbucket.org/ned/coveragepy/src/default/NOTICE.txt */

#ifndef _COVERAGE_STATS_H
#define _COVERAGE_STATS_H

#include "util.h"

#if COLLECT_STATS
#define STATS(x)        x
#else
#define STATS(x)
#endif

typedef struct Stats {
    unsigned int calls;     /* Need at least one member, but the rest only if needed. */
#if COLLECT_STATS
    unsigned int lines;
    unsigned int returns;
    unsigned int exceptions;
    unsigned int others;
    unsigned int new_files;
    unsigned int missed_returns;
    unsigned int stack_reallocs;
    unsigned int errors;
    unsigned int pycalls;
#endif
} Stats;

#endif /* _COVERAGE_STATS_H */
