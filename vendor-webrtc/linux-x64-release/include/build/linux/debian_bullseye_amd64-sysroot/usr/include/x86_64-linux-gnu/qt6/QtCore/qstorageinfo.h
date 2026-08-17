// Copyright (C) 2014 Ivan Komissarov <ABBAPOH@gmail.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTORAGEINFO_H
#define QSTORAGEINFO_H

#include <QtCore/qbytearray.h>
#include <QtCore/qdir.h>
#include <QtCore/qlist.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qstring.h>
#include <QtCore/qshareddata.h>

QT_BEGIN_NAMESPACE

class QDebug;

class QStorageInfoPrivate;
class Q_CORE_EXPORT QStorageInfo
{
public:
    QStorageInfo();
    explicit QStorageInfo(const QString &path);
    explicit QStorageInfo(const QDir &dir);
    QStorageInfo(const QStorageInfo &other);
    ~QStorageInfo();

    QStorageInfo &operator=(const QStorageInfo &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QStorageInfo)

    inline void swap(QStorageInfo &other) noexcept
    { d.swap(other.d); }

    void setPath(const QString &path);

    QString rootPath() const;
    QByteArray device() const;
    QByteArray subvolume() const;
    QByteArray fileSystemType() const;
    QString name() const;
    QString displayName() const;

    qint64 bytesTotal() const;
    qint64 bytesFree() const;
    qint64 bytesAvailable() const;
    int blockSize() const;

    inline bool isRoot() const;
    bool isReadOnly() const;
    bool isReady() const;
    bool isValid() const;

    void refresh();

    static QList<QStorageInfo> mountedVolumes();
    static QStorageInfo root();

private:
    friend class QStorageInfoPrivate;
    friend inline bool operator==(const QStorageInfo &first, const QStorageInfo &second)
    {
        if (first.d == second.d)
            return true;
        return first.device() == second.device() && first.rootPath() == second.rootPath();
    }

    friend inline bool operator!=(const QStorageInfo &first, const QStorageInfo &second)
    {
        return !(first == second);
    }

    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QStorageInfo &);
    QExplicitlySharedDataPointer<QStorageInfoPrivate> d;
};

inline bool QStorageInfo::isRoot() const
{ return *this == QStorageInfo::root(); }

Q_DECLARE_SHARED(QStorageInfo)

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug debug, const QStorageInfo &);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QStorageInfo, Q_CORE_EXPORT)

#endif // QSTORAGEINFO_H
