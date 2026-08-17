// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2012 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QVALIDATOR_H
#define QVALIDATOR_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qstring.h>
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
    void setRange(int bottom, int top);

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
    void fixup(QString &input) const override;

    void setRange(double bottom, double top, int decimals);
    void setRange(double bottom, double top);
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

    QValidator::State validate(QString &input, int &pos) const override;

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
