// Copyright 2008 Google LLC
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
//  breakpad_nlist_test.h
//  minidump_test
//
//  Created by Neal Sidhwaney on 4/13/08.
//  Copyright 2008 Google LLC
//
//

#ifndef CLIENT_MAC_HANDLER_TESTCASES_BREAKPAD_NLIST_TEST_H__
#define CLIENT_MAC_HANDLER_TESTCASES_BREAKPAD_NLIST_TEST_H__

#include <CPlusTest/CPlusTest.h>

class BreakpadNlistTest : public TestCase {
 private:

  // nm dumps multiple addresses for the same symbol in
  // /usr/lib/dyld. So we track those so we don't report failures
  // in mismatches between what our nlist returns and what nm has
  // for the duplicate symbols.
  bool IsSymbolMoreThanOnceInDyld(const char* symbolName);

 public:
  explicit BreakpadNlistTest(TestInvocation* invocation);
  virtual ~BreakpadNlistTest();


  /* This test case runs nm on /usr/lib/dyld and then compares the
     output of every symbol to what our nlist implementation returns */
  void CompareToNM();
};

#endif /* CLIENT_MAC_HANDLER_TESTCASES_BREAKPAD_NLIST_TEST_H__*/
