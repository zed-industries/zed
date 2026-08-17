/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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

#ifndef QFUTURE_H
#define QFUTURE_H

#include <QtCore/qglobal.h>

#include <QtCore/qfutureinterface.h>
#include <QtCore/qstring.h>

QT_REQUIRE_CONFIG(future);

QT_BEGIN_NAMESPACE


template <typename T>
class QFutureWatcher;
template <>
class QFutureWatcher<void>;

template <typename T>
class QFuture
{
public:
    QFuture()
        : d(QFutureInterface<T>::canceledResult())
    { }
    explicit QFuture(QFutureInterface<T> *p) // internal
        : d(*p)
    { }
#if defined(Q_CLANG_QDOC)
    ~QFuture() { }
    QFuture(const QFuture<T> &) { }
    QFuture<T> & operator=(const QFuture<T> &) { }
#endif

    bool operator==(const QFuture &other) const { return (d == other.d); }
    bool operator!=(const QFuture &other) const { return (d != other.d); }

    void cancel() { d.cancel(); }
    bool isCanceled() const { return d.isCanceled(); }

    void setPaused(bool paused) { d.setPaused(paused); }
    bool isPaused() const { return d.isPaused(); }
    void pause() { setPaused(true); }
    void resume() { setPaused(false); }
    void togglePaused() { d.togglePaused(); }

    bool isStarted() const { return d.isStarted(); }
    bool isFinished() const { return d.isFinished(); }
    bool isRunning() const { return d.isRunning(); }

    int resultCount() const { return d.resultCount(); }
    int progressValue() const { return d.progressValue(); }
    int progressMinimum() const { return d.progressMinimum(); }
    int progressMaximum() const { return d.progressMaximum(); }
    QString progressText() const { return d.progressText(); }
    void waitForFinished() { d.waitForFinished(); }

    inline T result() const;
    inline T resultAt(int index) const;
    bool isResultReadyAt(int resultIndex) const { return d.isResultReadyAt(resultIndex); }

    operator T() const { return result(); }
    QList<T> results() const { return d.results(); }

    class const_iterator
    {
    public:
        typedef std::bidirectional_iterator_tag iterator_category;
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef const T *pointer;
        typedef const T &reference;

        inline const_iterator() {}
        inline const_iterator(QFuture const * const _future, int _index)
        : future(_future), index(advanceIndex(_index, 0)) { }
        inline const_iterator(const const_iterator &o) : future(o.future), index(o.index)  {}
        inline const_iterator &operator=(const const_iterator &o)
        { future = o.future; index = o.index; return *this; }
        inline const T &operator*() const { return future->d.resultReference(index); }
        inline const T *operator->() const { return future->d.resultPointer(index); }
        inline bool operator!=(const const_iterator &other) const { return index != other.index; }
        inline bool operator==(const const_iterator &o) const { return !operator!=(o); }
        inline const_iterator &operator++()
        { index = advanceIndex(index, 1); return *this; }
        inline const_iterator &operator--()
        { index = advanceIndex(index, -1); return *this; }
        inline const_iterator operator++(int)
        {
            const_iterator r = *this;
            index = advanceIndex(index, 1);
            return r;
        }
        inline const_iterator operator--(int)
        {
            const_iterator r = *this;
            index = advanceIndex(index, -1);
            return r;
        }
        inline const_iterator operator+(int j) const
        { return const_iterator(future, advanceIndex(index, j)); }
        inline const_iterator operator-(int j) const
        { return const_iterator(future, advanceIndex(index, -j)); }
        inline const_iterator &operator+=(int j)
        { index = advanceIndex(index, j); return *this; }
        inline const_iterator &operator-=(int j)
        { index = advanceIndex(index, -j); return *this; }
        friend inline const_iterator operator+(int j, const_iterator k)
        { return const_iterator(k.future, k.advanceIndex(k.index, j)); }

