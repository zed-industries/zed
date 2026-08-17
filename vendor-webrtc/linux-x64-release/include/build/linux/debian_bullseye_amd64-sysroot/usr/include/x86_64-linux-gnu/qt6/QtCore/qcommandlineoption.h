// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2013 Laszlo Papp <lpapp@kde.org>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCOMMANDLINEOPTION_H
#define QCOMMANDLINEOPTION_H

#include <QtCore/qstringlist.h>
#include <QtCore/qshareddata.h>

QT_REQUIRE_CONFIG(commandlineparser);

QT_BEGIN_NAMESPACE

class QCommandLineOptionPrivate;

class Q_CORE_EXPORT QCommandLineOption
{
public:
    enum Flag {
        HiddenFromHelp = 0x1,
        ShortOptionStyle = 0x2
    };
    Q_DECLARE_FLAGS(Flags, Flag)

    explicit QCommandLineOption(const QString &name);
    explicit QCommandLineOption(const QStringList &names);
    /*implicit*/ QCommandLineOption(const QString &name, const QString &description,
                                const QString &valueName = QString(),
                                const QString &defaultValue = QString());
    /*implicit*/ QCommandLineOption(const QStringList &names, const QString &description,
                                const QString &valueName = QString(),
                                const QString &defaultValue = QString());
    QCommandLineOption(const QCommandLineOption &other);

    ~QCommandLineOption();

    QCommandLineOption &operator=(const QCommandLineOption &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QCommandLineOption)

    void swap(QCommandLineOption &other) noexcept
    { d.swap(other.d); }

    QStringList names() const;

    void setValueName(const QString &name);
    QString valueName() const;

    void setDescription(const QString &description);
    QString description() const;

    void setDefaultValue(const QString &defaultValue);
    void setDefaultValues(const QStringList &defaultValues);
    QStringList defaultValues() const;

    Flags flags() const;
    void setFlags(Flags aflags);

private:
    QSharedDataPointer<QCommandLineOptionPrivate> d;
};

Q_DECLARE_SHARED(QCommandLineOption)
Q_DECLARE_OPERATORS_FOR_FLAGS(QCommandLineOption::Flags)


QT_END_NAMESPACE

#endif // QCOMMANDLINEOPTION_H
