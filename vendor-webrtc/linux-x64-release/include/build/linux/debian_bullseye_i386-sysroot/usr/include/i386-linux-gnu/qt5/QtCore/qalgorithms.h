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

#ifndef QALGORITHMS_H
#define QALGORITHMS_H

#include <QtCore/qglobal.h>

#ifdef Q_CC_MSVC
#include <intrin.h>
#endif

QT_BEGIN_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED

/*
    Warning: The contents of QAlgorithmsPrivate is not a part of the public Qt API
    and may be changed from version to version or even be completely removed.
*/
namespace QAlgorithmsPrivate {

#if QT_DEPRECATED_SINCE(5, 2)
template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::sort") Q_OUTOFLINE_TEMPLATE void qSortHelper(RandomAccessIterator start, RandomAccessIterator end, const T &t, LessThan lessThan);
template <typename RandomAccessIterator, typename T>
QT_DEPRECATED_X("Use std::sort") inline void qSortHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &dummy);

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::stable_sort") Q_OUTOFLINE_TEMPLATE void qStableSortHelper(RandomAccessIterator start, RandomAccessIterator end, const T &t, LessThan lessThan);
template <typename RandomAccessIterator, typename T>
QT_DEPRECATED_X("Use std::stable_sort") inline void qStableSortHelper(RandomAccessIterator, RandomAccessIterator, const T &);

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::lower_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qLowerBoundHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan);
template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::upper_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qUpperBoundHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan);
template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::binary_search") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qBinaryFindHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan);
#endif // QT_DEPRECATED_SINCE(5, 2)

}

#if QT_DEPRECATED_SINCE(5, 2)
template <typename InputIterator, typename OutputIterator>
QT_DEPRECATED_X("Use std::copy") inline OutputIterator qCopy(InputIterator begin, InputIterator end, OutputIterator dest)
{
    while (begin != end)
        *dest++ = *begin++;
    return dest;
}

template <typename BiIterator1, typename BiIterator2>
QT_DEPRECATED_X("Use std::copy_backward") inline BiIterator2 qCopyBackward(BiIterator1 begin, BiIterator1 end, BiIterator2 dest)
{
    while (begin != end)
        *--dest = *--end;
    return dest;
}

template <typename InputIterator1, typename InputIterator2>
QT_DEPRECATED_X("Use std::equal") inline bool qEqual(InputIterator1 first1, InputIterator1 last1, InputIterator2 first2)
{
    for (; first1 != last1; ++first1, ++first2)
        if (!(*first1 == *first2))
            return false;
    return true;
}

template <typename ForwardIterator, typename T>
QT_DEPRECATED_X("Use std::fill") inline void qFill(ForwardIterator first, ForwardIterator last, const T &val)
{
    for (; first != last; ++first)
        *first = val;
}

template <typename Container, typename T>
QT_DEPRECATED_X("Use std::fill") inline void qFill(Container &container, const T &val)
{
    qFill(container.begin(), container.end(), val);
}

template <typename InputIterator, typename T>
QT_DEPRECATED_X("Use std::find") inline InputIterator qFind(InputIterator first, InputIterator last, const T &val)
{
    while (first != last && !(*first == val))
        ++first;
    return first;
}

template <typename Container, typename T>
QT_DEPRECATED_X("Use std::find") inline typename Container::const_iterator qFind(const Container &container, const T &val)
{
    return qFind(container.constBegin(), container.constEnd(), val);
}

template <typename InputIterator, typename T, typename Size>
QT_DEPRECATED_X("Use std::count") inline void qCount(InputIterator first, InputIterator last, const T &value, Size &n)
{
    for (; first != last; ++first)
        if (*first == value)
            ++n;
}

template <typename Container, typename T, typename Size>
QT_DEPRECATED_X("Use std::count") inline void qCount(const Container &container, const T &value, Size &n)
{
    qCount(container.constBegin(), container.constEnd(), value, n);
}

#ifdef Q_QDOC
typedef void* LessThan;
template <typename T> LessThan qLess();
template <typename T> LessThan qGreater();
#else
template <typename T>
class QT_DEPRECATED_X("Use std::less") qLess
{
public:
    inline bool operator()(const T &t1, const T &t2) const
    {
        return (t1 < t2);
    }
};

