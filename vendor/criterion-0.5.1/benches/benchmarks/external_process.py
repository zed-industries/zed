import time
import sys


def fibonacci(n):
    if n == 0 or n == 1:
        return 1
    return fibonacci(n - 1) + fibonacci(n - 2)


MILLIS = 1000
MICROS = MILLIS * 1000
NANOS = MICROS * 1000


def benchmark():
    depth = int(sys.argv[1])
    for line in sys.stdin:
        iters = int(line.strip())

        # Setup

        start = time.perf_counter()
        for x in range(iters):
            fibonacci(depth)
        end = time.perf_counter()

        # Teardown

        delta = end - start
        nanos = int(delta * NANOS)
        print("%d" % nanos)
        sys.stdout.flush()


benchmark()
