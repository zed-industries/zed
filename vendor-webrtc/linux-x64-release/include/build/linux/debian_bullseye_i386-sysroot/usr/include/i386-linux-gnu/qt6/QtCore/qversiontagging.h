// Copyright (C) 2022 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

// qglobal.h includes this header, so keep it outside of our include guards
#include <QtCore/qglobal.h>

#if !defined(QVERSIONTAGGING_H)
#define QVERSIONTAGGING_H

QT_BEGIN_NAMESPACE

/*
 * Explanation
 *
 * This file causes all libraries, plugins, and applications that #include this
 * file to automatically pull in a symbol found in QtCore that encodes the
 * current Qt version number at the time of compilation. The relocation is
 * designed so that it's impossible for the dynamic linker to perform lazy
 * binding. Instead, it must resolve at load time or fail. That way, attempting
 * to load such a library or plugin while an older QtCore is loaded will fail.
 * Similarly, if an older QtCore is found when launching an application, the
 * application will fail to launch.
 *
 * It's also possible to inspect which version is required by decoding the
 * .qtversion section. The second pointer-sized variable is the required
 * version, for example, for Qt 6.4.1:
 *
 *      Hex dump of section [18] '.qtversion', 16 bytes at offset 0x1ee48:
 *        0x00000000 b0ffffff ffffffff 01040600 00000000 ................
 *                                     ^^^^^^^^ ^^^^^^^^
 *
 * There will only be one copy of the section in the output library or application.
 *
 * This functionality can be disabled by defining QT_NO_VERSION_TAGGING. It's
 * disabled if Qt was built statically.
 *
 * Windows notes:
 *
 *  On Windows, the address of a __declspec(dllimport) variable is not a
 *  constant expression, unlike Unix systems. So we instead use the address of
 *  the import variable, which is created by prefixing the external name with
 *  "__imp_". Using that variable causes an import of the corresponding symbol
 *  from QtCore DLL.
 *
 *  With MinGW (GCC and Clang), we use a C++17 inline variable, so the compiler
 *  and linker automatically merge the variables. The "used" __attribute__
 *  tells the compiler to always emit that variable, whether it's used or not.
 *
 *  MSVC has no equivalent to that attribute, so instead we create an extern
 *  const variable and tell the linker to merge them all via
 *  __declspec(selectany).
 *
 * Unix notes:
 *
 *  On Unix, we use the same C++17 inline variable solution as MinGW, but we
 *  don't need the "__imp_" trick.
 *
 *  Additionally, on ELF systems like Linux and FreeBSD, the symbol in question
 *  is simply "qt_version_tag" in both QtCore and in this ELF module, but it
 *  has an ELF version attached to it (see qversiontagging.cpp and
 *  QtFlagHandlingHelpers.cmake). That way, the error message from the dynamic
 *  linker will say it can't find version "Qt_6.x".
 */

namespace QtPrivate {
struct QVersionTag
{
    const void *symbol;
    quintptr version;
    constexpr QVersionTag(const void *sym, int currentVersion = QT_VERSION)
        : symbol(sym), version(currentVersion)
    {}
};
}

#if !defined(QT_NO_VERSION_TAGGING) && (defined(QT_BUILD_CORE_LIB) || defined(QT_BOOTSTRAPPED) || defined(QT_STATIC))
// don't make tags in QtCore, bootstrapped systems or if the user asked not to
#  define QT_NO_VERSION_TAGGING
#endif

#if defined(Q_OS_WIN)
#  ifdef Q_PROCESSOR_X86_32
//   32-bit x86 convention does prepend a _
#    define QT_MANGLE_IMPORT_PREFIX     _imp__
#  else
//   Calling convention on other architectures does not prepend a _
#    define QT_MANGLE_IMPORT_PREFIX     __imp_
#  endif
#  ifdef Q_CC_MSVC
#    pragma section(".qtversion",read,shared)
#    define QT_VERSION_TAG_SECTION      __declspec(allocate(".qtversion"))
#    define QT_VERSION_TAG_ATTRIBUTE    __declspec(selectany) extern const
#  else
#    define QT_VERSION_TAG_ATTRIBUTE    __attribute__((used)) constexpr inline
#  endif
#  define QT_VERSION_TAG2(sym, imp)     \
    extern "C" const char * const imp;  \
    QT_VERSION_TAG_ATTRIBUTE QT_VERSION_TAG_SECTION QtPrivate::QVersionTag sym ## _used(&imp)
#  define QT_VERSION_TAG(sym, imp)       QT_VERSION_TAG2(sym, imp)
#elif defined(Q_CC_GNU) && __has_attribute(used)
#  ifdef Q_OS_DARWIN
#    define QT_VERSION_TAG_SECTION      __attribute__((section("__DATA,.qtversion")))
#  endif
#  define QT_VERSION_TAG_ATTRIBUTE    __attribute__((visibility("hidden"), used))
#  define QT_VERSION_TAG2(sym, imp)     \
    extern "C" Q_DECL_IMPORT const char sym; \
    QT_VERSION_TAG_ATTRIBUTE QT_VERSION_TAG_SECTION constexpr inline QtPrivate::QVersionTag sym ## _use(&sym)
#  define QT_VERSION_TAG(sym, imp)       QT_VERSION_TAG2(sym, imp)
#endif

#ifdef Q_OF_ELF
#  define QT_VERSION_TAG_SYMBOL(prefix, sym, m, n)      sym
#else
#  define QT_VERSION_TAG_SYMBOL2(prefix, sym, m, n)     prefix ## sym ## _ ## m ## _ ## n
#  define QT_VERSION_TAG_SYMBOL(prefix, sym, m, n)      QT_VERSION_TAG_SYMBOL2(prefix, sym, m, n)
#endif

#if defined(QT_VERSION_TAG) && !defined(QT_NO_VERSION_TAGGING)
#  ifndef QT_VERSION_TAG_SECTION
#    define QT_VERSION_TAG_SECTION          __attribute__((section(".qtversion")))
#  endif
#  define QT_MANGLED_VERSION_TAG_IMPORT     QT_VERSION_TAG_SYMBOL(QT_MANGLE_IMPORT_PREFIX, QT_MANGLE_NAMESPACE(qt_version_tag), QT_VERSION_MAJOR, QT_VERSION_MINOR)
#  define QT_MANGLED_VERSION_TAG            QT_VERSION_TAG_SYMBOL(, QT_MANGLE_NAMESPACE(qt_version_tag), QT_VERSION_MAJOR, QT_VERSION_MINOR)

QT_VERSION_TAG(QT_MANGLED_VERSION_TAG, QT_MANGLED_VERSION_TAG_IMPORT);

#  undef QT_MANGLED_VERSION_TAG
#  undef QT_MANGLED_VERSION_TAG_IMPORT
#  undef QT_VERSION_TAG_SECTION
#endif

QT_END_NAMESPACE

#endif // QVERSIONTAGGING_H