template <typename T>
class QT_DEPRECATED_X("Use std::greater") qGreater
{
public:
    inline bool operator()(const T &t1, const T &t2) const
    {
        return (t2 < t1);
    }
};
#endif

template <typename RandomAccessIterator>
QT_DEPRECATED_X("Use std::sort") inline void qSort(RandomAccessIterator start, RandomAccessIterator end)
{
    if (start != end)
        QAlgorithmsPrivate::qSortHelper(start, end, *start);
}

template <typename RandomAccessIterator, typename LessThan>
QT_DEPRECATED_X("Use std::sort") inline void qSort(RandomAccessIterator start, RandomAccessIterator end, LessThan lessThan)
{
    if (start != end)
        QAlgorithmsPrivate::qSortHelper(start, end, *start, lessThan);
}

template<typename Container>
QT_DEPRECATED_X("Use std::sort") inline void qSort(Container &c)
{
#ifdef Q_CC_BOR
    // Work around Borland 5.5 optimizer bug
    c.detach();
#endif
    if (!c.empty())
        QAlgorithmsPrivate::qSortHelper(c.begin(), c.end(), *c.begin());
}

template <typename RandomAccessIterator>
QT_DEPRECATED_X("Use std::stable_sort") inline void qStableSort(RandomAccessIterator start, RandomAccessIterator end)
{
    if (start != end)
        QAlgorithmsPrivate::qStableSortHelper(start, end, *start);
}

template <typename RandomAccessIterator, typename LessThan>
QT_DEPRECATED_X("Use std::stable_sort") inline void qStableSort(RandomAccessIterator start, RandomAccessIterator end, LessThan lessThan)
{
    if (start != end)
        QAlgorithmsPrivate::qStableSortHelper(start, end, *start, lessThan);
}

template<typename Container>
QT_DEPRECATED_X("Use std::stable_sort") inline void qStableSort(Container &c)
{
#ifdef Q_CC_BOR
    // Work around Borland 5.5 optimizer bug
    c.detach();
#endif
    if (!c.empty())
        QAlgorithmsPrivate::qStableSortHelper(c.begin(), c.end(), *c.begin());
}

