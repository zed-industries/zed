/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

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
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
        if (count == 0) // !isSharable
            return false;
#endif
        if (count != -1) // !isStatic
            atomic.ref();
        return true;
    }

    inline bool deref() noexcept {
        int count = atomic.loadRelaxed();
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
        if (count == 0) // !isSharable
            return false;
#endif
        if (count == -1) // isStatic
            return true;
        return atomic.deref();
    }

#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
    bool setSharable(bool sharable) noexcept
    {
        Q_ASSERT(!isShared());
        if (sharable)
            return atomic.testAndSetRelaxed(0, 1);
        else
            return atomic.testAndSetRelaxed(1, 0);
    }

    bool isSharable() const noexcept
    {
        // Sharable === Shared ownership.
        return atomic.loadRelaxed() != 0;
    }
#endif

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
