/*
    Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_NAVIGATOR_MEDIA_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_NAVIGATOR_MEDIA_STREAM_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class MediaStreamConstraints;
class Navigator;
class V8NavigatorUserMediaErrorCallback;
class V8NavigatorUserMediaSuccessCallback;

class NavigatorMediaStream {
  STATIC_ONLY(NavigatorMediaStream);

 public:
  static void getUserMedia(Navigator&,
                           const MediaStreamConstraints*,
                           V8NavigatorUserMediaSuccessCallback*,
                           V8NavigatorUserMediaErrorCallback*,
                           ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_NAVIGATOR_MEDIA_STREAM_H_
