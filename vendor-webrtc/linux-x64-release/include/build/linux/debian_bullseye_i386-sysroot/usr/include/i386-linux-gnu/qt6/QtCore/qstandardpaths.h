// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTANDARDPATHS_H
#define QSTANDARDPATHS_H

#include <QtCore/qstringlist.h>
#include <QtCore/qobjectdefs.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_STANDARDPATHS

class Q_CORE_EXPORT QStandardPaths
{
    Q_GADGET

public:
    enum StandardLocation {
        DesktopLocation,
        DocumentsLocation,
        FontsLocation,
        ApplicationsLocation,
        MusicLocation,
        MoviesLocation,
        PicturesLocation,
        TempLocation,
        HomeLocation,
        AppLocalDataLocation,
        CacheLocation,
        GenericDataLocation,
        RuntimeLocation,
        ConfigLocation,
        DownloadLocation,
        GenericCacheLocation,
        GenericConfigLocation,
        AppDataLocation,
        AppConfigLocation,
        PublicShareLocation,
        TemplatesLocation
    };
    Q_ENUM(StandardLocation)

    static QString writableLocation(StandardLocation type);
    static QStringList standardLocations(StandardLocation type);

    enum LocateOption {
        LocateFile = 0x0,
        LocateDirectory = 0x1
    };
    Q_DECLARE_FLAGS(LocateOptions, LocateOption)
    Q_FLAG(LocateOptions)

    static QString locate(StandardLocation type, const QString &fileName, LocateOptions options = LocateFile);
    static QStringList locateAll(StandardLocation type, const QString &fileName, LocateOptions options = LocateFile);
#ifndef QT_BOOTSTRAPPED
    static QString displayName(StandardLocation type);
#endif

    static QString findExecutable(const QString &executableName, const QStringList &paths = QStringList());

    static void setTestModeEnabled(bool testMode);
    static bool isTestModeEnabled();

private:
    // prevent construction
    QStandardPaths();
    ~QStandardPaths();
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QStandardPaths::LocateOptions)

#endif // QT_NO_STANDARDPATHS

QT_END_NAMESPACE

#endif // QSTANDARDPATHS_H
