// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBXML_CHROMIUM_LIBXML_UTILS_H_
#define THIRD_PARTY_LIBXML_CHROMIUM_LIBXML_UTILS_H_

#include <libxml/xmlreader.h>

#include <string>

// libxml uses a global error function pointer for reporting errors.
// A ScopedXmlErrorFunc object lets you change the global error pointer
// for the duration of the object's lifetime.
class ScopedXmlErrorFunc {
 public:
  ScopedXmlErrorFunc(void* context, xmlGenericErrorFunc func) {
    old_error_func_ = xmlGenericError;
    old_error_context_ = xmlGenericErrorContext;
    xmlSetGenericErrorFunc(context, func);
  }
  ~ScopedXmlErrorFunc() {
    xmlSetGenericErrorFunc(old_error_context_, old_error_func_);
  }

 private:
  xmlGenericErrorFunc old_error_func_;
  void* old_error_context_;
};

namespace internal {

// Converts a libxml xmlChar* into a UTF-8 std::string.
// Null inputs produce an empty string.
std::string XmlStringToStdString(const xmlChar* xmlstring);

}  // namespace internal

#endif  // THIRD_PARTY_LIBXML_CHROMIUM_INCLUDE_LIBXML_LIBXML_UTILS_H_
