// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef Q23FUNCTIONAL_H
#define Q23FUNCTIONAL_H

#include <QtCore/qglobal.h>
#include <QtCore/q20functional.h>

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

namespace q23 {
// like std::invoke_r
#ifdef __cpp_lib_invoke_r
using std::invoke_r;
#else
template <typename R, typename F, typename...Args>
constexpr
std::enable_if_t<std::is_invocable_r_v<R, F, Args...>, R>
invoke_r(F&& f, Args&&... args)
    noexcept(std::is_nothrow_invocable_r_v<R, F, Args...>)
{
    // ### use q20::invoke for a constexpr std::invoke
    if constexpr (std::is_void_v<R>)
        std::invoke(std::forward<F>(f), std::forward<Args>(args)...);
    else
        return std::invoke(std::forward<F>(f), std::forward<Args>(args)...);
}
#endif // __cpp_lib_invoke_r
} // namespace q23

QT_END_NAMESPACE

#endif /* Q23FUNCTIONAL_H */
