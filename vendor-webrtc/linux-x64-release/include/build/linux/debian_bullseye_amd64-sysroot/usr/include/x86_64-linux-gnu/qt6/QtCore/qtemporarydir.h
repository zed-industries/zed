// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTEMPORARYDIR_H
#define QTEMPORARYDIR_H

#include <QtCore/qglobal.h>
#include <QtCore/qdir.h>
#include <QtCore/qscopedpointer.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_TEMPORARYFILE

class QTemporaryDirPrivate;

class Q_CORE_EXPORT QTemporaryDir
{
public:
    QTemporaryDir();
    explicit QTemporaryDir(const QString &templateName);
    QTemporaryDir(QTemporaryDir &&other) noexcept
        : d_ptr{std::exchange(other.d_ptr, nullptr)}
    { }

    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QTemporaryDir)

    ~QTemporaryDir();

    void swap(QTemporaryDir &other) noexcept
    { qt_ptr_swap(d_ptr, other.d_ptr); }

    bool isValid() const;
    QString errorString() const;

    bool autoRemove() const;
    void setAutoRemove(bool b);
    bool remove();

    QString path() const;
    QString filePath(const QString &fileName) const;

private:
    QTemporaryDirPrivate *d_ptr;

    Q_DISABLE_COPY(QTemporaryDir)
};

inline void swap(QTemporaryDir &lhs, QTemporaryDir &rhs) noexcept
{
    lhs.swap(rhs);
}

#endif // QT_NO_TEMPORARYFILE

QT_END_NAMESPACE

#endif // QTEMPORARYDIR_H
