// Copyright (C) 2013 David Faure <faure+bluesystems@kde.org>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QLOCKFILE_H
#define QLOCKFILE_H

#include <QtCore/qstring.h>
#include <QtCore/qscopedpointer.h>

#include <chrono>

QT_BEGIN_NAMESPACE

class QLockFilePrivate;

class Q_CORE_EXPORT QLockFile
{
public:
    QLockFile(const QString &fileName);
    ~QLockFile();

    QString fileName() const;

    bool lock();
    bool tryLock(int timeout = 0);
    void unlock();

    void setStaleLockTime(int);
    int staleLockTime() const;

    bool tryLock(std::chrono::milliseconds timeout) { return tryLock(int(timeout.count())); }

    void setStaleLockTime(std::chrono::milliseconds value) { setStaleLockTime(int(value.count())); }

    std::chrono::milliseconds staleLockTimeAsDuration() const
    {
        return std::chrono::milliseconds(staleLockTime());
    }

    bool isLocked() const;
    bool getLockInfo(qint64 *pid, QString *hostname, QString *appname) const;
    bool removeStaleLockFile();

    enum LockError {
        NoError = 0,
        LockFailedError = 1,
        PermissionError = 2,
        UnknownError = 3
    };
    LockError error() const;

protected:
    QScopedPointer<QLockFilePrivate> d_ptr;

private:
    Q_DECLARE_PRIVATE(QLockFile)
    Q_DISABLE_COPY(QLockFile)
};

QT_END_NAMESPACE

#endif // QLOCKFILE_H