    private:
        /*! \internal

            Advances the iterator index \a idx \a n steps, waits for the
            result at the target index, and returns the target index.

            The index may be -1, indicating the end iterator, either
            as the argument or as the return value. The end iterator
            may be decremented.

            The caller is responsible for not advancing the iterator
            before begin() or past end(), with the exception that
            attempting to advance a non-end iterator past end() for
            a running future is allowed and will return the end iterator.

            Note that n == 0 is valid and will wait for the result
            at the given index.
        */
        int advanceIndex(int idx, int n) const
        {
            // The end iterator can be decremented, leave as-is for other cases
            if (idx == -1 && n >= 0)
                return idx;

            // Special case for decrementing the end iterator: wait for
            // finished to get the total result count.
            if (idx == -1 && future->isRunning())
                future->d.waitForFinished();

            // Wait for result at target index
            const int targetIndex = (idx == -1) ? future->resultCount() + n : idx + n;
            future->d.waitForResult(targetIndex);

            // After waiting there is either a result or the end was reached
            return (targetIndex < future->resultCount()) ? targetIndex : -1;
        }

        QFuture const * future;
        int index;
    };
    friend class const_iterator;
    typedef const_iterator ConstIterator;

    const_iterator begin() const { return  const_iterator(this, 0); }
    const_iterator constBegin() const { return  const_iterator(this, 0); }
    const_iterator end() const { return const_iterator(this, -1); }
    const_iterator constEnd() const { return const_iterator(this, -1); }

private:
    friend class QFutureWatcher<T>;

public: // Warning: the d pointer is not documented and is considered private.
    mutable QFutureInterface<T> d;
};

template <typename T>
inline T QFuture<T>::result() const
{
    d.waitForResult(0);
    return d.resultReference(0);
}

template <typename T>
inline T QFuture<T>::resultAt(int index) const
{
    d.waitForResult(index);
    return d.resultReference(index);
}

template <typename T>
inline QFuture<T> QFutureInterface<T>::future()
{
    return QFuture<T>(this);
}

Q_DECLARE_SEQUENTIAL_ITERATOR(Future)

template <>
class QFuture<void>
{
public:
    QFuture()
        : d(QFutureInterface<void>::canceledResult())
    { }
    explicit QFuture(QFutureInterfaceBase *p) // internal
        : d(*p)
    { }

    bool operator==(const QFuture &other) const { return (d == other.d); }
    bool operator!=(const QFuture &other) const { return (d != other.d); }

#if !defined(Q_CC_XLC)
    template <typename T>
    QFuture(const QFuture<T> &other)
        : d(other.d)
    { }

    template <typename T>
    QFuture<void> &operator=(const QFuture<T> &other)
    {
        d = other.d;
        return *this;
    }
#endif

    void cancel() { d.cancel(); }
    bool isCanceled() const { return d.isCanceled(); }

    void setPaused(bool paused) { d.setPaused(paused); }
    bool isPaused() const { return d.isPaused(); }
    void pause() { setPaused(true); }
    void resume() { setPaused(false); }
    void togglePaused() { d.togglePaused(); }

    bool isStarted() const { return d.isStarted(); }
    bool isFinished() const { return d.isFinished(); }
    bool isRunning() const { return d.isRunning(); }

    int resultCount() const { return d.resultCount(); }
    int progressValue() const { return d.progressValue(); }
    int progressMinimum() const { return d.progressMinimum(); }
    int progressMaximum() const { return d.progressMaximum(); }
    QString progressText() const { return d.progressText(); }
    void waitForFinished() { d.waitForFinished(); }

private:
    friend class QFutureWatcher<void>;

#ifdef QFUTURE_TEST
public:
#endif
    mutable QFutureInterfaceBase d;
};

inline QFuture<void> QFutureInterface<void>::future()
{
    return QFuture<void>(this);
}

template <typename T>
QFuture<void> qToVoidFuture(const QFuture<T> &future)
{
    return QFuture<void>(future.d);
}

QT_END_NAMESPACE

#endif // QFUTURE_H
