// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSMETATYPE_H
#define QDBUSMETATYPE_H

#include <QtDBus/qtdbusglobal.h>
#include "QtCore/qmetatype.h"
#include <QtDBus/qdbusargument.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class Q_DBUS_EXPORT QDBusMetaType
{
public:
    typedef void (*MarshallFunction)(QDBusArgument &, const void *);
    typedef void (*DemarshallFunction)(const QDBusArgument &, void *);

    static void registerMarshallOperators(QMetaType typeId, MarshallFunction, DemarshallFunction);
    static bool marshall(QDBusArgument &, QMetaType id, const void *data);
    static bool demarshall(const QDBusArgument &, QMetaType id, void *data);

    static void registerCustomType(QMetaType type, const QByteArray &signature);

    static QMetaType signatureToMetaType(const char *signature);
    static const char *typeToSignature(QMetaType type);
};

template<typename T>
QMetaType qDBusRegisterMetaType()
{
    auto mf = [](QDBusArgument &arg, const void *t) { arg << *static_cast<const T *>(t); };
    auto df = [](const QDBusArgument &arg, void *t) { arg >> *static_cast<T *>(t); };

    QMetaType metaType = QMetaType::fromType<T>();
    QDBusMetaType::registerMarshallOperators(metaType, mf, df);
    return metaType;
}

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
