/****************************************************************************
**
** Copyright (C) 2017 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtNetwork module of the Qt Toolkit.
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

#ifndef QHSTSPOLICY_H
#define QHSTSPOLICY_H

#include <QtNetwork/qtnetworkglobal.h>

#include <QtCore/qshareddata.h>
#include <QtCore/qflags.h>
#include <QtCore/qurl.h>

QT_BEGIN_NAMESPACE

class QHstsPolicyPrivate;
class QDateTime;
class QString;
class Q_NETWORK_EXPORT QHstsPolicy
{
public:
    enum PolicyFlag
    {
        IncludeSubDomains = 1
    };
    Q_DECLARE_FLAGS(PolicyFlags, PolicyFlag)

    QHstsPolicy();
    QHstsPolicy(const QDateTime &expiry, PolicyFlags flags, const QString &host,
                QUrl::ParsingMode mode = QUrl::DecodedMode);
    QHstsPolicy(const QHstsPolicy &rhs);
    QHstsPolicy &operator=(const QHstsPolicy &rhs);
    QHstsPolicy &operator=(QHstsPolicy &&other) noexcept { swap(other); return *this; }
    ~QHstsPolicy();

    void swap(QHstsPolicy &other) noexcept { qSwap(d, other.d); }

    void setHost(const QString &host, QUrl::ParsingMode mode = QUrl::DecodedMode);
    QString host(QUrl::ComponentFormattingOptions options = QUrl::FullyDecoded) const;
    void setExpiry(const QDateTime &expiry);
    QDateTime expiry() const;
    void setIncludesSubDomains(bool include);
    bool includesSubDomains() const;

    bool isExpired() const;

private:

    QSharedDataPointer<QHstsPolicyPrivate> d;

    friend Q_NETWORK_EXPORT bool operator==(const QHstsPolicy &lhs, const QHstsPolicy &rhs);
};

Q_DECLARE_SHARED(QHstsPolicy)
Q_DECLARE_OPERATORS_FOR_FLAGS(QHstsPolicy::PolicyFlags)

Q_NETWORK_EXPORT bool operator==(const QHstsPolicy &lhs, const QHstsPolicy &rhs);

inline bool operator!=(const QHstsPolicy &lhs, const QHstsPolicy &rhs)
{
    return !(lhs == rhs);
}


QT_END_NAMESPACE

#endif // QHSTSPOLICY_H
