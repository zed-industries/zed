/****************************************************************************
**
** Copyright (C) 2016 Intel Corporation.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QURLQUERY_H
#define QURLQUERY_H

#include <QtCore/qpair.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qurl.h>

#if QT_DEPRECATED_SINCE(5,0)
#include <QtCore/qstringlist.h>
#endif

#include <initializer_list>

QT_BEGIN_NAMESPACE

Q_CORE_EXPORT uint qHash(const QUrlQuery &key, uint seed = 0) noexcept;

class QUrlQueryPrivate;
class Q_CORE_EXPORT QUrlQuery
{
public:
    QUrlQuery();
    explicit QUrlQuery(const QUrl &url);
    explicit QUrlQuery(const QString &queryString);
    QUrlQuery(std::initializer_list<QPair<QString, QString>> list)
        : QUrlQuery()
    {
        for (const QPair<QString, QString> &item : list)
            addQueryItem(item.first, item.second);
    }

    QUrlQuery(const QUrlQuery &other);
    QUrlQuery &operator=(const QUrlQuery &other);
    QUrlQuery &operator=(QUrlQuery &&other) noexcept { swap(other); return *this; }
    ~QUrlQuery();

    bool operator==(const QUrlQuery &other) const;
    bool operator!=(const QUrlQuery &other) const
    { return !(*this == other); }

    void swap(QUrlQuery &other) noexcept { qSwap(d, other.d); }

    bool isEmpty() const;
    bool isDetached() const;
    void clear();

    QString query(QUrl::ComponentFormattingOptions encoding = QUrl::PrettyDecoded) const;
    void setQuery(const QString &queryString);
    QString toString(QUrl::ComponentFormattingOptions encoding = QUrl::PrettyDecoded) const
    { return query(encoding); }

    void setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
    QChar queryValueDelimiter() const;
    QChar queryPairDelimiter() const;

    void setQueryItems(const QList<QPair<QString, QString> > &query);
    QList<QPair<QString, QString> > queryItems(QUrl::ComponentFormattingOptions encoding = QUrl::PrettyDecoded) const;

    bool hasQueryItem(const QString &key) const;
    void addQueryItem(const QString &key, const QString &value);
    void removeQueryItem(const QString &key);
    QString queryItemValue(const QString &key, QUrl::ComponentFormattingOptions encoding = QUrl::PrettyDecoded) const;
    QStringList allQueryItemValues(const QString &key, QUrl::ComponentFormattingOptions encoding = QUrl::PrettyDecoded) const;
    void removeAllQueryItems(const QString &key);

    static QChar defaultQueryValueDelimiter()
    { return QChar(ushort('=')); }
    static QChar defaultQueryPairDelimiter()
    { return QChar(ushort('&')); }

private:
    friend class QUrl;
    friend Q_CORE_EXPORT uint qHash(const QUrlQuery &key, uint seed) noexcept;
    QSharedDataPointer<QUrlQueryPrivate> d;
public:
    typedef QSharedDataPointer<QUrlQueryPrivate> DataPtr;
    inline DataPtr &data_ptr() { return d; }
};

Q_DECLARE_SHARED(QUrlQuery)

#if QT_DEPRECATED_SINCE(5,0)
inline void QUrl::setQueryItems(const QList<QPair<QString, QString> > &qry)
{ QUrlQuery q(*this); q.setQueryItems(qry); setQuery(q); }
inline void QUrl::addQueryItem(const QString &key, const QString &value)
{ QUrlQuery q(*this); q.addQueryItem(key, value); setQuery(q); }
inline QList<QPair<QString, QString> > QUrl::queryItems() const
{ return QUrlQuery(*this).queryItems(); }
inline bool QUrl::hasQueryItem(const QString &key) const
{ return QUrlQuery(*this).hasQueryItem(key); }
inline QString QUrl::queryItemValue(const QString &key) const
{ return QUrlQuery(*this).queryItemValue(key); }
inline QStringList QUrl::allQueryItemValues(const QString &key) const
{ return QUrlQuery(*this).allQueryItemValues(key); }
inline void QUrl::removeQueryItem(const QString &key)
{ QUrlQuery q(*this); q.removeQueryItem(key); setQuery(q); }
inline void QUrl::removeAllQueryItems(const QString &key)
{ QUrlQuery q(*this); q.removeAllQueryItems(key); setQuery(q); }

inline void QUrl::addEncodedQueryItem(const QByteArray &key, const QByteArray &value)
{ QUrlQuery q(*this); q.addQueryItem(fromEncodedComponent_helper(key), fromEncodedComponent_helper(value)); setQuery(q); }
inline bool QUrl::hasEncodedQueryItem(const QByteArray &key) const
{ return QUrlQuery(*this).hasQueryItem(fromEncodedComponent_helper(key)); }
inline QByteArray QUrl::encodedQueryItemValue(const QByteArray &key) const
{ return QUrlQuery(*this).queryItemValue(fromEncodedComponent_helper(key), QUrl::FullyEncoded).toLatin1(); }
inline void QUrl::removeEncodedQueryItem(const QByteArray &key)
{ QUrlQuery q(*this); q.removeQueryItem(fromEncodedComponent_helper(key)); setQuery(q); }
inline void QUrl::removeAllEncodedQueryItems(const QByteArray &key)
{ QUrlQuery q(*this); q.removeAllQueryItems(fromEncodedComponent_helper(key)); setQuery(q); }

inline void QUrl::setEncodedQueryItems(const QList<QPair<QByteArray, QByteArray> > &qry)
{
    QUrlQuery q;
    QList<QPair<QByteArray, QByteArray> >::ConstIterator it = qry.constBegin();
    for ( ; it != qry.constEnd(); ++it)
        q.addQueryItem(fromEncodedComponent_helper(it->first), fromEncodedComponent_helper(it->second));
    setQuery(q);
}
inline QList<QPair<QByteArray, QByteArray> > QUrl::encodedQueryItems() const
{
    QList<QPair<QString, QString> > items = QUrlQuery(*this).queryItems(QUrl::FullyEncoded);
    QList<QPair<QString, QString> >::ConstIterator it = items.constBegin();
    QList<QPair<QByteArray, QByteArray> > result;
    result.reserve(items.size());
    for ( ; it != items.constEnd(); ++it)
        result << qMakePair(it->first.toLatin1(), it->second.toLatin1());
    return result;
}
inline QList<QByteArray> QUrl::allEncodedQueryItemValues(const QByteArray &key) const
{
    const QStringList items = QUrlQuery(*this).allQueryItemValues(fromEncodedComponent_helper(key), QUrl::FullyEncoded);
    QList<QByteArray> result;
    result.reserve(items.size());
    for (const QString &item : items)
        result << item.toLatin1();
    return result;
}
#endif

QT_END_NAMESPACE

#endif // QURLQUERY_H
