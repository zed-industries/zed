// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_MESSAGE_H_
#define PHP_PROTOBUF_MESSAGE_H_

#include <stdbool.h>

#include "def.h"

// Registers the PHP Message class.
void Message_ModuleInit();

// Gets a upb_Message* for the PHP object |val|, which must either be a Message
// object or 'null'. Returns true and stores the message in |msg| if the
// conversion succeeded (we can't return upb_Message* because null->NULL is a
// valid conversion). Returns false and raises a PHP error if this isn't a
// Message object or null, or if the Message object doesn't match this
// Descriptor.
//
// The given |arena| will be fused to this message's arena.
bool Message_GetUpbMessage(zval* val, const Descriptor* desc, upb_Arena* arena,
                           upb_Message** msg);

// Gets or creates a PHP Message object to wrap the given upb_Message* and
// |desc| and returns it in |val|. The PHP object will keep a reference to this
// |arena| to ensure the underlying message data stays alive.
//
// If |msg| is NULL, this will return a PHP null.
void Message_GetPhpWrapper(zval* val, const Descriptor* desc, upb_Message* msg,
                           zval* arena);

bool ValueEq(upb_MessageValue val1, upb_MessageValue val2, TypeInfo type);

#endif  // PHP_PROTOBUF_MESSAGE_H_
