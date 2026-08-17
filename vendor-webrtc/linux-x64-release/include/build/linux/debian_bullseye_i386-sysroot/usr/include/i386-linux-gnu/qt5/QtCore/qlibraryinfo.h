/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QLIBRARYINFO_H
#define QLIBRARYINFO_H

#include <QtCore/qstring.h>
#include <QtCore/qdatetime.h>
#include <QtCore/qversionnumber.h>

QT_BEGIN_NAMESPACE

class QStringList;

class Q_CORE_EXPORT QLibraryInfo
{
public:
#if QT_DEPRECATED_SINCE(5, 8)
    static QT_DEPRECATED QString licensee();
    static QT_DEPRECATED QString licensedProducts();
#endif

#if QT_CONFIG(datestring)
#if QT_DEPRECATED_SINCE(5, 5)
    static QT_DEPRECATED QDate buildDate();
#endif // QT_DEPRECATED_SINCE(5, 5)
#endif // datestring

    static const char * build() noexcept;

    static bool isDebugBuild();

#ifndef QT_BOOTSTRAPPED
    static QVersionNumber version() noexcept Q_DECL_CONST_FUNCTION;
#endif

    enum LibraryLocation
    {
        PrefixPath = 0,
        DocumentationPath,
        HeadersPath,
        LibrariesPath,
        LibraryExecutablesPath,
        BinariesPath,
        PluginsPath,
        ImportsPath,
        Qml2ImportsPath,
        ArchDataPath,
        DataPath,
        TranslationsPath,
        ExamplesPath,
        TestsPath,
        // Insert new values above this line
        // Please read the comments in qlibraryinfo.cpp before adding
#ifdef QT_BUILD_QMAKE
        // These are not subject to binary compatibility constraints
        SysrootPath,
        SysrootifyPrefixPath,
        HostBinariesPath,
        HostLibrariesPath,
        HostDataPath,
        TargetSpecPath,
        HostSpecPath,
        HostPrefixPath,
        LastHostPath = HostPrefixPath,
#endif
        SettingsPath = 100
    };
    static QString location(LibraryLocation); // ### Qt 6: consider renaming it to path()
#ifdef QT_BUILD_QMAKE
    enum PathGroup { FinalPaths, EffectivePaths, EffectiveSourcePaths, DevicePaths };
    static QString rawLocation(LibraryLocation, PathGroup);
    static void reload();
    static void sysrootify(QString *path);
#endif

    static QStringList platformPluginArguments(const QString &platformName);

private:
    QLibraryInfo();
};

QT_END_NAMESPACE

#endif // QLIBRARYINFO_H
