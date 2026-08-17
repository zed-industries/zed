// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2015 Klaralvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author David Faure <david.faure@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QMIMEDATABASE_H
#define QMIMEDATABASE_H

#include <QtCore/qmimetype.h>

QT_REQUIRE_CONFIG(mimetype);

#include <QtCore/qstringlist.h>

QT_BEGIN_NAMESPACE

class QByteArray;
class QFileInfo;
class QIODevice;
class QUrl;

class QMimeDatabasePrivate;
class Q_CORE_EXPORT QMimeDatabase
{
    Q_DISABLE_COPY(QMimeDatabase)

public:
    QMimeDatabase();
    ~QMimeDatabase();

    QMimeType mimeTypeForName(const QString &nameOrAlias) const;

    enum MatchMode {
        MatchDefault = 0x0,
        MatchExtension = 0x1,
        MatchContent = 0x2
    };

    QMimeType mimeTypeForFile(const QString &fileName, MatchMode mode = MatchDefault) const;
    QMimeType mimeTypeForFile(const QFileInfo &fileInfo, MatchMode mode = MatchDefault) const;
    QList<QMimeType> mimeTypesForFileName(const QString &fileName) const;

    QMimeType mimeTypeForData(const QByteArray &data) const;
    QMimeType mimeTypeForData(QIODevice *device) const;

    QMimeType mimeTypeForUrl(const QUrl &url) const;
    QMimeType mimeTypeForFileNameAndData(const QString &fileName, QIODevice *device) const;
    QMimeType mimeTypeForFileNameAndData(const QString &fileName, const QByteArray &data) const;

    QString suffixForFileName(const QString &fileName) const;

    QList<QMimeType> allMimeTypes() const;

private:
    QMimeDatabasePrivate *d;
};

QT_END_NAMESPACE

#endif // QMIMEDATABASE_H
