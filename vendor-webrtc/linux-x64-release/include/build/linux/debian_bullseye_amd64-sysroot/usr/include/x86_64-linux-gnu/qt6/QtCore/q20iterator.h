// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef Q20ITERATOR_H
#define Q20ITERATOR_H

#include <QtCore/qglobal.h>

#include <iterator>

//
//  W A R N I N G
//  -------------
//
// This file is not part of the Qt API. Types and functions defined in this
// file can reliably be replaced by their std counterparts, once available.
// You may use these definitions in your own code, but be aware that we
// will remove them once Qt depends on the C++ version that supports
// them in namespace std. There will be NO deprecation warning, the
// definitions will JUST go away.
//
// If you can't agree to these terms, don't use these definitions!
//
// We mean it.
//

QT_BEGIN_NAMESPACE

// like std::ssize
namespace q20 {
#ifdef __cpp_lib_ssize
    using std::ssize;
#else
    template<class C> constexpr auto ssize(const C &c)
      -> std::common_type_t<std::ptrdiff_t, std::make_signed_t<decltype(c.size())>>
    { return static_cast<std::common_type_t<std::ptrdiff_t, std::make_signed_t<decltype(c.size())>>>(c.size()); }

    template<class T, std::ptrdiff_t N> constexpr std::ptrdiff_t ssize(const T (&)[N]) noexcept
    { return N; }
#endif
} // namespace q20

QT_END_NAMESPACE

#endif /* Q20ITERATOR_H */
