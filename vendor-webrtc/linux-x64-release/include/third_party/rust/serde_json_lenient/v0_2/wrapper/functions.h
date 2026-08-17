// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_RUST_SERDE_JSON_LENIENT_V0_2_WRAPPER_SERDE_JSON_LENIENT_H_
#define THIRD_PARTY_RUST_SERDE_JSON_LENIENT_V0_2_WRAPPER_SERDE_JSON_LENIENT_H_

#include <stdint.h>

#include "third_party/rust/cxx/v1/cxx.h"

namespace base {
class DictValue;
class ListValue;
}  // namespace base

namespace serde_json_lenient {

using Dict = base::DictValue;
using List = base::ListValue;

void list_append_none(List&);
void list_append_bool(List&, bool val);
void list_append_i32(List&, int32_t val);
void list_append_f64(List&, double val);
void list_append_str(List&, rust::Str val);
List& list_append_list(List&);
Dict& list_append_dict(List&);

void dict_set_none(Dict&, rust::Str key);
void dict_set_bool(Dict&, rust::Str key, bool val);
void dict_set_i32(Dict&, rust::Str key, int32_t val);
void dict_set_f64(Dict&, rust::Str key, double val);
void dict_set_str(Dict&, rust::Str key, rust::Str val);
List& dict_set_list(Dict&, rust::Str key);
Dict& dict_set_dict(Dict&, rust::Str key);

}  // namespace serde_json_lenient

#endif  // THIRD_PARTY_RUST_SERDE_JSON_LENIENT_V0_2_WRAPPER_SERDE_JSON_LENIENT_H_
