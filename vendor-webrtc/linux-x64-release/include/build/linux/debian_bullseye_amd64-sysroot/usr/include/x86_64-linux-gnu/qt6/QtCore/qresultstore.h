// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCORE_RESULTSTORE_H
#define QTCORE_RESULTSTORE_H

#include <QtCore/qmap.h>

#include <utility>

QT_REQUIRE_CONFIG(future);

QT_BEGIN_NAMESPACE

/*
    ResultStore stores indexed results. Results can be added and retrieved
    either individually batched in a QList. Retriveing results and checking
    which indexes are in the store can be done either by iterating or by random
    access. In addition results can be removed from the front of the store,
    either individually or in batches.
*/

namespace QtPrivate {

class ResultItem
{
public:
    ResultItem(const void *_result, int _count) : m_count(_count), result(_result) { } // construct with vector of results
    ResultItem(const void *_result) : m_count(0), result(_result) { } // construct with result
    ResultItem() : m_count(0), result(nullptr) { }
    bool isValid() const { return result != nullptr; }
    bool isVector() const { return m_count != 0; }
    int count() const { return (m_count == 0) ?  1 : m_count; }
    int m_count;          // result is either a pointer to a result or to a vector of results,
    const void *result; // if count is 0 it's a result, otherwise it's a vector.
};

class Q_CORE_EXPORT ResultIteratorBase
{
public:
    ResultIteratorBase();
    ResultIteratorBase(QMap<int, ResultItem>::const_iterator _mapIterator, int _vectorIndex = 0);
    int vectorIndex() const;
    int resultIndex() const;

    ResultIteratorBase operator++();
    int batchSize() const;
    void batchedAdvance();
    bool operator==(const ResultIteratorBase &other) const;
    bool operator!=(const ResultIteratorBase &other) const;
    bool isVector() const;
    bool canIncrementVectorIndex() const;
    bool isValid() const;

protected:
    QMap<int, ResultItem>::const_iterator mapIterator;
    int m_vectorIndex;
public:
    template <typename T>
    const T &value() const
    {
        return *pointer<T>();
    }

    template<typename T>
    T &value()
    {
        return *pointer<T>();
    }

    template <typename T>
    T *pointer()
    {
        const T *p = std::as_const(*this).pointer<T>();
        return const_cast<T *>(p);
    }

    template <typename T>
    const T *pointer() const
    {
        if (mapIterator.value().isVector())
            return &(reinterpret_cast<const QList<T> *>(mapIterator.value().result)->at(m_vectorIndex));
        else
            return reinterpret_cast<const T *>(mapIterator.value().result);
    }
};

class Q_CORE_EXPORT ResultStoreBase final
{
public:
    ResultStoreBase();
    void setFilterMode(bool enable);
    bool filterMode() const;
    int addResult(int index, const void *result);
    int addResults(int index, const void *results, int vectorSize, int logicalCount);
    ResultIteratorBase begin() const;
    ResultIteratorBase end() const;
    bool hasNextResult() const;
    ResultIteratorBase resultAt(int index) const;
    bool contains(int index) const;
    int count() const;
    // ### Qt 7: 'virtual' isn't required, can be removed, along with renaming
    // the class to ResultStore and changing the members below to be private.
    virtual ~ResultStoreBase();

protected:
    int insertResultItem(int index, ResultItem &resultItem);
    void insertResultItemIfValid(int index, ResultItem &resultItem);
    bool containsValidResultItem(int index) const;
    void syncPendingResults();
    void syncResultCount();
    int updateInsertIndex(int index, int _count);

    QMap<int, ResultItem> m_results;
    int insertIndex;     // The index where the next results(s) will be inserted.
    int resultCount;     // The number of consecutive results stored, starting at index 0.

    bool m_filterMode;
    QMap<int, ResultItem> pendingResults;
    int filteredResults;

    template <typename T>
    static void clear(QMap<int, ResultItem> &store)
    {
        QMap<int, ResultItem>::const_iterator mapIterator = store.constBegin();
        while (mapIterator != store.constEnd()) {
            if (mapIterator.value().isVector())
                delete reinterpret_cast<const QList<T> *>(mapIterator.value().result);
            else
                delete reinterpret_cast<const T *>(mapIterator.value().result);
            ++mapIterator;
        }
        store.clear();
    }

public:
    template <typename T>
    int addResult(int index, const T *result)
    {
        if (containsValidResultItem(index)) // reject if already present
            return -1;

        if (result == nullptr)
            return addResult(index, static_cast<void *>(nullptr));

        return addResult(index, static_cast<void *>(new T(*result)));
    }

    template <typename T>
    int moveResult(int index, T &&result)
    {
        if (containsValidResultItem(index)) // reject if already present
            return -1;

        return addResult(index, static_cast<void *>(new T(std::move_if_noexcept(result))));
    }

    template<typename T>
    int addResults(int index, const QList<T> *results)
    {
        if (results->empty()) // reject if results are empty
            return -1;

        if (containsValidResultItem(index)) // reject if already present
            return -1;

        return addResults(index, new QList<T>(*results), results->size(), results->size());
    }

    template<typename T>
    int addResults(int index, const QList<T> *results, int totalCount)
    {
        // reject if results are empty, and nothing is filtered away
        if ((m_filterMode == false || results->size() == totalCount) && results->empty())
            return -1;

        if (containsValidResultItem(index)) // reject if already present
            return -1;

        if (m_filterMode == true && results->size() != totalCount && 0 == results->size())
            return addResults(index, nullptr, 0, totalCount);

        return addResults(index, new QList<T>(*results), results->size(), totalCount);
    }

    int addCanceledResult(int index)
    {
        if (containsValidResultItem(index)) // reject if already present
            return -1;

        return addResult(index, static_cast<void *>(nullptr));
    }

    template <typename T>
    int addCanceledResults(int index, int _count)
    {
        if (containsValidResultItem(index)) // reject if already present
            return -1;

        QList<T> empty;
        return addResults(index, &empty, _count);
    }

    template <typename T>
    void clear()
    {
        ResultStoreBase::clear<T>(m_results);
        resultCount = 0;
        insertIndex = 0;
        ResultStoreBase::clear<T>(pendingResults);
        filteredResults = 0;
    }
};

} // namespace QtPrivate

Q_DECLARE_TYPEINFO(QtPrivate::ResultItem, Q_PRIMITIVE_TYPE);


QT_END_NAMESPACE

#endif