template <typename RandomAccessIterator, typename T>
QT_DEPRECATED_X("Use std::lower_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qLowerBound(RandomAccessIterator begin, RandomAccessIterator end, const T &value)
{
    // Implementation is duplicated from QAlgorithmsPrivate to keep existing code
    // compiling. We have to allow using *begin and value with different types,
    // and then implementing operator< for those types.
    RandomAccessIterator middle;
    int n = end - begin;
    int half;

    while (n > 0) {
        half = n >> 1;
        middle = begin + half;
        if (*middle < value) {
            begin = middle + 1;
            n -= half + 1;
        } else {
            n = half;
        }
    }
    return begin;
}

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::lower_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qLowerBound(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan)
{
    return QAlgorithmsPrivate::qLowerBoundHelper(begin, end, value, lessThan);
}

template <typename Container, typename T>
QT_DEPRECATED_X("Use std::lower_bound") Q_OUTOFLINE_TEMPLATE typename Container::const_iterator qLowerBound(const Container &container, const T &value)
{
    return QAlgorithmsPrivate::qLowerBoundHelper(container.constBegin(), container.constEnd(), value, qLess<T>());
}

template <typename RandomAccessIterator, typename T>
QT_DEPRECATED_X("Use std::upper_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qUpperBound(RandomAccessIterator begin, RandomAccessIterator end, const T &value)
{
    // Implementation is duplicated from QAlgorithmsPrivate.
    RandomAccessIterator middle;
    int n = end - begin;
    int half;

    while (n > 0) {
        half = n >> 1;
        middle = begin + half;
        if (value < *middle) {
            n = half;
        } else {
            begin = middle + 1;
            n -= half + 1;
        }
    }
    return begin;
}

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::upper_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qUpperBound(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan)
{
    return QAlgorithmsPrivate::qUpperBoundHelper(begin, end, value, lessThan);
}

template <typename Container, typename T>
QT_DEPRECATED_X("Use std::upper_bound") Q_OUTOFLINE_TEMPLATE typename Container::const_iterator qUpperBound(const Container &container, const T &value)
{
    return QAlgorithmsPrivate::qUpperBoundHelper(container.constBegin(), container.constEnd(), value, qLess<T>());
}

template <typename RandomAccessIterator, typename T>
QT_DEPRECATED_X("Use std::binary_search") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qBinaryFind(RandomAccessIterator begin, RandomAccessIterator end, const T &value)
{
    // Implementation is duplicated from QAlgorithmsPrivate.
    RandomAccessIterator it = qLowerBound(begin, end, value);

    if (it == end || value < *it)
        return end;

    return it;
}

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::binary_search") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qBinaryFind(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan)
{
    return QAlgorithmsPrivate::qBinaryFindHelper(begin, end, value, lessThan);
}

template <typename Container, typename T>
QT_DEPRECATED_X("Use std::binary_search") Q_OUTOFLINE_TEMPLATE typename Container::const_iterator qBinaryFind(const Container &container, const T &value)
{
    return QAlgorithmsPrivate::qBinaryFindHelper(container.constBegin(), container.constEnd(), value, qLess<T>());
}
#endif // QT_DEPRECATED_SINCE(5, 2)

template <typename ForwardIterator>
Q_OUTOFLINE_TEMPLATE void qDeleteAll(ForwardIterator begin, ForwardIterator end)
{
    while (begin != end) {
        delete *begin;
        ++begin;
    }
}

template <typename Container>
inline void qDeleteAll(const Container &c)
{
    qDeleteAll(c.begin(), c.end());
}

/*
    Warning: The contents of QAlgorithmsPrivate is not a part of the public Qt API
    and may be changed from version to version or even be completely removed.
*/
namespace QAlgorithmsPrivate {

#if QT_DEPRECATED_SINCE(5, 2)

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::sort") Q_OUTOFLINE_TEMPLATE void qSortHelper(RandomAccessIterator start, RandomAccessIterator end, const T &t, LessThan lessThan)
{
top:
    int span = int(end - start);
    if (span < 2)
        return;

    --end;
    RandomAccessIterator low = start, high = end - 1;
    RandomAccessIterator pivot = start + span / 2;

    if (lessThan(*end, *start))
        qSwap(*end, *start);
    if (span == 2)
        return;

    if (lessThan(*pivot, *start))
        qSwap(*pivot, *start);
    if (lessThan(*end, *pivot))
        qSwap(*end, *pivot);
    if (span == 3)
        return;

    qSwap(*pivot, *end);

    while (low < high) {
        while (low < high && lessThan(*low, *end))
            ++low;

        while (high > low && lessThan(*end, *high))
            --high;

        if (low < high) {
            qSwap(*low, *high);
            ++low;
            --high;
        } else {
            break;
        }
    }

    if (lessThan(*low, *end))
        ++low;

    qSwap(*end, *low);
    qSortHelper(start, low, t, lessThan);

    start = low + 1;
    ++end;
    goto top;
}

template <typename RandomAccessIterator, typename T>
QT_DEPRECATED_X("Use std::sort") inline void qSortHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &dummy)
{
    qSortHelper(begin, end, dummy, qLess<T>());
}

template <typename RandomAccessIterator>
QT_DEPRECATED_X("Use std::reverse") Q_OUTOFLINE_TEMPLATE void qReverse(RandomAccessIterator begin, RandomAccessIterator end)
{
    --end;
    while (begin < end)
        qSwap(*begin++, *end--);
}

template <typename RandomAccessIterator>
QT_DEPRECATED_X("Use std::rotate") Q_OUTOFLINE_TEMPLATE void qRotate(RandomAccessIterator begin, RandomAccessIterator middle, RandomAccessIterator end)
{
    qReverse(begin, middle);
    qReverse(middle, end);
    qReverse(begin, end);
}

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::merge") Q_OUTOFLINE_TEMPLATE void qMerge(RandomAccessIterator begin, RandomAccessIterator pivot, RandomAccessIterator end, T &t, LessThan lessThan)
{
    const int len1 = pivot - begin;
    const int len2 = end - pivot;

    if (len1 == 0 || len2 == 0)
        return;

    if (len1 + len2 == 2) {
        if (lessThan(*(begin + 1), *(begin)))
            qSwap(*begin, *(begin + 1));
        return;
    }

    RandomAccessIterator firstCut;
    RandomAccessIterator secondCut;
    int len2Half;
    if (len1 > len2) {
        const int len1Half = len1 / 2;
        firstCut = begin + len1Half;
        secondCut = qLowerBound(pivot, end, *firstCut, lessThan);
        len2Half = secondCut - pivot;
    } else {
        len2Half = len2 / 2;
        secondCut = pivot + len2Half;
        firstCut = qUpperBound(begin, pivot, *secondCut, lessThan);
    }

    qRotate(firstCut, pivot, secondCut);
    const RandomAccessIterator newPivot = firstCut + len2Half;
    qMerge(begin, firstCut, newPivot, t, lessThan);
    qMerge(newPivot, secondCut, end, t, lessThan);
}

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::stable_sort") Q_OUTOFLINE_TEMPLATE void qStableSortHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &t, LessThan lessThan)
{
    const int span = end - begin;
    if (span < 2)
       return;

    const RandomAccessIterator middle = begin + span / 2;
    qStableSortHelper(begin, middle, t, lessThan);
    qStableSortHelper(middle, end, t, lessThan);
    qMerge(begin, middle, end, t, lessThan);
}

template <typename RandomAccessIterator, typename T>
QT_DEPRECATED_X("Use std::stable_sort") inline void qStableSortHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &dummy)
{
    qStableSortHelper(begin, end, dummy, qLess<T>());
}

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::lower_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qLowerBoundHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan)
{
    RandomAccessIterator middle;
    int n = int(end - begin);
    int half;

    while (n > 0) {
        half = n >> 1;
        middle = begin + half;
        if (lessThan(*middle, value)) {
            begin = middle + 1;
            n -= half + 1;
        } else {
            n = half;
        }
    }
    return begin;
}


