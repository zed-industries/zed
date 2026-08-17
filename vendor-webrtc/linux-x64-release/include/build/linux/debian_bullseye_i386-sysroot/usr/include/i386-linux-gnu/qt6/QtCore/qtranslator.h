// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTRANSLATOR_H
#define QTRANSLATOR_H

#include <QtCore/qobject.h>
#include <QtCore/qbytearray.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_TRANSLATION

class QLocale;
class QTranslatorPrivate;

class Q_CORE_EXPORT QTranslator : public QObject
{
    Q_OBJECT
public:
    explicit QTranslator(QObject *parent = nullptr);
    ~QTranslator();

    virtual QString translate(const char *context, const char *sourceText,
                              const char *disambiguation = nullptr, int n = -1) const;

    virtual bool isEmpty() const;

    QString language() const;
    QString filePath() const;

    [[nodiscard]] bool load(const QString & filename,
                            const QString & directory = QString(),
                            const QString & search_delimiters = QString(),
                            const QString & suffix = QString());
    [[nodiscard]] bool load(const QLocale & locale,
                            const QString & filename,
                            const QString & prefix = QString(),
                            const QString & directory = QString(),
                            const QString & suffix = QString());
    [[nodiscard]] bool load(const uchar *data, int len,
                            const QString &directory = QString());

private:
    Q_DISABLE_COPY(QTranslator)
    Q_DECLARE_PRIVATE(QTranslator)
};

#endif // QT_NO_TRANSLATION

QT_END_NAMESPACE

#endif // QTRANSLATOR_H
