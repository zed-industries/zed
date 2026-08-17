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

#include <QtCore/qglobal.h>

#ifndef QOPERATINGSYSTEMVERSION_H
#define QOPERATINGSYSTEMVERSION_H

QT_BEGIN_NAMESPACE

class QString;
class QVersionNumber;

class Q_CORE_EXPORT QOperatingSystemVersion
{
public:
    enum OSType {
        Unknown = 0,
        Windows,
        MacOS,
        IOS,
        TvOS,
        WatchOS,
        Android
    };

    static const QOperatingSystemVersion Windows7;
    static const QOperatingSystemVersion Windows8;
    static const QOperatingSystemVersion Windows8_1;
    static const QOperatingSystemVersion Windows10;

    static const QOperatingSystemVersion OSXMavericks;
    static const QOperatingSystemVersion OSXYosemite;
    static const QOperatingSystemVersion OSXElCapitan;
    static const QOperatingSystemVersion MacOSSierra;
    static const QOperatingSystemVersion MacOSHighSierra;
    static const QOperatingSystemVersion MacOSMojave;
    static const QOperatingSystemVersion MacOSCatalina;
    static const QOperatingSystemVersion MacOSBigSur;

    static const QOperatingSystemVersion AndroidJellyBean;
    static const QOperatingSystemVersion AndroidJellyBean_MR1;
    static const QOperatingSystemVersion AndroidJellyBean_MR2;
    static const QOperatingSystemVersion AndroidKitKat;
    static const QOperatingSystemVersion AndroidLollipop;
    static const QOperatingSystemVersion AndroidLollipop_MR1;
    static const QOperatingSystemVersion AndroidMarshmallow;
    static const QOperatingSystemVersion AndroidNougat;
    static const QOperatingSystemVersion AndroidNougat_MR1;
    static const QOperatingSystemVersion AndroidOreo;

    Q_DECL_CONSTEXPR QOperatingSystemVersion(OSType osType,
                                             int vmajor, int vminor = -1, int vmicro = -1)
        : m_os(osType),
          m_major(qMax(-1, vmajor)),
          m_minor(vmajor < 0 ? -1 : qMax(-1, vminor)),
          m_micro(vmajor < 0 || vminor < 0 ? -1 : qMax(-1, vmicro))
    { }

    static QOperatingSystemVersion current();

    static Q_DECL_CONSTEXPR OSType currentType()
    {
#if defined(Q_OS_WIN)
        return Windows;
#elif defined(Q_OS_MACOS)
        return MacOS;
#elif defined(Q_OS_IOS)
        return IOS;
#elif defined(Q_OS_TVOS)
        return TvOS;
#elif defined(Q_OS_WATCHOS)
        return WatchOS;
#elif defined(Q_OS_ANDROID)
        return Android;
#else
        return Unknown;
#endif
    }

    Q_DECL_CONSTEXPR int majorVersion() const { return m_major; }
    Q_DECL_CONSTEXPR int minorVersion() const { return m_minor; }
    Q_DECL_CONSTEXPR int microVersion() const { return m_micro; }

    Q_DECL_CONSTEXPR int segmentCount() const
    { return m_micro >= 0 ? 3 : m_minor >= 0 ? 2 : m_major >= 0 ? 1 : 0; }

    bool isAnyOfType(std::initializer_list<OSType> types) const;
    Q_DECL_CONSTEXPR OSType type() const { return m_os; }
    QString name() const;

    friend bool operator>(const QOperatingSystemVersion &lhs, const QOperatingSystemVersion &rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersion::compare(lhs, rhs) > 0; }

    friend bool operator>=(const QOperatingSystemVersion &lhs, const QOperatingSystemVersion &rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersion::compare(lhs, rhs) >= 0; }

    friend bool operator<(const QOperatingSystemVersion &lhs, const QOperatingSystemVersion &rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersion::compare(lhs, rhs) < 0; }

    friend bool operator<=(const QOperatingSystemVersion &lhs, const QOperatingSystemVersion &rhs)
    { return lhs.type() == rhs.type() && QOperatingSystemVersion::compare(lhs, rhs) <= 0; }

private:
    QOperatingSystemVersion() = default;
    OSType m_os;
    int m_major;
    int m_minor;
    int m_micro;

    static int compare(const QOperatingSystemVersion &v1, const QOperatingSystemVersion &v2);
};
Q_DECLARE_TYPEINFO(QOperatingSystemVersion, QT_VERSION < QT_VERSION_CHECK(6, 0, 0) ? Q_RELOCATABLE_TYPE : Q_PRIMITIVE_TYPE);

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_CORE_EXPORT QDebug operator<<(QDebug debug, const QOperatingSystemVersion &ov);
#endif

QT_END_NAMESPACE

#endif // QOPERATINGSYSTEMVERSION_H
