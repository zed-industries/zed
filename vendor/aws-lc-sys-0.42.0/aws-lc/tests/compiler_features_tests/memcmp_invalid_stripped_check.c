// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
  // A bug of 'memcmp' is reported in gcc (9.2, 9.3, 10.1). See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189
  // AWS-LC warns the build when detecting the unexpected 'memcmp' behavior.
  // Below test case is equivalent to https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189#c7
  char a[] = "\0abc";
  int res = memcmp(a, "\0\0\0\0", 4);
  printf("memcmp result %d\n", res);
  // If the 'memcmp' bug exists, below will return 1.
  return (res == 0);
}
