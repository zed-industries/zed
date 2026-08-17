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

#ifndef QLIBRARY_H
#define QLIBRARY_H

#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(library);

QT_BEGIN_NAMESPACE

class QLibraryPrivate;

class Q_CORE_EXPORT QLibrary : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString fileName READ fileName WRITE setFileName)
    Q_PROPERTY(LoadHints loadHints READ loadHints WRITE setLoadHints)
public:
    enum LoadHint {
        ResolveAllSymbolsHint = 0x01,
        ExportExternalSymbolsHint = 0x02,
        LoadArchiveMemberHint = 0x04,
        PreventUnloadHint = 0x08,
        DeepBindHint = 0x10
    };
    Q_DECLARE_FLAGS(LoadHints, LoadHint)
    Q_FLAG(LoadHint)
    Q_FLAG(LoadHints)

    explicit QLibrary(QObject *parent = nullptr);
    explicit QLibrary(const QString& fileName, QObject *parent = nullptr);
    explicit QLibrary(const QString& fileName, int verNum, QObject *parent = nullptr);
    explicit QLibrary(const QString& fileName, const QString &version, QObject *parent = nullptr);
    ~QLibrary();

    QFunctionPointer resolve(const char *symbol);
    static QFunctionPointer resolve(const QString &fileName, const char *symbol);
    static QFunctionPointer resolve(const QString &fileName, int verNum, const char *symbol);
    static QFunctionPointer resolve(const QString &fileName, const QString &version, const char *symbol);

    bool load();
    bool unload();
    bool isLoaded() const;

    static bool isLibrary(const QString &fileName);

    void setFileName(const QString &fileName);
    QString fileName() const;

    void setFileNameAndVersion(const QString &fileName, int verNum);
    void setFileNameAndVersion(const QString &fileName, const QString &version);
    QString errorString() const;

    void setLoadHints(LoadHints hints);
    LoadHints loadHints() const;
private:
    QLibraryPrivate *d;
    bool did_load;
    Q_DISABLE_COPY(QLibrary)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QLibrary::LoadHints)

QT_END_NAMESPACE

#endif //QLIBRARY_H
