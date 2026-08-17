// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QDBUSUNIXFILEDESCRIPTOR_H
#define QDBUSUNIXFILEDESCRIPTOR_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qshareddata.h>

#ifndef QT_NO_DBUS

#include <utility>

QT_BEGIN_NAMESPACE


class QDBusUnixFileDescriptorPrivate;

class Q_DBUS_EXPORT QDBusUnixFileDescriptor
{
public:
    QDBusUnixFileDescriptor();
    explicit QDBusUnixFileDescriptor(int fileDescriptor);
    QDBusUnixFileDescriptor(const QDBusUnixFileDescriptor &other);
    QDBusUnixFileDescriptor &operator=(QDBusUnixFileDescriptor &&other) noexcept { swap(other); return *this; }
    QDBusUnixFileDescriptor &operator=(const QDBusUnixFileDescriptor &other);
    ~QDBusUnixFileDescriptor();

    void swap(QDBusUnixFileDescriptor &other) noexcept
    { d.swap(other.d); }

    bool isValid() const;

    int fileDescriptor() const;
    void setFileDescriptor(int fileDescriptor);

    void giveFileDescriptor(int fileDescriptor);
    int takeFileDescriptor();

    static bool isSupported();

protected:
    typedef QExplicitlySharedDataPointer<QDBusUnixFileDescriptorPrivate>  Data;
    Data d;
};

Q_DECLARE_SHARED(QDBusUnixFileDescriptor)

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QDBusUnixFileDescriptor, Q_DBUS_EXPORT)

#endif // QT_NO_DBUS
#endif // QDBUSUNIXFILEDESCRIPTOR_H
