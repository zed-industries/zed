// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTEXTDOCUMENTFRAGMENT_H
#define QTEXTDOCUMENTFRAGMENT_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qtextdocument.h>
#include <QtCore/qstring.h>

QT_BEGIN_NAMESPACE


class QTextStream;
class QTextDocumentFragmentPrivate;
class QTextCursor;

class Q_GUI_EXPORT QTextDocumentFragment
{
public:
    QTextDocumentFragment();
    explicit QTextDocumentFragment(const QTextDocument *document);
    explicit QTextDocumentFragment(const QTextCursor &range);
    QTextDocumentFragment(const QTextDocumentFragment &rhs);
    QTextDocumentFragment &operator=(const QTextDocumentFragment &rhs);
    ~QTextDocumentFragment();

    bool isEmpty() const;

    QString toPlainText() const;
    QString toRawText() const;
#ifndef QT_NO_TEXTHTMLPARSER
    QString toHtml() const;
#endif // QT_NO_TEXTHTMLPARSER
#if QT_CONFIG(textmarkdownwriter)
    QString toMarkdown(QTextDocument::MarkdownFeatures features = QTextDocument::MarkdownDialectGitHub) const;
#endif

    static QTextDocumentFragment fromPlainText(const QString &plainText);
#ifndef QT_NO_TEXTHTMLPARSER
    static QTextDocumentFragment fromHtml(const QString &html, const QTextDocument *resourceProvider = nullptr);
#endif // QT_NO_TEXTHTMLPARSER
#if QT_CONFIG(textmarkdownreader)
    static QTextDocumentFragment fromMarkdown(const QString &markdown,
                                              QTextDocument::MarkdownFeatures features = QTextDocument::MarkdownDialectGitHub);
#endif

private:
    QTextDocumentFragmentPrivate *d;
    friend class QTextCursor;
    friend class QTextDocumentWriter;
};

QT_END_NAMESPACE

#endif // QTEXTDOCUMENTFRAGMENT_H