template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::upper_bound") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qUpperBoundHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan)
{
    RandomAccessIterator middle;
    int n = end - begin;
    int half;

    while (n > 0) {
        half = n >> 1;
        middle = begin + half;
        if (lessThan(value, *middle)) {
            n = half;
        } else {
            begin = middle + 1;
            n -= half + 1;
        }
    }
    return begin;
}

template <typename RandomAccessIterator, typename T, typename LessThan>
QT_DEPRECATED_X("Use std::binary_search") Q_OUTOFLINE_TEMPLATE RandomAccessIterator qBinaryFindHelper(RandomAccessIterator begin, RandomAccessIterator end, const T &value, LessThan lessThan)
{
    RandomAccessIterator it = qLowerBoundHelper(begin, end, value, lessThan);

    if (it == end || lessThan(value, *it))
        return end;

    return it;
}

#endif // QT_DEPRECATED_SINCE(5, 2)

#ifdef Q_CC_CLANG
// Clang had a bug where __builtin_ctz/clz/popcount were not marked as constexpr.
#  if (defined __apple_build_version__ &&  __clang_major__ >= 7) || (Q_CC_CLANG >= 307)
#    define QT_HAS_CONSTEXPR_BUILTINS
#  endif
#elif defined(Q_CC_MSVC) && !defined(Q_CC_INTEL) && !defined(Q_PROCESSOR_ARM)
#  define QT_HAS_CONSTEXPR_BUILTINS
#elif defined(Q_CC_GNU)
#  define QT_HAS_CONSTEXPR_BUILTINS
#endif

