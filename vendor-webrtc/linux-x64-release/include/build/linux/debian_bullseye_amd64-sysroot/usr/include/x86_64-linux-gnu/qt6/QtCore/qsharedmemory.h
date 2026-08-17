// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSHAREDMEMORY_H
#define QSHAREDMEMORY_H

#include <QtCore/qglobal.h>
#ifndef QT_NO_QOBJECT
# include <QtCore/qobject.h>
#else
# include <QtCore/qobjectdefs.h>
# include <QtCore/qscopedpointer.h>
# include <QtCore/qstring.h>
#endif
QT_BEGIN_NAMESPACE


#ifndef QT_NO_SHAREDMEMORY

class QSharedMemoryPrivate;

class Q_CORE_EXPORT QSharedMemory
#ifndef QT_NO_QOBJECT
    : public QObject
#endif
{
#ifndef QT_NO_QOBJECT
    Q_OBJECT
#endif
    Q_DECLARE_PRIVATE(QSharedMemory)

public:
    enum AccessMode
    {
        ReadOnly,
        ReadWrite
    };

    enum SharedMemoryError
    {
        NoError,
        PermissionDenied,
        InvalidSize,
        KeyError,
        AlreadyExists,
        NotFound,
        LockError,
        OutOfResources,
        UnknownError
    };

#ifndef QT_NO_QOBJECT
    QSharedMemory(QObject *parent = nullptr);
    QSharedMemory(const QString &key, QObject *parent = nullptr);
#else
    QSharedMemory();
    QSharedMemory(const QString &key);
    static QString tr(const char * str)
    {
        return QString::fromLatin1(str);
    }
#endif
    ~QSharedMemory();

    void setKey(const QString &key);
    QString key() const;
    void setNativeKey(const QString &key);
    QString nativeKey() const;

    bool create(qsizetype size, AccessMode mode = ReadWrite);
    qsizetype size() const;

    bool attach(AccessMode mode = ReadWrite);
    bool isAttached() const;
    bool detach();

    void *data();
    const void* constData() const;
    const void *data() const;

#ifndef QT_NO_SYSTEMSEMAPHORE
    bool lock();
    bool unlock();
#endif

    SharedMemoryError error() const;
    QString errorString() const;

private:
    Q_DISABLE_COPY(QSharedMemory)
#ifdef QT_NO_QOBJECT
    QScopedPointer<QSharedMemoryPrivate> d_ptr;
#endif
};

#endif // QT_NO_SHAREDMEMORY

QT_END_NAMESPACE

#endif // QSHAREDMEMORY_H

