// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_STATE_PAGE_STATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_STATE_PAGE_STATE_H_

#include <stdint.h>

#include <string>
#include <vector>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

class GURL;

namespace base {
class FilePath;
}

namespace blink {

// The PageState class represents the information needed by the rendering
// engine to reconstruct a web page (and its tree of frames), including for
// example the URLs of the documents and the values of any form fields.  This
// information is used when navigating back & forward through session history.
//
// The format of the encoded data is not exposed by the content API.
class BLINK_COMMON_EXPORT PageState {
 public:
  static PageState CreateFromEncodedData(const std::string& data);
  static PageState CreateFromURL(const GURL& url);

  static PageState CreateForTesting(
      const GURL& url,
      bool body_contains_password_data,
      const char* optional_body_data,
      const base::FilePath* optional_body_file_path);

  // Creates an encoded page state from the |url|, |item_sequence_mnumber| and
  // |document_sequence_number| parameters.
  static PageState CreateForTestingWithSequenceNumbers(
      const GURL& url,
      int64_t item_sequence_number,
      int64_t document_sequence_number);

  PageState();

  bool IsValid() const;
  bool Equals(const PageState& page_state) const;
  const std::string& ToEncodedData() const;

  std::vector<base::FilePath> GetReferencedFiles() const;
  PageState RemovePasswordData() const;
  PageState RemoveScrollOffset() const;
  PageState RemoveReferrer() const;

  // Support DCHECK_EQ(a, b), etc.
  bool operator==(const PageState& other) const { return this->Equals(other); }
  bool operator!=(const PageState& other) const {
    return !(this->Equals(other));
  }

  void WriteIntoTrace(perfetto::TracedValue context) const;

 private:
  PageState(const std::string& data);

  std::string data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_STATE_PAGE_STATE_H_