#if defined QT_HAS_CONSTEXPR_BUILTINS
#if defined(Q_CC_GNU) || defined(Q_CC_CLANG)
#  define QT_HAS_BUILTIN_CTZS
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_ctzs(quint16 v) noexcept
{
#  if __has_builtin(__builtin_ctzs)
    return __builtin_ctzs(v);
#  else
    return __builtin_ctz(v);
#  endif
}
#define QT_HAS_BUILTIN_CLZS
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_clzs(quint16 v) noexcept
{
#  if __has_builtin(__builtin_clzs)
    return __builtin_clzs(v);
#  else
    return __builtin_clz(v) - 16U;
#  endif
}
#define QT_HAS_BUILTIN_CTZ
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_ctz(quint32 v) noexcept
{
    return __builtin_ctz(v);
}
#define QT_HAS_BUILTIN_CLZ
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_clz(quint32 v) noexcept
{
    return __builtin_clz(v);
}
#define QT_HAS_BUILTIN_CTZLL
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_ctzll(quint64 v) noexcept
{
    return __builtin_ctzll(v);
}
#define QT_HAS_BUILTIN_CLZLL
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_clzll(quint64 v) noexcept
{
    return __builtin_clzll(v);
}
#define QALGORITHMS_USE_BUILTIN_POPCOUNT
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_popcount(quint32 v) noexcept
{
    return __builtin_popcount(v);
}
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_popcount(quint8 v) noexcept
{
    return __builtin_popcount(v);
}
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_popcount(quint16 v) noexcept
{
    return __builtin_popcount(v);
}
#define QALGORITHMS_USE_BUILTIN_POPCOUNTLL
Q_DECL_CONSTEXPR Q_ALWAYS_INLINE uint qt_builtin_popcountll(quint64 v) noexcept
{
    return __builtin_popcountll(v);
}
#elif defined(Q_CC_MSVC) && !defined(Q_PROCESSOR_ARM)
#define QT_POPCOUNT_CONSTEXPR
#define QT_POPCOUNT_RELAXED_CONSTEXPR
#define QT_HAS_BUILTIN_CTZ
Q_ALWAYS_INLINE unsigned long qt_builtin_ctz(quint32 val)
{
    unsigned long result;
    _BitScanForward(&result, val);
    return result;
}
#define QT_HAS_BUILTIN_CLZ
Q_ALWAYS_INLINE unsigned long qt_builtin_clz(quint32 val)
{
    unsigned long result;
    _BitScanReverse(&result, val);
    // Now Invert the result: clz will count *down* from the msb to the lsb, so the msb index is 31
    // and the lsb index is 0. The result for the index when counting up: msb index is 0 (because it
    // starts there), and the lsb index is 31.
    result ^= sizeof(quint32) * 8 - 1;
    return result;
}
#if Q_PROCESSOR_WORDSIZE == 8
// These are only defined for 64bit builds.
#define QT_HAS_BUILTIN_CTZLL
Q_ALWAYS_INLINE unsigned long qt_builtin_ctzll(quint64 val)
{
    unsigned long result;
    _BitScanForward64(&result, val);
    return result;
}
// MSVC calls it _BitScanReverse and returns the carry flag, which we don't need
#define QT_HAS_BUILTIN_CLZLL
Q_ALWAYS_INLINE unsigned long qt_builtin_clzll(quint64 val)
{
    unsigned long result;
    _BitScanReverse64(&result, val);
    // see qt_builtin_clz
    result ^= sizeof(quint64) * 8 - 1;
    return result;
}
#endif // MSVC 64bit
#  define QT_HAS_BUILTIN_CTZS
Q_ALWAYS_INLINE uint qt_builtin_ctzs(quint16 v) noexcept
{
    return qt_builtin_ctz(v);
}
#define QT_HAS_BUILTIN_CLZS
Q_ALWAYS_INLINE uint qt_builtin_clzs(quint16 v) noexcept
{
    return qt_builtin_clz(v) - 16U;
}

