// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_V8_VALUE_CONVERTER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_V8_VALUE_CONVERTER_H_

#include <memory>

#include "base/functional/callback.h"
#include "third_party/blink/public/platform/web_common.h"
#include "v8/include/v8-forward.h"

namespace base {
class Value;
class ValueView;
}

namespace blink {

// Converts between v8::Value (JavaScript values in the v8 heap) and Chrome's
// values (from base/values.h). Lists and dictionaries are converted
// recursively.
class BLINK_EXPORT WebV8ValueConverter {
 public:
  virtual ~WebV8ValueConverter() = default;

  // If true, Date objects are converted into DoubleValues with the number of
  // seconds since Unix epoch.
  //
  // Otherwise they are converted into DictionaryValues with whatever additional
  // properties has been set on them.
  virtual void SetDateAllowed(bool val) = 0;

  // If true, RegExp objects are converted into StringValues with the regular
  // expression between / and /, for example "/ab?c/".
  //
  // Otherwise they are converted into DictionaryValues with whatever additional
  // properties has been set on them.
  virtual void SetRegExpAllowed(bool val) = 0;

  // Converts a base::Value to a v8::Value.
  //
  // Unsupported types are replaced with null.  If an array or object throws
  // while setting a value, that property or item is skipped, leaving a hole in
  // the case of arrays.
  // TODO(dcheng): This should just take a const reference.
  virtual v8::Local<v8::Value> ToV8Value(base::ValueView value,
                                         v8::Local<v8::Context> context) = 0;

  // Converts a v8::Value to base::Value.
  //
  // Unsupported types (unless explicitly configured) are not converted, so
  // this method may return NULL -- the exception is when converting arrays,
  // where unsupported types are converted to Value(Type::NONE).
  //
  // Likewise, if an object throws while converting a property it will not be
  // converted, whereas if an array throws while converting an item it will be
  // converted to Value(Type::NONE).
  virtual std::unique_ptr<base::Value> FromV8Value(
      v8::Local<v8::Value> value,
      v8::Local<v8::Context> context) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_V8_VALUE_CONVERTER_H_
