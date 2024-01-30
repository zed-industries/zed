#!/usr/bin/python
#
# Program that writes a number to stdout repeatedly
#
# This requires Python 2.6 or later.

from __future__ import print_function
import sys
import time

if __name__ == "__main__":

    done = 0
    while done < 10:
        done = done + 1
        print(done)
        sys.stdout.flush()
        time.sleep(0.05)  # sleep 50 msec
