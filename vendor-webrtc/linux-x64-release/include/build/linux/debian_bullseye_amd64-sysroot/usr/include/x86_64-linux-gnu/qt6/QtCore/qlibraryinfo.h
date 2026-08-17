// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QLIBRARYINFO_H
#define QLIBRARYINFO_H

#include <QtCore/qstring.h>
#include <QtCore/qdatetime.h>
#include <QtCore/qversionnumber.h>

QT_BEGIN_NAMESPACE

class Q_CORE_EXPORT QLibraryInfo
{
public:
    static const char *build() noexcept;

    static bool isDebugBuild() noexcept Q_DECL_CONST_FUNCTION;

#ifndef QT_BOOTSTRAPPED
    static QVersionNumber version() noexcept Q_DECL_CONST_FUNCTION;
#endif

    enum LibraryPath {
        PrefixPath = 0,
        DocumentationPath,
        HeadersPath,
        LibrariesPath,
        LibraryExecutablesPath,
        BinariesPath,
        PluginsPath,
        QmlImportsPath,
        Qml2ImportsPath = QmlImportsPath,
        ArchDataPath,
        DataPath,
        TranslationsPath,
        ExamplesPath,
        TestsPath,
        // Insert new values above this line
        // Please read the comments in qconfig.cpp.in before adding
        SettingsPath = 100
    };
    static QString path(LibraryPath p);
#if QT_DEPRECATED_SINCE(6, 0)
    using LibraryLocation = LibraryPath;
    QT_DEPRECATED_VERSION_X_6_0("Use path()")
    static QString location(LibraryLocation location)
    { return path(location); }
#endif
    static QStringList platformPluginArguments(const QString &platformName);

private:
    QLibraryInfo();
};

QT_END_NAMESPACE

#endif // QLIBRARYINFO_H
