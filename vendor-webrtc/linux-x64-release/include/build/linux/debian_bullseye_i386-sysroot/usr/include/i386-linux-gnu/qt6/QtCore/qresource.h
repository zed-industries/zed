// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2019 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QRESOURCE_H
#define QRESOURCE_H

#include <QtCore/qstring.h>
#include <QtCore/qlocale.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qlist.h>
#include <QtCore/qscopedpointer.h>

QT_BEGIN_NAMESPACE


class QResourcePrivate;

class Q_CORE_EXPORT QResource
{
public:
    enum Compression {
        NoCompression,
        ZlibCompression,
        ZstdCompression
    };

    QResource(const QString &file = QString(), const QLocale &locale = QLocale());
    ~QResource();

    void setFileName(const QString &file);
    QString fileName() const;
    QString absoluteFilePath() const;

    void setLocale(const QLocale &locale);
    QLocale locale() const;

    bool isValid() const;

    Compression compressionAlgorithm() const;
    qint64 size() const;
    const uchar *data() const;
    qint64 uncompressedSize() const;
    QByteArray uncompressedData() const;
    QDateTime lastModified() const;

    static bool registerResource(const QString &rccFilename, const QString &resourceRoot=QString());
    static bool unregisterResource(const QString &rccFilename, const QString &resourceRoot=QString());

    static bool registerResource(const uchar *rccData, const QString &resourceRoot=QString());
    static bool unregisterResource(const uchar *rccData, const QString &resourceRoot=QString());

protected:
    friend class QResourceFileEngine;
    friend class QResourceFileEngineIterator;
    bool isDir() const;
    inline bool isFile() const { return !isDir(); }
    QStringList children() const;

protected:
    QScopedPointer<QResourcePrivate> d_ptr;

private:
    Q_DECLARE_PRIVATE(QResource)
};

QT_END_NAMESPACE

#endif // QRESOURCE_H
