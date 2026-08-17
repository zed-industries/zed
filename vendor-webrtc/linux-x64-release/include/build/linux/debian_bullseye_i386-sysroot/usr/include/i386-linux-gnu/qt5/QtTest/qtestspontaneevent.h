/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtTest module of the Qt Toolkit.
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

#ifndef QTESTSPONTANEEVENT_H
#define QTESTSPONTANEEVENT_H

#include <QtCore/qcoreevent.h>

#if 0
// inform syncqt
#pragma qt_no_master_include
#endif

QT_BEGIN_NAMESPACE


#ifndef QTEST_NO_SIZEOF_CHECK
template <int>
class QEventSizeOfChecker
{
private:
    QEventSizeOfChecker() {}
};

template <>
class QEventSizeOfChecker<sizeof(QEvent)>
{
public:
    QEventSizeOfChecker() {}
};
#endif

class QSpontaneKeyEvent
{
public:
    void setSpontaneous() { spont = 1; Q_UNUSED(posted) Q_UNUSED(m_accept) Q_UNUSED(reserved) }
    bool spontaneous() { return spont; }
    virtual void dummyFunc() {}
    virtual ~QSpontaneKeyEvent() {}

#ifndef QTEST_NO_SIZEOF_CHECK
    inline void ifYouGetCompileErrorHereYouUseWrongQt()
    {
        // this is a static assert in case QEvent changed in Qt
        QEventSizeOfChecker<sizeof(QSpontaneKeyEvent)> dummy;
    }
#endif

    // ### Qt 6: remove everything except this function:
    static inline void setSpontaneous(QEvent *ev)
    {
        ev->setSpontaneous();
    }

protected:
    void *d;
    ushort t;

private:
    ushort posted : 1;
    ushort spont : 1;
    ushort m_accept : 1;
    ushort reserved : 13;
};

QT_END_NAMESPACE

#endif
