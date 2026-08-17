// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef QTEXTLAYOUT_H
#define QTEXTLAYOUT_H

#include <QtGui/qcolor.h>
#include <QtGui/qevent.h>
#include <QtGui/qglyphrun.h>
#include <QtGui/qtextcursor.h>
#include <QtGui/qtextformat.h>
#include <QtGui/qtguiglobal.h>

#include <QtCore/qlist.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qobject.h>
#include <QtCore/qrect.h>
#include <QtCore/qstring.h>

QT_BEGIN_NAMESPACE


class QTextEngine;
class QFont;
#ifndef QT_NO_RAWFONT
class QRawFont;
#endif
class QRect;
class QRegion;
class QTextFormat;
class QPalette;
class QPainter;

class Q_GUI_EXPORT QTextInlineObject
{
public:
    QTextInlineObject(int i, QTextEngine *e) : itm(i), eng(e) {}
    inline QTextInlineObject() : itm(0), eng(nullptr) {}
    inline bool isValid() const { return eng; }

    QRectF rect() const;
    qreal width() const;
    qreal ascent() const;
    qreal descent() const;
    qreal height() const;

    Qt::LayoutDirection textDirection() const;

    void setWidth(qreal w);
    void setAscent(qreal a);
    void setDescent(qreal d);

    int textPosition() const;

    int formatIndex() const;
    QTextFormat format() const;

private:
    friend class QTextLayout;
    int itm;
    QTextEngine *eng;
};

class QPaintDevice;
class QTextFormat;
class QTextLine;
class QTextBlock;
class QTextOption;

class Q_GUI_EXPORT QTextLayout
{
public:
    // does itemization
    QTextLayout();
    QTextLayout(const QString& text);
    QTextLayout(const QString &text, const QFont &font, const QPaintDevice *paintdevice = nullptr);
    QTextLayout(const QTextBlock &b);
    ~QTextLayout();

    void setFont(const QFont &f);
    QFont font() const;

#ifndef QT_NO_RAWFONT
    void setRawFont(const QRawFont &rawFont);
#endif

    void setText(const QString& string);
    QString text() const;

    void setTextOption(const QTextOption &option);
    const QTextOption &textOption() const;

    void setPreeditArea(int position, const QString &text);
    int preeditAreaPosition() const;
    QString preeditAreaText() const;

    struct FormatRange {
        int start;
        int length;
        QTextCharFormat format;

        friend bool operator==(const FormatRange &lhs, const FormatRange &rhs)
        { return lhs.start == rhs.start && lhs.length == rhs.length && lhs.format == rhs.format; }
        friend bool operator!=(const FormatRange &lhs, const FormatRange &rhs)
        { return !operator==(lhs, rhs); }
    };
    void setFormats(const QList<FormatRange> &overrides);
    QList<FormatRange> formats() const;
    void clearFormats();

    void setCacheEnabled(bool enable);
    bool cacheEnabled() const;

    void setCursorMoveStyle(Qt::CursorMoveStyle style);
    Qt::CursorMoveStyle cursorMoveStyle() const;

    void beginLayout();
    void endLayout();
    void clearLayout();

    QTextLine createLine();

    int lineCount() const;
    QTextLine lineAt(int i) const;
    QTextLine lineForTextPosition(int pos) const;

    enum CursorMode {
        SkipCharacters,
        SkipWords
    };
    bool isValidCursorPosition(int pos) const;
    int nextCursorPosition(int oldPos, CursorMode mode = SkipCharacters) const;
    int previousCursorPosition(int oldPos, CursorMode mode = SkipCharacters) const;
    int leftCursorPosition(int oldPos) const;
    int rightCursorPosition(int oldPos) const;

    void draw(QPainter *p, const QPointF &pos,
              const QList<FormatRange> &selections = QList<FormatRange>(),
              const QRectF &clip = QRectF()) const;
    void drawCursor(QPainter *p, const QPointF &pos, int cursorPosition) const;
    void drawCursor(QPainter *p, const QPointF &pos, int cursorPosition, int width) const;

    QPointF position() const;
    void setPosition(const QPointF &p);

    QRectF boundingRect() const;

    qreal minimumWidth() const;
    qreal maximumWidth() const;

#if !defined(QT_NO_RAWFONT)
    QList<QGlyphRun> glyphRuns(int from = -1, int length = -1) const;
#endif

    QTextEngine *engine() const { return d; }
    void setFlags(int flags);
private:
    QTextLayout(QTextEngine *e) : d(e) {}
    Q_DISABLE_COPY(QTextLayout)

    friend class QPainter;
    friend class QGraphicsSimpleTextItemPrivate;
    friend class QGraphicsSimpleTextItem;
    friend void qt_format_text(const QFont &font, const QRectF &_r, int tf, const QTextOption *, const QString& str,
                               QRectF *brect, int tabstops, int* tabarray, int tabarraylen,
                               QPainter *painter);
    QTextEngine *d;
};
Q_DECLARE_TYPEINFO(QTextLayout::FormatRange, Q_RELOCATABLE_TYPE);


class Q_GUI_EXPORT QTextLine
{
public:
    inline QTextLine() : index(0), eng(nullptr) {}
    inline bool isValid() const { return eng; }

    QRectF rect() const;
    qreal x() const;
    qreal y() const;
    qreal width() const;
    qreal ascent() const;
    qreal descent() const;
    qreal height() const;
    qreal leading() const;

    void setLeadingIncluded(bool included);
    bool leadingIncluded() const;

    qreal naturalTextWidth() const;
    qreal horizontalAdvance() const;
    QRectF naturalTextRect() const;

    enum Edge {
        Leading,
        Trailing
    };
    enum CursorPosition {
        CursorBetweenCharacters,
        CursorOnCharacter
    };

    /* cursorPos gets set to the valid position */
    qreal cursorToX(int *cursorPos, Edge edge = Leading) const;
    inline qreal cursorToX(int cursorPos, Edge edge = Leading) const { return cursorToX(&cursorPos, edge); }
    int xToCursor(qreal x, CursorPosition = CursorBetweenCharacters) const;

    void setLineWidth(qreal width);
    void setNumColumns(int columns);
    void setNumColumns(int columns, qreal alignmentWidth);

    void setPosition(const QPointF &pos);
    QPointF position() const;

    int textStart() const;
    int textLength() const;

    int lineNumber() const { return index; }

    void draw(QPainter *painter, const QPointF &position) const;

#if !defined(QT_NO_RAWFONT)
    QList<QGlyphRun> glyphRuns(int from = -1, int length = -1) const;
#endif

private:
    QTextLine(int line, QTextEngine *e) : index(line), eng(e) {}
    void layout_helper(int numGlyphs);
    void draw_internal(QPainter *p, const QPointF &pos,
                       const QTextLayout::FormatRange *selection) const;

    friend class QTextLayout;
    friend class QTextFragment;
    int index;
    QTextEngine *eng;
};

QT_END_NAMESPACE

#endif // QTEXTLAYOUT_H
