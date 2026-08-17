// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFILESYSTEMWATCHER_H
#define QFILESYSTEMWATCHER_H

#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(filesystemwatcher);

QT_BEGIN_NAMESPACE


class QFileSystemWatcherPrivate;

class Q_CORE_EXPORT QFileSystemWatcher : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QFileSystemWatcher)

public:
    QFileSystemWatcher(QObject *parent = nullptr);
    QFileSystemWatcher(const QStringList &paths, QObject *parent = nullptr);
    ~QFileSystemWatcher();

    bool addPath(const QString &file);
    QStringList addPaths(const QStringList &files);
    bool removePath(const QString &file);
    QStringList removePaths(const QStringList &files);

    QStringList files() const;
    QStringList directories() const;

Q_SIGNALS:
    void fileChanged(const QString &path, QPrivateSignal);
    void directoryChanged(const QString &path, QPrivateSignal);

private:
    Q_PRIVATE_SLOT(d_func(), void _q_fileChanged(const QString &path, bool removed))
    Q_PRIVATE_SLOT(d_func(), void _q_directoryChanged(const QString &path, bool removed))
};

QT_END_NAMESPACE

#endif // QFILESYSTEMWATCHER_H
