// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFONTDATABASE_H
#define QFONTDATABASE_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qstring.h>
#include <QtGui/qfont.h>

QT_BEGIN_NAMESPACE


struct QFontDef;
class QFontEngine;

class Q_GUI_EXPORT QFontDatabase
{
    Q_GADGET
public:
    enum WritingSystem {
        Any,

        Latin,
        Greek,
        Cyrillic,
        Armenian,
        Hebrew,
        Arabic,
        Syriac,
        Thaana,
        Devanagari,
        Bengali,
        Gurmukhi,
        Gujarati,
        Oriya,
        Tamil,
        Telugu,
        Kannada,
        Malayalam,
        Sinhala,
        Thai,
        Lao,
        Tibetan,
        Myanmar,
        Georgian,
        Khmer,
        SimplifiedChinese,
        TraditionalChinese,
        Japanese,
        Korean,
        Vietnamese,

        Symbol,
        Other = Symbol,

        Ogham,
        Runic,
        Nko,

        WritingSystemsCount
    };
    Q_ENUM(WritingSystem)

    enum SystemFont {
        GeneralFont,
        FixedFont,
        TitleFont,
        SmallestReadableFont
    };
    Q_ENUM(SystemFont)

    static QList<int> standardSizes();

#if QT_DEPRECATED_SINCE(6, 0) && !defined(QT_BUILD_GUI_LIB)
    QT_DEPRECATED_VERSION_X_6_0("Call the static functions instead") explicit QFontDatabase() = default;
#else
    QFontDatabase() = delete;
#endif

    static QList<WritingSystem> writingSystems();
    static QList<WritingSystem> writingSystems(const QString &family);

    static QStringList families(WritingSystem writingSystem = Any);
    static QStringList styles(const QString &family);
    static QList<int> pointSizes(const QString &family, const QString &style = QString());
    static QList<int> smoothSizes(const QString &family, const QString &style);
    static QString styleString(const QFont &font);
    static QString styleString(const QFontInfo &fontInfo);

    static QFont font(const QString &family, const QString &style, int pointSize);

    static bool isBitmapScalable(const QString &family, const QString &style = QString());
    static bool isSmoothlyScalable(const QString &family, const QString &style = QString());
    static bool isScalable(const QString &family, const QString &style = QString());
    static bool isFixedPitch(const QString &family, const QString &style = QString());

    static bool italic(const QString &family, const QString &style);
    static bool bold(const QString &family, const QString &style);
    static int weight(const QString &family, const QString &style);

    static bool hasFamily(const QString &family);
    static bool isPrivateFamily(const QString &family);

    static QString writingSystemName(WritingSystem writingSystem);
    static QString writingSystemSample(WritingSystem writingSystem);

    static int addApplicationFont(const QString &fileName);
    static int addApplicationFontFromData(const QByteArray &fontData);
    static QStringList applicationFontFamilies(int id);
    static bool removeApplicationFont(int id);
    static bool removeAllApplicationFonts();

    static QFont systemFont(SystemFont type);
};

QT_END_NAMESPACE

#endif // QFONTDATABASE_H
