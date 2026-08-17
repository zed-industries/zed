// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSQLQUERY_H
#define QSQLQUERY_H

#include <QtSql/qtsqlglobal.h>
#include <QtSql/qsqldatabase.h>
#include <QtCore/qstring.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE


class QSqlDriver;
class QSqlError;
class QSqlResult;
class QSqlRecord;
class QSqlQueryPrivate;


class Q_SQL_EXPORT QSqlQuery
{
public:
    explicit QSqlQuery(QSqlResult *r);
    explicit QSqlQuery(const QString& query = QString(), const QSqlDatabase &db = QSqlDatabase());
    explicit QSqlQuery(const QSqlDatabase &db);

#if QT_DEPRECATED_SINCE(6, 2)
    QT_DEPRECATED_VERSION_X_6_2("QSqlQuery is not meant to be copied. Use move construction instead.")
    QSqlQuery(const QSqlQuery &other);
    QT_DEPRECATED_VERSION_X_6_2("QSqlQuery is not meant to be copied. Use move assignment instead.")
    QSqlQuery& operator=(const QSqlQuery &other);
#else
    QSqlQuery(const QSqlQuery &other) = delete;
    QSqlQuery& operator=(const QSqlQuery &other) = delete;
#endif

    QSqlQuery(QSqlQuery &&other) noexcept
        : d(std::exchange(other.d, nullptr))
    {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QSqlQuery)

    ~QSqlQuery();

    void swap(QSqlQuery &other) noexcept
    { qt_ptr_swap(d, other.d); }

    bool isValid() const;
    bool isActive() const;
    bool isNull(int field) const;
    bool isNull(const QString &name) const;
    int at() const;
    QString lastQuery() const;
    int numRowsAffected() const;
    QSqlError lastError() const;
    bool isSelect() const;
    int size() const;
    const QSqlDriver* driver() const;
    const QSqlResult* result() const;
    bool isForwardOnly() const;
    QSqlRecord record() const;

    void setForwardOnly(bool forward);
    bool exec(const QString& query);
    QVariant value(int i) const;
    QVariant value(const QString& name) const;

    void setNumericalPrecisionPolicy(QSql::NumericalPrecisionPolicy precisionPolicy);
    QSql::NumericalPrecisionPolicy numericalPrecisionPolicy() const;

    bool seek(int i, bool relative = false);
    bool next();
    bool previous();
    bool first();
    bool last();

    void clear();

    // prepared query support
    bool exec();
    enum BatchExecutionMode { ValuesAsRows, ValuesAsColumns };
    bool execBatch(BatchExecutionMode mode = ValuesAsRows);
    bool prepare(const QString& query);
    void bindValue(const QString& placeholder, const QVariant& val,
                   QSql::ParamType type = QSql::In);
    void bindValue(int pos, const QVariant& val, QSql::ParamType type = QSql::In);
    void addBindValue(const QVariant& val, QSql::ParamType type = QSql::In);
    QVariant boundValue(const QString& placeholder) const;
    QVariant boundValue(int pos) const;
    QVariantList boundValues() const;
    QString executedQuery() const;
    QVariant lastInsertId() const;
    void finish();
    bool nextResult();

private:
    QSqlQueryPrivate* d;
};

QT_END_NAMESPACE

#endif // QSQLQUERY_H
