// Copyright 2010 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Implements a Tokenize function for splitting up strings.

#ifndef GOOGLE_BREAKPAD_PROCESSOR_TOKENIZE_H_
#define GOOGLE_BREAKPAD_PROCESSOR_TOKENIZE_H_

#include <string>
#include <vector>

#include "common/using_std_string.h"

namespace google_breakpad {

// Splits line into at most max_tokens tokens, separated by any of the
// characters in separators and placing them in the tokens vector.
// line is a 0-terminated string that optionally ends with a newline
// character or combination, which will be removed. 
// If more tokens than max_tokens are present, the final token is placed
// into the vector without splitting it up at all.  This modifies line as
// a side effect.  Returns true if exactly max_tokens tokens are returned,
// and false if fewer are returned.  This is not considered a failure of
// Tokenize, but may be treated as a failure if the caller expects an
// exact, as opposed to maximum, number of tokens.

bool Tokenize(char* line,
              const char* separators,
              int max_tokens,
              std::vector<char*>* tokens);
// For convenience, since you need a char* to pass to Tokenize.
// You can call StringToVector on a string, and use &vec[0].
void StringToVector(const string& str, std::vector<char>& vec);

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_TOKENIZE_H_
