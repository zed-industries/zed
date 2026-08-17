/*
 * Copyright 2005 Frerich Raabe <raabe@kde.org>
 * Copyright (C) 2006 Apple Computer, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_UTIL_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Node;

namespace xpath {

// @return whether the given node is the root node
bool IsRootDomNode(Node*);

// @return the 'string-value' of the given node as specified by
// http://www.w3.org/TR/xpath
WTF::String StringValue(Node*);

// @return whether the given node is a valid context node
bool IsValidContextNode(Node*);

// https://www.w3.org/TR/REC-xml/#NT-S
bool IsXMLSpace(UChar ch);

}  // namespace xpath

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_UTIL_H_
