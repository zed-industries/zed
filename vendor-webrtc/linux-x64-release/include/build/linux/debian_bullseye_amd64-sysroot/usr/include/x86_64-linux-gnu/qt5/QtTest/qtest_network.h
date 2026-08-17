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

#ifndef QTEST_NETWORK_H
#define QTEST_NETWORK_H

#include <QtTest/qtest.h>

// enable NETWORK features
#ifndef QT_NETWORK_LIB
#define QT_NETWORK_LIB
#endif

#if 0
#pragma qt_class(QtTestNetwork)
#endif

#include <QtNetwork/QHostAddress>
#include <QtNetwork/QNetworkCookie>
#include <QtNetwork/QNetworkReply>

#if 0
// inform syncqt
#pragma qt_no_master_include
#endif

QT_BEGIN_NAMESPACE

namespace QTest
{
template<>
inline char *toString<QHostAddress>(const QHostAddress &addr)
{
    switch (addr.protocol()) {
    case QAbstractSocket::UnknownNetworkLayerProtocol:
        return qstrdup("<unknown address (parse error)>");
    case QAbstractSocket::AnyIPProtocol:
        return qstrdup("QHostAddress::Any");
    case QAbstractSocket::IPv4Protocol:
    case QAbstractSocket::IPv6Protocol:
        break;
    }

    return toString(addr.toString());
}

inline char *toString(QNetworkReply::NetworkError code)
{
    const QMetaObject *mo = &QNetworkReply::staticMetaObject;
    int index = mo->indexOfEnumerator("NetworkError");
    if (index == -1)
        return qstrdup("");

    QMetaEnum qme = mo->enumerator(index);
    return qstrdup(qme.valueToKey(code));
}

inline char *toString(const QNetworkCookie &cookie)
{
    return toString(cookie.toRawForm());
}

inline char *toString(const QList<QNetworkCookie> &list)
{
    QByteArray result = "QList(";
    if (!list.isEmpty()) {
        for (const QNetworkCookie &cookie : list)
            result += "QNetworkCookie(" + cookie.toRawForm() + "), ";
        result.chop(2); // remove trailing ", "
    }
    result.append(')');
    return toString(result);
}

} // namespace QTest

QT_END_NAMESPACE

#endif