// Neither MSVC nor the Intel compiler define a macro for the POPCNT processor
// feature, so we're using either the SSE4.2 or the AVX macro as a proxy (Clang
// does define the macro). It's incorrect for two reasons:
// 1. It's a separate bit in CPUID, so a processor could implement SSE4.2 and
//    not POPCNT, but that's unlikely to happen.
// 2. There are processors that support POPCNT but not AVX (Intel Nehalem
//    architecture), but unlike the other compilers, MSVC has no option
//    to generate code for those processors.
// So it's an acceptable compromise.
#if defined(__AVX__) || defined(__SSE4_2__) || defined(__POPCNT__)
#define QALGORITHMS_USE_BUILTIN_POPCOUNT
#define QALGORITHMS_USE_BUILTIN_POPCOUNTLL
Q_ALWAYS_INLINE uint qt_builtin_popcount(quint32 v) noexcept
{
    return __popcnt(v);
}
Q_ALWAYS_INLINE uint qt_builtin_popcount(quint8 v) noexcept
{
    return __popcnt16(v);
}
Q_ALWAYS_INLINE uint qt_builtin_popcount(quint16 v) noexcept
{
    return __popcnt16(v);
}
Q_ALWAYS_INLINE uint qt_builtin_popcountll(quint64 v) noexcept
{
#if Q_PROCESSOR_WORDSIZE == 8
    return __popcnt64(v);
#else
    return __popcnt(quint32(v)) + __popcnt(quint32(v >> 32));
#endif // MSVC 64bit
}

#endif // __AVX__ || __SSE4_2__ || __POPCNT__

#endif // MSVC
#endif // QT_HAS_CONSTEXPR_BUILTINS

#ifndef QT_POPCOUNT_CONSTEXPR
#define QT_POPCOUNT_CONSTEXPR Q_DECL_CONSTEXPR
#define QT_POPCOUNT_RELAXED_CONSTEXPR Q_DECL_RELAXED_CONSTEXPR
#endif

} //namespace QAlgorithmsPrivate

Q_DECL_CONST_FUNCTION QT_POPCOUNT_CONSTEXPR inline uint qPopulationCount(quint32 v) noexcept
{
#ifdef QALGORITHMS_USE_BUILTIN_POPCOUNT
    return QAlgorithmsPrivate::qt_builtin_popcount(v);
#else
    // See http://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetParallel
    return
        (((v      ) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 12) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 24) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f;
#endif
}

Q_DECL_CONST_FUNCTION QT_POPCOUNT_CONSTEXPR inline uint qPopulationCount(quint8 v) noexcept
{
#ifdef QALGORITHMS_USE_BUILTIN_POPCOUNT
    return QAlgorithmsPrivate::qt_builtin_popcount(v);
#else
    return
        (((v      ) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f;
#endif
}

Q_DECL_CONST_FUNCTION QT_POPCOUNT_CONSTEXPR inline uint qPopulationCount(quint16 v) noexcept
{
#ifdef QALGORITHMS_USE_BUILTIN_POPCOUNT
    return QAlgorithmsPrivate::qt_builtin_popcount(v);
#else
    return
        (((v      ) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 12) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f;
#endif
}

Q_DECL_CONST_FUNCTION QT_POPCOUNT_CONSTEXPR inline uint qPopulationCount(quint64 v) noexcept
{
#ifdef QALGORITHMS_USE_BUILTIN_POPCOUNTLL
    return QAlgorithmsPrivate::qt_builtin_popcountll(v);
#else
    return
        (((v      ) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 12) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 24) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 36) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 48) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f +
        (((v >> 60) & 0xfff)    * Q_UINT64_C(0x1001001001001) & Q_UINT64_C(0x84210842108421)) % 0x1f;
#endif
}

Q_DECL_CONST_FUNCTION QT_POPCOUNT_CONSTEXPR inline uint qPopulationCount(long unsigned int v) noexcept
{
    return qPopulationCount(static_cast<quint64>(v));
}

#if defined(Q_CC_GNU) && !defined(Q_CC_CLANG)
#undef QALGORITHMS_USE_BUILTIN_POPCOUNT
#endif
#undef QT_POPCOUNT_CONSTEXPR

Q_DECL_RELAXED_CONSTEXPR inline uint qCountTrailingZeroBits(quint32 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CTZ)
    return v ? QAlgorithmsPrivate::qt_builtin_ctz(v) : 32U;
#else
    // see http://graphics.stanford.edu/~seander/bithacks.html#ZerosOnRightParallel
    unsigned int c = 32; // c will be the number of zero bits on the right
    v &= -signed(v);
    if (v) c--;
    if (v & 0x0000FFFF) c -= 16;
    if (v & 0x00FF00FF) c -= 8;
    if (v & 0x0F0F0F0F) c -= 4;
    if (v & 0x33333333) c -= 2;
    if (v & 0x55555555) c -= 1;
    return c;
#endif
}

