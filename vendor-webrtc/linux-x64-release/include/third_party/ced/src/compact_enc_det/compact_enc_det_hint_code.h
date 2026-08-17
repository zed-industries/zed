// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

#ifndef COMPACT_ENC_DET_COMPACT_ENC_DET_HINT_CODE_H_
#define COMPACT_ENC_DET_COMPACT_ENC_DET_HINT_CODE_H_

#include <string>                        // for string

#include "util/basictypes.h"             // for uint32
#include "util/encodings/encodings.h"    // for Encoding

using std::string;

// Return name for extended encoding
const char* MyEncodingName(Encoding enc);

// Normalize ASCII string to first 4 alphabetic chars and last 4 digit chars
// Letters are forced to lowercase ASCII
// Used to normalize charset= values
string MakeChar44(const string& str);

// Normalize ASCII string to first 4 alphabetic/digit chars
// Letters are forced to lowercase ASCII
// Used to normalize TLD values
string MakeChar4(const string& str);

// Normalize ASCII string to first 8 alphabetic/digit chars
// Letters are forced to lowercase ASCII
// Used to normalize other values
string MakeChar8(const string& str);

#endif  // COMPACT_ENC_DET_COMPACT_ENC_DET_HINT_CODE_H_
