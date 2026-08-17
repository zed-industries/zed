// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPLUGINLOADER_H
#define QPLUGINLOADER_H

#include <QtCore/qglobal.h>
#if QT_CONFIG(library)
#include <QtCore/qlibrary.h>
#endif
#include <QtCore/qplugin.h>

QT_BEGIN_NAMESPACE

#if QT_CONFIG(library)

class QLibraryPrivate;
class QJsonObject;

class Q_CORE_EXPORT QPluginLoader : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString fileName READ fileName WRITE setFileName)
    Q_PROPERTY(QLibrary::LoadHints loadHints READ loadHints WRITE setLoadHints)
public:
    explicit QPluginLoader(QObject *parent = nullptr);
    explicit QPluginLoader(const QString &fileName, QObject *parent = nullptr);
    ~QPluginLoader();

    QObject *instance();
    QJsonObject metaData() const;

    static QObjectList staticInstances();
    static QList<QStaticPlugin> staticPlugins();

    bool load();
    bool unload();
    bool isLoaded() const;

    void setFileName(const QString &fileName);
    QString fileName() const;

    QString errorString() const;

    void setLoadHints(QLibrary::LoadHints loadHints);
    QLibrary::LoadHints loadHints() const;

private:
    QLibraryPrivate *d;
    bool did_load;
    Q_DISABLE_COPY(QPluginLoader)
};

#else

class Q_CORE_EXPORT QPluginLoader
{
public:
    static QObjectList staticInstances();
    static QList<QStaticPlugin> staticPlugins();
};

#endif // QT_CONFIG(library)

QT_END_NAMESPACE

#endif // QPLUGINLOADER_H