Q_DECL_RELAXED_CONSTEXPR inline uint qCountTrailingZeroBits(quint8 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CTZ)
    return v ? QAlgorithmsPrivate::qt_builtin_ctz(v) : 8U;
#else
    unsigned int c = 8; // c will be the number of zero bits on the right
    v &= -signed(v);
    if (v) c--;
    if (v & 0x0000000F) c -= 4;
    if (v & 0x00000033) c -= 2;
    if (v & 0x00000055) c -= 1;
    return c;
#endif
}

Q_DECL_RELAXED_CONSTEXPR inline uint qCountTrailingZeroBits(quint16 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CTZS)
    return v ? QAlgorithmsPrivate::qt_builtin_ctzs(v) : 16U;
#else
    unsigned int c = 16; // c will be the number of zero bits on the right
    v &= -signed(v);
    if (v) c--;
    if (v & 0x000000FF) c -= 8;
    if (v & 0x00000F0F) c -= 4;
    if (v & 0x00003333) c -= 2;
    if (v & 0x00005555) c -= 1;
    return c;
#endif
}

Q_DECL_RELAXED_CONSTEXPR inline uint qCountTrailingZeroBits(quint64 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CTZLL)
    return v ? QAlgorithmsPrivate::qt_builtin_ctzll(v) : 64;
#else
    quint32 x = static_cast<quint32>(v);
    return x ? qCountTrailingZeroBits(x)
             : 32 + qCountTrailingZeroBits(static_cast<quint32>(v >> 32));
#endif
}

Q_DECL_RELAXED_CONSTEXPR inline uint qCountTrailingZeroBits(unsigned long v) noexcept
{
    return qCountTrailingZeroBits(QIntegerForSizeof<long>::Unsigned(v));
}

Q_DECL_RELAXED_CONSTEXPR inline uint qCountLeadingZeroBits(quint32 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CLZ)
    return v ? QAlgorithmsPrivate::qt_builtin_clz(v) : 32U;
#else
    // Hacker's Delight, 2nd ed. Fig 5-16, p. 102
    v = v | (v >> 1);
    v = v | (v >> 2);
    v = v | (v >> 4);
    v = v | (v >> 8);
    v = v | (v >> 16);
    return qPopulationCount(~v);
#endif
}

QT_POPCOUNT_RELAXED_CONSTEXPR inline uint qCountLeadingZeroBits(quint8 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CLZ)
    return v ? QAlgorithmsPrivate::qt_builtin_clz(v)-24U : 8U;
#else
    v = v | (v >> 1);
    v = v | (v >> 2);
    v = v | (v >> 4);
    return qPopulationCount(static_cast<quint8>(~v));
#endif
}

QT_POPCOUNT_RELAXED_CONSTEXPR inline uint qCountLeadingZeroBits(quint16 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CLZS)
    return v ? QAlgorithmsPrivate::qt_builtin_clzs(v) : 16U;
#else
    v = v | (v >> 1);
    v = v | (v >> 2);
    v = v | (v >> 4);
    v = v | (v >> 8);
    return qPopulationCount(static_cast<quint16>(~v));
#endif
}

QT_POPCOUNT_RELAXED_CONSTEXPR inline uint qCountLeadingZeroBits(quint64 v) noexcept
{
#if defined(QT_HAS_BUILTIN_CLZLL)
    return v ? QAlgorithmsPrivate::qt_builtin_clzll(v) : 64U;
#else
    v = v | (v >> 1);
    v = v | (v >> 2);
    v = v | (v >> 4);
    v = v | (v >> 8);
    v = v | (v >> 16);
    v = v | (v >> 32);
    return qPopulationCount(~v);
#endif
}

QT_POPCOUNT_RELAXED_CONSTEXPR inline uint qCountLeadingZeroBits(unsigned long v) noexcept
{
    return qCountLeadingZeroBits(QIntegerForSizeof<long>::Unsigned(v));
}

QT_WARNING_POP
QT_END_NAMESPACE

#endif // QALGORITHMS_H
