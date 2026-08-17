/*
 * Copyright (C) 2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2008 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2009 - 2010  Torch Mobile (Beijing) Co. Ltd. All rights
 * reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MARKUP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MARKUP_H_

namespace WTF {
class AtomicString;
class String;
class StringBuilder;
}  // namespace WTF

// Helper functions for converting from CSSValues to text.

namespace blink {

// Common serializing methods. See:
// https://drafts.csswg.org/cssom/#common-serializing-idioms
void SerializeIdentifier(const WTF::String& identifier,
                         WTF::StringBuilder& append_to,
                         bool skip_start_checks = false);
void SerializeString(const WTF::String&, WTF::StringBuilder& append_to);
WTF::String SerializeString(const WTF::String&);
WTF::String SerializeURI(const WTF::String&);
WTF::String SerializeFontFamily(const WTF::AtomicString&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MARKUP_H_
