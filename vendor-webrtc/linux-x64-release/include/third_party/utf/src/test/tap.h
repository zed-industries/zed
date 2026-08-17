/* See LICENSE file for copyright and license details. */
#ifndef TAP_H
#define TAP_H

#include <stdio.h>
#include <stdlib.h>

static int _tap_test = 0;

#define plan(N) printf("1..%d\n", (N))
#define skip_all(S) printf("1..0 # %s\n", (S))

#define pass(S) (printf("ok %d - %s\n", ++_tap_test, (S)), 1)
#define fail(S) (printf("not ok %d - %s\n", ++_tap_test, (S)), 0)

#define ok(P, S) ((P) ? pass(S) : fail(S))
#define is(A, B, S) ok((A) == (B), (S))
#define isnt(A, B, S) ok((A) != (B), (S))

#define skip(N, S)                                   \
  do {                                               \
    int _tap_skip = _tap_test + (N);                 \
    while (_tap_test < _tap_skip)                    \
      printf("ok %d # SKIP %s\n", ++_tap_test, (S)); \
  } while (0)

#define diag(S) fprintf(stderr, "# %s\n", (S))
#define note(S) fprintf(stdout, "# %s\n", (S))

#define bail_out(S) (printf("Bail out! %s\n", (S)), exit(0))

#endif
