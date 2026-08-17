// Copyright 2006 Google LLC
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

#import <Foundation/Foundation.h>

#import "HTTPRequest.h"
/**
 Represents a multipart/form-data HTTP upload (POST request).
 Each parameter pair is sent as a boundary.
 Each file is sent with a name field in addition to the filename and data.
 */
@interface HTTPMultipartUpload : HTTPRequest {
 @protected
  NSDictionary* parameters_;    // The key/value pairs for sending data (STRONG)
  NSMutableDictionary* files_;  // Dictionary of name/file-path (STRONG)
  NSString* boundary_;          // The boundary string (STRONG)
}

/**
 Sets the parameters that will be sent in the multipart POST request.
 */
- (void)setParameters:(NSDictionary*)parameters;
- (NSDictionary*)parameters;

/**
 Adds a file to be uploaded in the multipart POST request, by its file path.
 */
- (void)addFileAtPath:(NSString*)path name:(NSString*)name;

/**
 Adds a file to be uploaded in the multipart POST request, by its name and
 contents.
 */
- (void)addFileContents:(NSData*)data name:(NSString*)name;
- (NSDictionary*)files;

@end
