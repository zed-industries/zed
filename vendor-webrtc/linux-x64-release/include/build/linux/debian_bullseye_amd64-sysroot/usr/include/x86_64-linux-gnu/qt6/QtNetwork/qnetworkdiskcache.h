// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNETWORKDISKCACHE_H
#define QNETWORKDISKCACHE_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qabstractnetworkcache.h>

QT_REQUIRE_CONFIG(networkdiskcache);

QT_BEGIN_NAMESPACE

class QNetworkDiskCachePrivate;
class Q_NETWORK_EXPORT QNetworkDiskCache : public QAbstractNetworkCache
{
    Q_OBJECT

public:
    explicit QNetworkDiskCache(QObject *parent = nullptr);
    ~QNetworkDiskCache();

    QString cacheDirectory() const;
    void setCacheDirectory(const QString &cacheDir);

    qint64 maximumCacheSize() const;
    void setMaximumCacheSize(qint64 size);

    qint64 cacheSize() const override;
    QNetworkCacheMetaData metaData(const QUrl &url) override;
    void updateMetaData(const QNetworkCacheMetaData &metaData) override;
    QIODevice *data(const QUrl &url) override;
    bool remove(const QUrl &url) override;
    QIODevice *prepare(const QNetworkCacheMetaData &metaData) override;
    void insert(QIODevice *device) override;

    QNetworkCacheMetaData fileMetaData(const QString &fileName) const;

public Q_SLOTS:
    void clear() override;

protected:
    virtual qint64 expire();

private:
    Q_DECLARE_PRIVATE(QNetworkDiskCache)
    Q_DISABLE_COPY(QNetworkDiskCache)
};

QT_END_NAMESPACE

#endif // QNETWORKDISKCACHE_H
