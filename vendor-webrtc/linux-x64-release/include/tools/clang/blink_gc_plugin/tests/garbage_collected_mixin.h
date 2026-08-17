// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef GARBAGE_COLLECTED_MIXIN_H_
#define GARBAGE_COLLECTED_MIXIN_H_

#include "heap/stubs.h"

namespace blink {

class Mixin : public GarbageCollectedMixin {
public:
 virtual void Trace(Visitor*) const override;

private:
    Member<Mixin> m_self;
};

class HeapObject : public GarbageCollected<HeapObject>, public Mixin {
public:
 virtual void Trace(Visitor*) const override;

private:
    Member<Mixin> m_mix;
};

}

#endif
