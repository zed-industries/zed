// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSQLRESULT_H
#define QSQLRESULT_H

#include <QtSql/qtsqlglobal.h>
#include <QtCore/qvariant.h>
#include <QtCore/qcontainerfwd.h>

// for testing:
class tst_QSqlQuery;

QT_BEGIN_NAMESPACE


class QString;
class QSqlRecord;
class QVariant;
class QSqlDriver;
class QSqlError;
class QSqlResultPrivate;

class Q_SQL_EXPORT QSqlResult
{
    Q_DECLARE_PRIVATE(QSqlResult)
    friend class QSqlQuery;
    friend class QSqlTableModelPrivate;
    // for testing:
    friend class ::tst_QSqlQuery;

public:
    virtual ~QSqlResult();
    virtual QVariant handle() const;

protected:
    enum BindingSyntax {
        PositionalBinding,
        NamedBinding
    };

    explicit QSqlResult(const QSqlDriver * db);
    QSqlResult(QSqlResultPrivate &dd);
    int at() const;
    QString lastQuery() const;
    QSqlError lastError() const;
    bool isValid() const;
    bool isActive() const;
    bool isSelect() const;
    bool isForwardOnly() const;
    const QSqlDriver* driver() const;
    virtual void setAt(int at);
    virtual void setActive(bool a);
    virtual void setLastError(const QSqlError& e);
    virtual void setQuery(const QString& query);
    virtual void setSelect(bool s);
    virtual void setForwardOnly(bool forward);

    // prepared query support
    virtual bool exec();
    virtual bool prepare(const QString& query);
    virtual bool savePrepare(const QString& sqlquery);
    virtual void bindValue(int pos, const QVariant& val, QSql::ParamType type);
    virtual void bindValue(const QString& placeholder, const QVariant& val,
                           QSql::ParamType type);
    void addBindValue(const QVariant& val, QSql::ParamType type);
    QVariant boundValue(const QString& placeholder) const;
    QVariant boundValue(int pos) const;
    QSql::ParamType bindValueType(const QString& placeholder) const;
    QSql::ParamType bindValueType(int pos) const;
    int boundValueCount() const;
    QList<QVariant> &boundValues() const;
    QString executedQuery() const;
    QString boundValueName(int pos) const;
    void clear();
    bool hasOutValues() const;

    BindingSyntax bindingSyntax() const;

    virtual QVariant data(int i) = 0;
    virtual bool isNull(int i) = 0;
    virtual bool reset(const QString& sqlquery) = 0;
    virtual bool fetch(int i) = 0;
    virtual bool fetchNext();
    virtual bool fetchPrevious();
    virtual bool fetchFirst() = 0;
    virtual bool fetchLast() = 0;
    virtual int size() = 0;
    virtual int numRowsAffected() = 0;
    virtual QSqlRecord record() const;
    virtual QVariant lastInsertId() const;

    enum VirtualHookOperation { };
    virtual void virtual_hook(int id, void *data);
    virtual bool execBatch(bool arrayBind = false);
    virtual void detachFromResultSet();
    virtual void setNumericalPrecisionPolicy(QSql::NumericalPrecisionPolicy policy);
    QSql::NumericalPrecisionPolicy numericalPrecisionPolicy() const;
    virtual bool nextResult();
    void resetBindCount(); // HACK

    QSqlResultPrivate *d_ptr;

private:
    Q_DISABLE_COPY(QSqlResult)
};

QT_END_NAMESPACE

#endif // QSQLRESULT_H
