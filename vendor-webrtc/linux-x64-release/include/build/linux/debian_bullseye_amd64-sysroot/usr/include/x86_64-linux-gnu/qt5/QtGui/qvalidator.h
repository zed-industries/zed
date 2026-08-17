/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2012 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QVALIDATOR_H
#define QVALIDATOR_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qstring.h>
#include <QtCore/qregexp.h>
#if QT_CONFIG(regularexpression)
#  include <QtCore/qregularexpression.h>
#endif
#include <QtCore/qlocale.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_VALIDATOR

class QValidatorPrivate;

class Q_GUI_EXPORT QValidator : public QObject
{
    Q_OBJECT
public:
    explicit QValidator(QObject * parent = nullptr);
    ~QValidator();

    enum State {
        Invalid,
        Intermediate,
        Acceptable
    };
    Q_ENUM(State)

    void setLocale(const QLocale &locale);
    QLocale locale() const;

    virtual State validate(QString &, int &) const = 0;
    virtual void fixup(QString &) const;

Q_SIGNALS:
    void changed();

protected:
    QValidator(QObjectPrivate &d, QObject *parent);
    QValidator(QValidatorPrivate &d, QObject *parent);

private:
    Q_DISABLE_COPY(QValidator)
    Q_DECLARE_PRIVATE(QValidator)
};

class Q_GUI_EXPORT QIntValidator : public QValidator
{
    Q_OBJECT
    Q_PROPERTY(int bottom READ bottom WRITE setBottom NOTIFY bottomChanged)
    Q_PROPERTY(int top READ top WRITE setTop NOTIFY topChanged)

public:
    explicit QIntValidator(QObject * parent = nullptr);
    QIntValidator(int bottom, int top, QObject *parent = nullptr);
    ~QIntValidator();

    QValidator::State validate(QString &, int &) const override;
    void fixup(QString &input) const override;

    void setBottom(int);
    void setTop(int);
    virtual void setRange(int bottom, int top);

    int bottom() const { return b; }
    int top() const { return t; }
Q_SIGNALS:
    void bottomChanged(int bottom);
    void topChanged(int top);

private:
    Q_DISABLE_COPY(QIntValidator)

    int b;
    int t;
};

#ifndef QT_NO_REGEXP

class QDoubleValidatorPrivate;

class Q_GUI_EXPORT QDoubleValidator : public QValidator
{
    Q_OBJECT
    Q_PROPERTY(double bottom READ bottom WRITE setBottom NOTIFY bottomChanged)
    Q_PROPERTY(double top READ top WRITE setTop NOTIFY topChanged)
    Q_PROPERTY(int decimals READ decimals WRITE setDecimals NOTIFY decimalsChanged)
    Q_PROPERTY(Notation notation READ notation WRITE setNotation NOTIFY notationChanged)

public:
    explicit QDoubleValidator(QObject * parent = nullptr);
    QDoubleValidator(double bottom, double top, int decimals, QObject *parent = nullptr);
    ~QDoubleValidator();

    enum Notation {
        StandardNotation,
        ScientificNotation
    };
    Q_ENUM(Notation)
    QValidator::State validate(QString &, int &) const override;

    virtual void setRange(double bottom, double top, int decimals = 0);
    void setBottom(double);
    void setTop(double);
    void setDecimals(int);
    void setNotation(Notation);

    double bottom() const { return b; }
    double top() const { return t; }
    int decimals() const { return dec; }
    Notation notation() const;

Q_SIGNALS:
    void bottomChanged(double bottom);
    void topChanged(double top);
    void decimalsChanged(int decimals);
    void notationChanged(QDoubleValidator::Notation notation);

private:
    Q_DECLARE_PRIVATE(QDoubleValidator)
    Q_DISABLE_COPY(QDoubleValidator)

    double b;
    double t;
    int dec;
};


class Q_GUI_EXPORT QRegExpValidator : public QValidator
{
    Q_OBJECT
    Q_PROPERTY(QRegExp regExp READ regExp WRITE setRegExp NOTIFY regExpChanged)

public:
    explicit QRegExpValidator(QObject *parent = nullptr);
    explicit QRegExpValidator(const QRegExp& rx, QObject *parent = nullptr);
    ~QRegExpValidator();

    virtual QValidator::State validate(QString& input, int& pos) const override;

    void setRegExp(const QRegExp& rx);
    const QRegExp& regExp() const { return r; }

Q_SIGNALS:
    void regExpChanged(const QRegExp& regExp);

private:
    Q_DISABLE_COPY(QRegExpValidator)

    QRegExp r;
};

#endif // QT_NO_REGEXP

#if QT_CONFIG(regularexpression)

class QRegularExpressionValidatorPrivate;

class Q_GUI_EXPORT QRegularExpressionValidator : public QValidator
{
    Q_OBJECT
    Q_PROPERTY(QRegularExpression regularExpression READ regularExpression WRITE setRegularExpression NOTIFY regularExpressionChanged)

public:
    explicit QRegularExpressionValidator(QObject *parent = nullptr);
    explicit QRegularExpressionValidator(const QRegularExpression &re, QObject *parent = nullptr);
    ~QRegularExpressionValidator();

    virtual QValidator::State validate(QString &input, int &pos) const override;

    QRegularExpression regularExpression() const;

public Q_SLOTS:
    void setRegularExpression(const QRegularExpression &re);

Q_SIGNALS:
    void regularExpressionChanged(const QRegularExpression &re);

private:
    Q_DISABLE_COPY(QRegularExpressionValidator)
    Q_DECLARE_PRIVATE(QRegularExpressionValidator)
};

#endif // QT_CONFIG(regularexpression)

#endif // QT_NO_VALIDATOR

QT_END_NAMESPACE

#endif // QVALIDATOR_H
