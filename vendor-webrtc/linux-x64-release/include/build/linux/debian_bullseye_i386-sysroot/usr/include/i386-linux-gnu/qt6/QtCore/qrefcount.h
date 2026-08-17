// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QREFCOUNT_H
#define QREFCOUNT_H

#include <QtCore/qatomic.h>

QT_BEGIN_NAMESPACE


namespace QtPrivate
{

class RefCount
{
public:
    inline bool ref() noexcept {
        int count = atomic.loadRelaxed();
        if (count != -1) // !isStatic
            atomic.ref();
        return true;
    }

    inline bool deref() noexcept {
        int count = atomic.loadRelaxed();
        if (count == -1) // isStatic
            return true;
        return atomic.deref();
    }

    bool isStatic() const noexcept
    {
        // Persistent object, never deleted
        return atomic.loadRelaxed() == -1;
    }

    bool isShared() const noexcept
    {
        int count = atomic.loadRelaxed();
        return (count != 1) && (count != 0);
    }

    void initializeOwned() noexcept { atomic.storeRelaxed(1); }
    void initializeUnsharable() noexcept { atomic.storeRelaxed(0); }

    QBasicAtomicInt atomic;
};

}

#define Q_REFCOUNT_INITIALIZE_STATIC { Q_BASIC_ATOMIC_INITIALIZER(-1) }

QT_END_NAMESPACE

#endif
