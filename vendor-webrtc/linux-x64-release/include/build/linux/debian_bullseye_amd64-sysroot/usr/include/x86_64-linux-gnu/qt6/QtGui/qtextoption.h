// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTEXTOPTION_H
#define QTEXTOPTION_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qchar.h>
#include <QtCore/qmetatype.h>


QT_BEGIN_NAMESPACE

struct QTextOptionPrivate;

class Q_GUI_EXPORT QTextOption
{
public:
    enum TabType {
        LeftTab,
        RightTab,
        CenterTab,
        DelimiterTab
    };

    struct Q_GUI_EXPORT Tab {
        inline Tab() : position(80), type(QTextOption::LeftTab) { }
        inline Tab(qreal pos, TabType tabType, QChar delim = QChar())
            : position(pos), type(tabType), delimiter(delim) {}

        inline bool operator==(const Tab &other) const {
            return type == other.type
                   && qFuzzyCompare(position, other.position)
                   && delimiter == other.delimiter;
        }

        inline bool operator!=(const Tab &other) const {
            return !operator==(other);
        }

        qreal position;
        TabType type;
        QChar delimiter;
    };

    QTextOption();
    Q_IMPLICIT QTextOption(Qt::Alignment alignment);
    ~QTextOption();

    QTextOption(const QTextOption &o);
    QTextOption &operator=(const QTextOption &o);

    inline void setAlignment(Qt::Alignment alignment);
    inline Qt::Alignment alignment() const { return Qt::Alignment(align); }

    inline void setTextDirection(Qt::LayoutDirection aDirection) { this->direction = aDirection; }
    inline Qt::LayoutDirection textDirection() const { return Qt::LayoutDirection(direction); }

    enum WrapMode {
        NoWrap,
        WordWrap,
        ManualWrap,
        WrapAnywhere,
        WrapAtWordBoundaryOrAnywhere
    };
    inline void setWrapMode(WrapMode wrap) { wordWrap = wrap; }
    inline WrapMode wrapMode() const { return static_cast<WrapMode>(wordWrap); }

    enum Flag {
        ShowTabsAndSpaces = 0x1,
        ShowLineAndParagraphSeparators = 0x2,
        AddSpaceForLineAndParagraphSeparators = 0x4,
        SuppressColors = 0x8,
        ShowDocumentTerminator = 0x10,
        IncludeTrailingSpaces = 0x80000000
    };
    Q_DECLARE_FLAGS(Flags, Flag)
    inline void setFlags(Flags flags);
    inline Flags flags() const { return Flags(f); }

    inline void setTabStopDistance(qreal tabStopDistance);
    inline qreal tabStopDistance() const { return tab; }

    void setTabArray(const QList<qreal> &tabStops);
    QList<qreal> tabArray() const;

    void setTabs(const QList<Tab> &tabStops);
    QList<Tab> tabs() const;

    void setUseDesignMetrics(bool b) { design = b; }
    bool useDesignMetrics() const { return design; }

private:
    uint align : 9;
    uint wordWrap : 4;
    uint design : 1;
    uint direction : 2;
    uint unused : 16;
    uint f;
    qreal tab;
    QTextOptionPrivate *d;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QTextOption::Flags)

inline void QTextOption::setAlignment(Qt::Alignment aalignment)
{ align = uint(aalignment.toInt()); }

inline void QTextOption::setFlags(Flags aflags)
{ f = uint(aflags.toInt()); }

inline void QTextOption::setTabStopDistance(qreal atabStop)
{ tab = atabStop; }

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN_TAGGED(QTextOption::Tab, QTextOption_Tab, Q_GUI_EXPORT)

#endif // QTEXTOPTION_H
