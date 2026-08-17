// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSVIRTUALOBJECT_H
#define QDBUSVIRTUALOBJECT_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qobject.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class QDBusMessage;
class QDBusConnection;

class Q_DBUS_EXPORT QDBusVirtualObject : public QObject
{
    Q_OBJECT
public:
    explicit QDBusVirtualObject(QObject *parent = nullptr);
    virtual ~QDBusVirtualObject();

    virtual QString introspect(const QString &path) const = 0;
    virtual bool handleMessage(const QDBusMessage &message, const QDBusConnection &connection) = 0;

private:
    Q_DISABLE_COPY(QDBusVirtualObject)
};

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
