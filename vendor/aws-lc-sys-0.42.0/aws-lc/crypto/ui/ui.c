// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// AWS-LC does not support the UI APIs. These functions always fail at runtime.
// This file provides no-op implementations of several UI functions that return
// failure when called. This allows compilation to succeed for projects that use
// these functions for non-essential operations.

#include "openssl/ui.h"
#include "openssl/mem.h"

UI *UI_new(void) {
  return NULL;
}

void UI_free(UI *ui) {
  OPENSSL_free(ui);
}

int UI_add_input_string(UI *ui, const char *prompt, int flags,
        char *result_buf, int minsize, int maxsize) {
  return -1;
}
int UI_add_verify_string(UI *ui, const char *prompt, int flags,
        char *result_buf, int minsize, int maxsize, const char *test_buf) {
  return -1;
}

int UI_add_info_string(UI *ui, const char *text) {
  return -1;
}

int UI_process(UI *ui) {
  return -1;
}
