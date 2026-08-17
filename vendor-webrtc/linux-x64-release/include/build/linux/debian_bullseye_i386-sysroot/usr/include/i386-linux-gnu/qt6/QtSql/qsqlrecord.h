// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSQLRECORD_H
#define QSQLRECORD_H

#include <QtSql/qtsqlglobal.h>
#include <QtCore/qstring.h>

QT_BEGIN_NAMESPACE


class QSqlField;
class QVariant;
class QSqlRecordPrivate;

class Q_SQL_EXPORT QSqlRecord
{
public:
    QSqlRecord();
    QSqlRecord(const QSqlRecord& other);
    QSqlRecord& operator=(const QSqlRecord& other);
    ~QSqlRecord();

    bool operator==(const QSqlRecord &other) const;
    inline bool operator!=(const QSqlRecord &other) const { return !operator==(other); }

    QVariant value(int i) const;
    QVariant value(const QString& name) const;
    void setValue(int i, const QVariant& val);
    void setValue(const QString& name, const QVariant& val);

    void setNull(int i);
    void setNull(const QString& name);
    bool isNull(int i) const;
    bool isNull(const QString& name) const;

    int indexOf(const QString &name) const;
    QString fieldName(int i) const;

    QSqlField field(int i) const;
    QSqlField field(const QString &name) const;

    bool isGenerated(int i) const;
    bool isGenerated(const QString& name) const;
    void setGenerated(const QString& name, bool generated);
    void setGenerated(int i, bool generated);

    void append(const QSqlField& field);
    void replace(int pos, const QSqlField& field);
    void insert(int pos, const QSqlField& field);
    void remove(int pos);

    bool isEmpty() const;
    bool contains(const QString& name) const;
    void clear();
    void clearValues();
    int count() const;
    QSqlRecord keyValues(const QSqlRecord &keyFields) const;

private:
    void detach();
    QSqlRecordPrivate* d;
};

#ifndef QT_NO_DEBUG_STREAM
Q_SQL_EXPORT QDebug operator<<(QDebug, const QSqlRecord &);
#endif

QT_END_NAMESPACE

#endif // QSQLRECORD_H
