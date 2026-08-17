// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_TEXT_CHECK_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_TEXT_CHECK_CLIENT_H_

#include <memory>
#include <vector>

#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/web/web_text_checking_completion.h"

namespace blink {

class WebTextCheckClient {
 public:
  // Returns the Chromium setting of whether spell-checking is enabled.
  virtual bool IsSpellCheckingEnabled() const { return false; }

  // The client should perform spell-checking on the given text. If the
  // text contains a misspelled word, then upon return misspelledOffset
  // will point to the start of the misspelled word, and misspelledLength
  // will indicates its length. Otherwise, if there was not a spelling
  // error, then upon return misspelledLength is 0. If optional_suggestions
  // is given, then it will be filled with suggested words (not a cheap step).
  virtual void CheckSpelling(const WebString& text,
                             size_t& misspelled_offset,
                             size_t& misspelled_length,
                             std::vector<WebString>* optional_suggestions) {}

  // Requests asynchronous spelling and grammar checking, whose result should be
  // returned by passed completion object.
  virtual void RequestCheckingOfText(
      const WebString& text_to_check,
      std::unique_ptr<WebTextCheckingCompletion> completion_callback) {}

 protected:
  virtual ~WebTextCheckClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_TEXT_CHECK_CLIENT_H_
